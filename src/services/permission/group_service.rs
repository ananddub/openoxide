use std::{collections::HashSet, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::permission::{
        PermissionGroupDto, PermissionPolicyDto, ReplaceUserPoliciesDto, SavePermissionGroupDto,
    },
    db::repository::{OrganizationMemberRepository, PermissionGroupRepository},
};

use super::{PermissionService, UserRole};

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
}

#[singleton]
impl PermissionGroupService {
    pub fn new(
        repository: Arc<PermissionGroupRepository>,
        member_repository: Arc<OrganizationMemberRepository>,
        permission_service: Arc<PermissionService>,
    ) -> Self {
        Self {
            repository,
            member_repository,
            permission_service,
        }
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
        self.repository
            .create_group(organization_id, body.name.trim(), &policy_ids)
            .await
            .map_err(Into::into)
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
        Ok(())
    }

    pub async fn delete_group(
        &self,
        organization_id: i64,
        group_id: i64,
    ) -> Result<(), PermissionGroupError> {
        if !self
            .repository
            .delete_group(organization_id, group_id)
            .await?
        {
            return Err(PermissionGroupError::NotFound);
        }
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
