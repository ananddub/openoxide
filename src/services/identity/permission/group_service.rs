use getrandom::fill;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashSet, sync::Arc};

use auto_di::singleton;

use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    audit_logs::AuditLogsCreateParams,
    groups::GroupsGetUserFinalPermissionsParams,
    models::OrganizationInvites,
    organization_invites::OrganizationInvitesCreateParams,
    organization_members::{
        OrganizationMembersAddMemberWithGroupParams,
        OrganizationMembersListWithUserForOrganizationRow, OrganizationMembersUpdateRoleParams,
    },
    permission_groups::{
        PermissionGroupsAssignGroupParams, PermissionGroupsInsertUserPolicyParams,
        PermissionGroupsUpdateGroupNameParams,
    },
    users::UsersCreateParams,
};

use super::{PermissionService, UserRole};
use crate::services::identity::auth::{hash_password, notification_configs_for_user};
use crate::services::integrations::notification::senders::send_resend_to;
use crate::services::integrations::notification::{email::send_email_to, message::NotificationMessage};

#[derive(Debug, Deserialize)]
pub struct SavePermissionGroupDto {
    pub name: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PermissionGroupDto {
    pub id: i64,
    pub name: String,
    pub is_system: bool,
    pub organization_id: Option<i64>,
    pub actions: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PermissionPolicyDto {
    pub id: i64,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct UserPolicyEntryDto {
    pub policy_id: i64,
    pub effect: String,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceUserPoliciesDto {
    pub policies: Vec<UserPolicyEntryDto>,
}

#[derive(Debug, Deserialize)]
pub struct AddOrganizationMemberDto {
    pub email: String,
    pub password: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationInviteDto {
    pub email: String,
    pub role: String,
    pub group_id: i64,
}

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
    queries: Arc<FlashQueries>,
    permission_service: Arc<PermissionService>,
}

#[singleton]
impl PermissionGroupService {
    pub fn new(queries: Arc<FlashQueries>, permission_service: Arc<PermissionService>) -> Self {
        Self {
            queries,
            permission_service,
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
        let user = self.queries.get_user_by_i_d(actor_id).await.ok();
        if let Err(error) = self
            .queries
            .audit_logs_create(&AuditLogsCreateParams {
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
                resource_id: id.unwrap_or_default(),
                resource_name: String::new(),
                metadata: metadata.unwrap_or_default(),
                organization_id,
                user_id: actor_id,
                created_at: chrono::Utc::now().timestamp(),
            })
            .await
        {
            tracing::warn!(%error, action, "could not write permission audit event");
        }
    }

    pub async fn list_invites(
        &self,
        organization_id: i64,
    ) -> Result<Vec<OrganizationInvites>, PermissionGroupError> {
        self.queries.organization_invites_purge_expired().await?;
        Ok(self
            .queries
            .organization_invites_list_pending_for_organization(organization_id)
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
        if !self.actor_is_privileged(actor_id, organization_id).await? {
            let actions = self.queries.group_actions(body.group_id).await?;
            self.ensure_subset(actor_id, organization_id, &actions)
                .await?;
        }
        let mut bytes = [0_u8; 32];
        fill(&mut bytes)
            .map_err(|_| PermissionGroupError::Invalid("could not generate invite token".into()))?;
        let token = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
        let token_hash = hash_token(&token);
        let now = chrono::Utc::now().timestamp();
        self.queries
            .organization_invites_create(&OrganizationInvitesCreateParams {
                email: email.clone(),
                role: role.clone(),
                status: "PENDING".into(),
                token: token_hash,
                group_id: body.group_id,
                organization_id,
                invited_by: actor_id,
                expired_at: now + 7 * 86400,
                created_at: now,
            })
            .await?;
        self.send_invite_email(&email, &token, organization_id, actor_id)
            .await;
        self.audit(
            actor_id,
            organization_id,
            "INVITE_CREATED",
            "organization_invite",
            None,
            Some(serde_json::json!({"email": email, "role": role}).to_string()),
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
        let action = std::env::var("OPENOXIDE_PUBLIC_URL")
            .or_else(|_| std::env::var("RUSTPLOY_PUBLIC_URL"))
            .ok()
            .map(|base| format!("{}/accept-invite?token={token}", base.trim_end_matches('/')))
            .unwrap_or_else(|| format!("Invite token: {token}"));
        let message = NotificationMessage::new(
            "You have been invited to OpenOxide",
            format!(
                "You have been invited to join organization {organization_id}.\n\nAccept this invitation within 7 days:\n{action}"
            ),
        );
        let (email_config, resend_config) =
            match notification_configs_for_user(&self.queries, actor_id).await {
                Ok(configs) => configs,
                Err(error) => {
                    tracing::warn!(%error, "could not load invite email providers");
                    (None, None)
                }
            };
        if let Some(config) = email_config {
            if let Err(error) = send_email_to(&config, email, &message).await {
                tracing::warn!(%error, "organization invite email delivery failed");
            }
        } else if let Some(config) = resend_config {
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
        let invite = self.queries.organization_invites_get_by_id(invite_id).await?;
        if invite.organization_id != organization_id
            || invite.status.as_deref() != Some("PENDING")
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.queries
            .organization_invites_set_status("REJECTED", invite_id)
            .await?;
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
            .queries
            .organization_invites_find_pending_by_token(&hash_token(&token))
            .await?;
        let user = self.queries.get_user_by_i_d(user_id).await?;
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
            .member_role(user_id, invite.organization_id)
            .await?
            .is_some()
        {
            return Err(PermissionGroupError::Invalid(
                "user is already an organization member".into(),
            ));
        }
        self.queries
            .organization_members_add_member_with_group(&OrganizationMembersAddMemberWithGroupParams {
                role: invite.role.clone().unwrap_or_else(|| "MEMBER".into()),
                user_id,
                organization_id: invite.organization_id,
                group_id: invite.group_id,
            })
            .await?;
        self.queries
            .organization_invites_set_status("ACCEPTED", invite.id)
            .await?;
        self.audit(
            user_id,
            invite.organization_id,
            "INVITE_ACCEPTED",
            "organization_invite",
            Some(invite.id.to_string()),
            None,
        )
        .await;
        Ok(())
    }

    pub async fn list_groups(
        &self,
        organization_id: i64,
    ) -> Result<Vec<PermissionGroupDto>, PermissionGroupError> {
        let groups = self
            .queries
            .list_groups_for_organization(organization_id)
            .await?;
        let mut result = Vec::with_capacity(groups.len());
        for group in groups {
            result.push(PermissionGroupDto {
                actions: self.queries.group_actions(group.id).await?,
                id: group.id,
                name: group.name,
                is_system: group.is_system != 0,
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
    ) -> Result<Vec<OrganizationMembersListWithUserForOrganizationRow>, PermissionGroupError> {
        Ok(self
            .queries
            .organization_members_list_with_user_for_organization(organization_id)
            .await?)
    }

    pub async fn add_member(
        &self,
        actor_id: i64,
        organization_id: i64,
        body: AddOrganizationMemberDto,
    ) -> Result<(), PermissionGroupError> {
        if !self.actor_is_privileged(actor_id, organization_id).await? {
            return Err(PermissionGroupError::Escalation(
                "only organization admins can add members".into(),
            ));
        }

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

        let user_id = match self.queries.get_user_by_email(&email).await {
            Ok(user) => user.id,
            Err(sqlx::Error::RowNotFound) => {
                let password_raw = body.password.unwrap_or_else(|| "ChangeMe123!".to_string());
                let password_hash = hash_password(password_raw)
                    .await
                    .map_err(|_| PermissionGroupError::Invalid("failed to hash password".into()))?;
                let now = chrono::Utc::now().timestamp();
                self.queries
                    .users_create(&UsersCreateParams {
                        email: email.clone(),
                        last_name: String::new(),
                        first_name: String::new(),
                        avatar: String::new(),
                        role: "member".into(),
                        about_me: String::new(),
                        password: password_hash,
                        is_email_verify: 1,
                        email_verify_at: now,
                        two_factor_enable: 0,
                        is_registered: 1,
                        added_by: 0,
                        group_id: 1,
                        created_at: now,
                        updated_at: now,
                    })
                    .await?;
                self.queries.get_user_by_email(&email).await?.id
            }
            Err(error) => return Err(error.into()),
        };

        if self
            .member_role(user_id, organization_id)
            .await?
            .is_some()
        {
            return Err(PermissionGroupError::Invalid(
                "user is already a member of this organization".into(),
            ));
        }

        self.queries
            .organization_members_add_member_with_group(&OrganizationMembersAddMemberWithGroupParams {
                role,
                user_id,
                organization_id,
                group_id: 1,
            })
            .await?;
        Ok(())
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
        if self
            .member_role(user_id, organization_id)
            .await?
            .is_none()
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.queries
            .organization_members_update_role(&OrganizationMembersUpdateRoleParams {
                role: role.clone(),
                user_id,
                organization_id,
            })
            .await?;
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
        if self
            .member_role(user_id, organization_id)
            .await?
            .is_none()
        {
            return Err(PermissionGroupError::NotFound);
        }
        self.queries
            .organization_members_remove_member(user_id, organization_id)
            .await?;
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
            .queries
            .list_policies()
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
            .queries
            .permission_groups_create_group(body.name.trim(), organization_id)
            .await?;
        for policy_id in &policy_ids {
            self.queries
                .permission_groups_add_policy_to_group(&id.to_string(), &policy_id.to_string())
                .await?;
        }
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
        let updated = self
            .queries
            .permission_groups_update_group_name(&PermissionGroupsUpdateGroupNameParams {
                name: body.name.trim().to_string(),
                id: group_id,
                organization_id,
            })
            .await?;
        if updated.is_empty() {
            return Err(PermissionGroupError::NotFound);
        }
        self.queries
            .permission_groups_clear_group_policies(group_id)
            .await?;
        for policy_id in &policy_ids {
            self.queries
                .permission_groups_add_policy_to_group(&group_id.to_string(), &policy_id.to_string())
                .await?;
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
        let assigned = self
            .queries
            .permission_groups_delete_group_assigned(organization_id, group_id)
            .await?;
        if assigned > 0 {
            return Err(PermissionGroupError::Invalid(
                "group is protected, missing, or still assigned to members".into(),
            ));
        }
        let deleted = self
            .queries
            .permission_groups_delete_group(group_id, organization_id)
            .await?;
        if deleted.is_empty() {
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
        let actions = self.queries.group_actions(group_id).await?;
        self.ensure_subset(actor_id, organization_id, &actions)
            .await?;
        let hit = self
            .queries
            .permission_groups_valid_group_scope(group_id, organization_id)
            .await?;
        if hit == 0 {
            return Err(PermissionGroupError::NotFound);
        }
        let assigned = self
            .queries
            .permission_groups_assign_group(&PermissionGroupsAssignGroupParams {
                group_id,
                user_id,
                organization_id,
            })
            .await?;
        if assigned.is_empty() {
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
        let all = self.queries.list_policies().await?;
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
        let member = self
            .queries
            .permission_groups_member_exists(user_id, organization_id)
            .await?;
        if member == 0 {
            return Err(PermissionGroupError::NotFound);
        }
        self.queries
            .permission_groups_delete_user_policies(user_id, organization_id)
            .await?;
        for (policy_id, effect) in &policies {
            self.queries
                .permission_groups_insert_user_policy(&PermissionGroupsInsertUserPolicyParams {
                    user_id,
                    org_id: organization_id,
                    policy_id: *policy_id,
                    effect: effect.clone(),
                })
                .await?;
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
        let policies = self.queries.list_policies().await?;
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
            .queries
            .groups_get_user_final_permissions(&GroupsGetUserFinalPermissionsParams {
                user_id: actor_id.to_string(),
                org_id: organization_id.to_string(),
                user_id2: actor_id.to_string(),
                organization_id: organization_id.to_string(),
            })
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
        if self.permission_service.is_platform_owner(actor_id).await? {
            return Ok(true);
        }
        let role = self.member_role(actor_id, organization_id).await?;
        if let Some(r) = role.as_deref() {
            let upper = r.to_uppercase();
            if upper == "ADMIN" || upper == "OWNER" {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn member_role(
        &self,
        user_id: i64,
        organization_id: i64,
    ) -> Result<Option<String>, sqlx::Error> {
        match self
            .queries
            .organization_members_role(user_id, organization_id)
            .await
        {
            Ok(role) => Ok(role),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
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
        let role = self.member_role(user_id, organization_id).await?;
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
