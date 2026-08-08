use getrandom::fill;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::permission::{
        CreateOrganizationInviteDto, PermissionGroupDto, PermissionPolicyDto,
        ReplaceUserPoliciesDto, SavePermissionGroupDto,
    },
    db::{
        models::audit_logs::AuditLog,
        models::organization_invites::OrganizationInvite,
        repository::{
            AuditLogRepository, NotifEmailRepository, NotifResendRepository,
            OrganizationInviteRepository, OrganizationMemberRepository, PermissionGroupRepository,
            UserRepository,
        },
    },
};

use super::{PermissionService, UserRole};
use crate::services::notification::senders::send_resend_to;
use crate::services::notification::{email::send_email_to, message::NotificationMessage};

#[derive(Debug, thiserror::Error)]
pub enum PermissionGroupError {
    #[error("{0}")]
    Invalid(String),
    #[error("permission delegation exceeds the actor's permissions: {0}")]
    Escalation(String),
    #[error("resource not found or protected")]
    NotFound,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

pub struct PermissionGroupService {
    repository: Arc<PermissionGroupRepository>,
    member_repository: Arc<OrganizationMemberRepository>,
    permission_service: Arc<PermissionService>,
    invite_repository: Arc<OrganizationInviteRepository>,
    user_repository: Arc<UserRepository>,
    audit_repository: Arc<AuditLogRepository>,
    email_repository: Arc<NotifEmailRepository>,
    resend_repository: Arc<NotifResendRepository>,
}

#[singleton]
impl PermissionGroupService {
    pub fn new(
        repository: Arc<PermissionGroupRepository>,
        member_repository: Arc<OrganizationMemberRepository>,
        permission_service: Arc<PermissionService>,
        invite_repository: Arc<OrganizationInviteRepository>,
        user_repository: Arc<UserRepository>,
        audit_repository: Arc<AuditLogRepository>,
        email_repository: Arc<NotifEmailRepository>,
        resend_repository: Arc<NotifResendRepository>,
    ) -> Self {
        Self {
            repository,
            member_repository,
            permission_service,
            invite_repository,
            user_repository,
            audit_repository,
            email_repository,
            resend_repository,
        }
    }

    async fn audit(
        &self,
        actor_id: i64,
        organization_id: i64,
        action: &str,
        resource: &str,
        id: Option<String>,
        metadata: Option<String>,
    ) {
        let user = self
            .user_repository
            .get_by_id(actor_id)
            .await
            .ok()
            .flatten();
        let item = AuditLog {
            id: None,
            user_email: user
                .as_ref()
                .and_then(|u| u.email.clone())
                .unwrap_or_default(),
            user_role: user
                .as_ref()
                .and_then(|u| u.role.clone())
                .unwrap_or_else(|| "MEMBER".into()),
            action: action.into(),
            resource_type: resource.into(),
            resource_id: id,
            resource_name: None,
            metadata,
            organization_id: Some(organization_id),
            user_id: Some(actor_id),
            created_at: chrono::Utc::now().timestamp(),
        };
        if let Err(error) = self.audit_repository.create(&item).await {
            tracing::warn!(%error, action, "could not write permission audit event");
        }
    }

    pub async fn list_invites(
        &self,
        organization_id: i64,
    ) -> Result<Vec<OrganizationInvite>, PermissionGroupError> {
        self.invite_repository.purge_expired().await?;
        Ok(self
            .invite_repository
            .list_pending_for_organization(organization_id)
            .await?)
    }

    pub async fn create_invite(
        &self,
        actor_id: i64,
        organization_id: i64,
        body: CreateOrganizationInviteDto,
    ) -> Result<String, PermissionGroupError> {
        let email = body.email.trim().to_ascii_lowercase();
        if !email.contains('@') {
            return Err(PermissionGroupError::Invalid("invalid email".into()));
        }
        let role = body.role.to_ascii_uppercase();
        if !matches!(role.as_str(), "ADMIN" | "MEMBER") {
            return Err(PermissionGroupError::Invalid(
                "role must be ADMIN or MEMBER".into(),
            ));
        }
        if role == "ADMIN" && !self.actor_is_privileged(actor_id, organization_id).await? {
            return Err(PermissionGroupError::Escalation(
                "only admins can invite another admin".into(),
            ));
        }
        let actions = self.repository.group_actions(body.group_id).await?;
        self.ensure_subset(actor_id, organization_id, &actions)
            .await?;
        let mut bytes = [0_u8; 32];
        fill(&mut bytes)
            .map_err(|_| PermissionGroupError::Invalid("could not generate invite token".into()))?;
        let token = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
        let token_hash = hash_token(&token);
        let item = OrganizationInvite {
            id: None,
            email,
            role: Some(role),
            status: Some("PENDING".into()),
            token: token_hash,
            group_id: body.group_id,
            organization_id,
            invited_by: actor_id,
            expired_at: chrono::Utc::now().timestamp() + 7 * 86400,
            created_at: chrono::Utc::now().timestamp(),
        };
        self.invite_repository.create(&item).await?;
        self.send_invite_email(&item.email, &token, organization_id, actor_id)
            .await;
        self.audit(
            actor_id,
            organization_id,
            "INVITE_CREATED",
            "organization_invite",
            None,
            Some(serde_json::json!({"email": item.email, "role": item.role}).to_string()),
        )
        .await;
        Ok(token)
    }

    async fn send_invite_email(
        &self,
        email: &str,
        token: &str,
        organization_id: i64,
        actor_id: i64,
    ) {
        let action = std::env::var("RUSTPLOY_PUBLIC_URL")
            .ok()
            .map(|base| format!("{}/accept-invite?token={token}", base.trim_end_matches('/')))
            .unwrap_or_else(|| format!("Invite token: {token}"));
        let message = NotificationMessage::new(
            "You have been invited to Rustploy",
            format!(
                "You have been invited to join organization {organization_id}.\n\nAccept this invitation within 7 days:\n{action}"
            ),
        );
        if let Ok(Some(config)) = self.email_repository.find_for_user(actor_id).await {
            if let Err(error) = send_email_to(&config, email, &message).await {
                tracing::warn!(%error, "organization invite email delivery failed");
            }
        } else if let Ok(Some(config)) = self.resend_repository.find_for_user(actor_id).await {
            if let Err(error) =
                send_resend_to(&reqwest::Client::new(), &config, email, &message).await
            {
                tracing::warn!(%error, "organization invite Resend delivery failed");
            }
        } else {
            tracing::warn!(
                organization_id,
                "organization invite created but no email provider configured"
            );
        }
    }

    pub async fn cancel_invite(
        &self,
        actor_id: i64,
        organization_id: i64,
        invite_id: i64,
    ) -> Result<(), PermissionGroupError> {
        let invite = self
            .invite_repository
            .get_by_id(invite_id)
            .await?
            .filter(|i| i.organization_id == organization_id)
            .ok_or(PermissionGroupError::NotFound)?;
        if invite.status.as_deref() != Some("PENDING")
            || !self
                .invite_repository
                .set_status(invite_id, "REJECTED")
                .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.audit(
            actor_id,
            organization_id,
            "INVITE_CANCELLED",
            "organization_invite",
            Some(invite_id.to_string()),
            None,
        )
        .await;
        Ok(())
    }

    pub async fn accept_invite(
        &self,
        user_id: i64,
        token: String,
    ) -> Result<(), PermissionGroupError> {
        let invite = self
            .invite_repository
            .find_pending_by_token(&hash_token(&token))
            .await?
            .ok_or(PermissionGroupError::NotFound)?;
        let user = self
            .user_repository
            .get_by_id(user_id)
            .await?
            .ok_or(PermissionGroupError::NotFound)?;
        if user
            .email
            .as_deref()
            .map(str::to_ascii_lowercase)
            .as_deref()
            != Some(invite.email.as_str())
        {
            return Err(PermissionGroupError::Escalation(
                "invite email does not match authenticated user".into(),
            ));
        }
        if self
            .member_repository
            .role(user_id, invite.organization_id)
            .await?
            .is_some()
        {
            return Err(PermissionGroupError::Invalid(
                "user is already an organization member".into(),
            ));
        }
        self.member_repository
            .add_member_with_group(
                invite.role.as_deref().unwrap_or("MEMBER"),
                user_id,
                invite.organization_id,
                invite.group_id,
            )
            .await?;
        self.invite_repository
            .set_status(invite.id.ok_or(PermissionGroupError::NotFound)?, "ACCEPTED")
            .await?;
        self.audit(
            user_id,
            invite.organization_id,
            "INVITE_ACCEPTED",
            "organization_invite",
            invite.id.map(|id| id.to_string()),
            None,
        )
        .await;
        Ok(())
    }

    pub async fn list_groups(
        &self,
        organization_id: i64,
    ) -> Result<Vec<PermissionGroupDto>, PermissionGroupError> {
        let groups = self.repository.list_groups(organization_id).await?;
        let mut result = Vec::with_capacity(groups.len());
        for group in groups {
            result.push(PermissionGroupDto {
                actions: self.repository.group_actions(group.id).await?,
                id: group.id,
                name: group.name,
                is_system: group.is_system,
                organization_id: group.organization_id,
                created_at: group.created_at,
                updated_at: group.updated_at,
            });
        }
        Ok(result)
    }

    pub async fn list_members(
        &self,
        organization_id: i64,
    ) -> Result<
        Vec<crate::db::models::organization_members::OrganizationMember>,
        PermissionGroupError,
    > {
        Ok(self
            .member_repository
            .list_for_organization(organization_id)
            .await?)
    }

    pub async fn update_member_role(
        &self,
        actor_id: i64,
        organization_id: i64,
        user_id: i64,
        role: String,
    ) -> Result<(), PermissionGroupError> {
        if !self.actor_is_privileged(actor_id, organization_id).await? {
            return Err(PermissionGroupError::Escalation(
                "only organization admins can change roles".into(),
            ));
        }
        let role = role.to_ascii_uppercase();
        if !matches!(role.as_str(), "ADMIN" | "MEMBER") {
            return Err(PermissionGroupError::Invalid(
                "role must be ADMIN or MEMBER".into(),
            ));
        }
        if user_id == actor_id && role != "ADMIN" {
            return Err(PermissionGroupError::Invalid(
                "you cannot demote yourself".into(),
            ));
        }
        if !self
            .member_repository
            .update_role(user_id, organization_id, &role)
            .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.audit(
            actor_id,
            organization_id,
            "MEMBER_ROLE_UPDATED",
            "organization_member",
            Some(user_id.to_string()),
            Some(serde_json::json!({"role": role}).to_string()),
        )
        .await;
        Ok(())
    }

    pub async fn remove_member(
        &self,
        actor_id: i64,
        organization_id: i64,
        user_id: i64,
    ) -> Result<(), PermissionGroupError> {
        if !self.actor_is_privileged(actor_id, organization_id).await? {
            return Err(PermissionGroupError::Escalation(
                "only organization admins can remove members".into(),
            ));
        }
        if user_id == actor_id {
            return Err(PermissionGroupError::Invalid(
                "you cannot remove yourself".into(),
            ));
        }
        if !self
            .member_repository
            .remove_member(user_id, organization_id)
            .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.audit(
            actor_id,
            organization_id,
            "MEMBER_REMOVED",
            "organization_member",
            Some(user_id.to_string()),
            None,
        )
        .await;
        Ok(())
    }

    pub async fn list_policies(&self) -> Result<Vec<PermissionPolicyDto>, PermissionGroupError> {
        Ok(self
            .repository
            .all_policies()
            .await?
            .into_iter()
            .map(|policy| PermissionPolicyDto {
                id: policy.id,
                action: policy.action,
            })
            .collect())
    }

    pub async fn create_group(
        &self,
        actor_id: i64,
        organization_id: i64,
        body: SavePermissionGroupDto,
    ) -> Result<i64, PermissionGroupError> {
        let policy_ids = self
            .validated_policy_ids(actor_id, organization_id, &body.actions)
            .await?;
        let id = self
            .repository
            .create_group(organization_id, body.name.trim(), &policy_ids)
            .await?;
        self.audit(
            actor_id,
            organization_id,
            "PERMISSION_GROUP_CREATED",
            "permission_group",
            Some(id.to_string()),
            None,
        )
        .await;
        Ok(id)
    }

    pub async fn update_group(
        &self,
        actor_id: i64,
        organization_id: i64,
        group_id: i64,
        body: SavePermissionGroupDto,
    ) -> Result<(), PermissionGroupError> {
        let policy_ids = self
            .validated_policy_ids(actor_id, organization_id, &body.actions)
            .await?;
        if !self
            .repository
            .update_group(organization_id, group_id, body.name.trim(), &policy_ids)
            .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.audit(
            actor_id,
            organization_id,
            "PERMISSION_GROUP_UPDATED",
            "permission_group",
            Some(group_id.to_string()),
            None,
        )
        .await;
        Ok(())
    }

    pub async fn delete_group(
        &self,
        actor_id: i64,
        organization_id: i64,
        group_id: i64,
    ) -> Result<(), PermissionGroupError> {
        if !self
            .repository
            .delete_group(organization_id, group_id)
            .await?
        {
            return Err(PermissionGroupError::Invalid(
                "group is protected, missing, or still assigned to members".into(),
            ));
        }
        self.audit(
            actor_id,
            organization_id,
            "PERMISSION_GROUP_DELETED",
            "permission_group",
            Some(group_id.to_string()),
            None,
        )
        .await;
        Ok(())
    }

    pub async fn assign_group(
        &self,
        actor_id: i64,
        organization_id: i64,
        user_id: i64,
        group_id: i64,
    ) -> Result<(), PermissionGroupError> {
        self.ensure_manageable_target(actor_id, user_id, organization_id)
            .await?;
        let actions = self.repository.group_actions(group_id).await?;
        self.ensure_subset(actor_id, organization_id, &actions)
            .await?;
        if !self
            .repository
            .assign_group(organization_id, user_id, group_id)
            .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.audit(
            actor_id,
            organization_id,
            "PERMISSION_GROUP_ASSIGNED",
            "organization_member",
            Some(user_id.to_string()),
            Some(serde_json::json!({"group_id": group_id}).to_string()),
        )
        .await;
        Ok(())
    }

    pub async fn replace_user_policies(
        &self,
        actor_id: i64,
        organization_id: i64,
        user_id: i64,
        body: ReplaceUserPoliciesDto,
    ) -> Result<(), PermissionGroupError> {
        self.ensure_manageable_target(actor_id, user_id, organization_id)
            .await?;
        let all = self.repository.all_policies().await?;
        let by_id = all
            .into_iter()
            .map(|p| (p.id, p.action))
            .collect::<std::collections::HashMap<_, _>>();
        let mut actions = Vec::with_capacity(body.policies.len());
        let mut policies = Vec::with_capacity(body.policies.len());
        for entry in &body.policies {
            let effect = entry.effect.to_ascii_uppercase();
            if effect != "GRANT" && effect != "DENY" {
                return Err(PermissionGroupError::Invalid(format!(
                    "invalid effect: {}",
                    entry.effect
                )));
            }
            let action = by_id.get(&entry.policy_id).ok_or_else(|| {
                PermissionGroupError::Invalid(format!("unknown policy id: {}", entry.policy_id))
            })?;
            actions.push(action.clone());
            policies.push((entry.policy_id, effect));
        }
        self.ensure_subset(actor_id, organization_id, &actions)
            .await?;
        let borrowed = policies
            .iter()
            .map(|(id, effect)| (*id, effect.as_str()))
            .collect::<Vec<_>>();
        if !self
            .repository
            .replace_user_policies(organization_id, user_id, &borrowed)
            .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.audit(
            actor_id,
            organization_id,
            "USER_POLICIES_REPLACED",
            "organization_member",
            Some(user_id.to_string()),
            Some(serde_json::json!({"policies": policies}).to_string()),
        )
        .await;
        Ok(())
    }

    async fn validated_policy_ids(
        &self,
        actor_id: i64,
        organization_id: i64,
        requested: &[String],
    ) -> Result<Vec<i64>, PermissionGroupError> {
        self.ensure_subset(actor_id, organization_id, requested)
            .await?;
        let policies = self.repository.all_policies().await?;
        let by_action = policies
            .into_iter()
            .map(|p| (p.action, p.id))
            .collect::<std::collections::HashMap<_, _>>();
        requested
            .iter()
            .map(|action| {
                by_action.get(action).copied().ok_or_else(|| {
                    PermissionGroupError::Invalid(format!("unknown action: {action}"))
                })
            })
            .collect()
    }

    async fn ensure_subset(
        &self,
        actor_id: i64,
        organization_id: i64,
        requested: &[String],
    ) -> Result<(), PermissionGroupError> {
        if self.actor_is_privileged(actor_id, organization_id).await? {
            return Ok(());
        }
        let effective = self
            .repository
            .effective_actions(actor_id, organization_id)
            .await?
            .into_iter()
            .collect::<HashSet<_>>();
        if let Some(action) = missing_delegated_action(&effective, requested) {
            return Err(PermissionGroupError::Escalation(action.to_owned()));
        }
        Ok(())
    }

    async fn actor_is_privileged(
        &self,
        actor_id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        if self.permission_service.is_platform_owner(actor_id).await?
            || self
                .permission_service
                .has_legacy_full_access(actor_id)
                .await?
        {
            return Ok(true);
        }
        Ok(matches!(
            self.member_repository
                .role(actor_id, organization_id)
                .await?
                .as_deref(),
            Some("ADMIN")
        ))
    }

    async fn ensure_manageable_target(
        &self,
        actor_id: i64,
        user_id: i64,
        organization_id: i64,
    ) -> Result<(), PermissionGroupError> {
        if self.actor_is_privileged(actor_id, organization_id).await? {
            return Ok(());
        }
        let role = self
            .member_repository
            .role(user_id, organization_id)
            .await?;
        if role.as_deref() != Some(UserRole::Member.as_str()) {
            return Err(PermissionGroupError::Escalation(
                "cannot manage an admin or non-member".into(),
            ));
        }
        Ok(())
    }
}

fn missing_delegated_action<'a>(
    actor_permissions: &HashSet<String>,
    requested: &'a [String],
) -> Option<&'a str> {
    requested
        .iter()
        .find(|action| !actor_permissions.contains(action.as_str()))
        .map(String::as_str)
}

fn hash_token(token: &str) -> String {
    Sha256::digest(token.trim().as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::missing_delegated_action;
    use std::collections::HashSet;

    #[test]
    fn member_can_delegate_a_subset_of_own_permissions() {
        let owned = ["app:read", "app:deploy", "server:read"]
            .into_iter()
            .map(str::to_owned)
            .collect::<HashSet<_>>();
        let requested = vec!["app:read".to_owned(), "server:read".to_owned()];

        assert_eq!(missing_delegated_action(&owned, &requested), None);
    }

    #[test]
    fn member_cannot_delegate_a_permission_they_do_not_have() {
        let owned = ["app:read"]
            .into_iter()
            .map(str::to_owned)
            .collect::<HashSet<_>>();
        let requested = vec!["app:read".to_owned(), "server:delete".to_owned()];

        assert_eq!(
            missing_delegated_action(&owned, &requested),
            Some("server:delete")
        );
    }
}
