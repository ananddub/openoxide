use auto_di::singleton;
use std::sync::Arc;

use crate::db::repository::{
    GroupRepository, OrganizationMemberRepository, ResourceAccessRepository, UserPolicyRepository,
    UserRepository,
};
use crate::services::permission::types::{PolicyAction, ResourceType, UserRole};

pub struct PermissionService {
    user_repo: Arc<UserRepository>,
    group_repo: Arc<GroupRepository>,
    resource_access_repo: Arc<ResourceAccessRepository>,
    user_policy_repo: Arc<UserPolicyRepository>,
    member_repo: Arc<OrganizationMemberRepository>,
}

#[singleton]
impl PermissionService {
    pub async fn is_platform_owner(&self, user_id: i64) -> Result<bool, sqlx::Error> {
        Ok(self
            .user_repo
            .get_by_id(user_id)
            .await?
            .and_then(|user| user.role)
            .is_some_and(|role| UserRole::from(role.as_str()) == UserRole::Owner))
    }

    pub fn new(
        user_repo: Arc<UserRepository>,
        group_repo: Arc<GroupRepository>,
        resource_access_repo: Arc<ResourceAccessRepository>,
        user_policy_repo: Arc<UserPolicyRepository>,
        member_repo: Arc<OrganizationMemberRepository>,
    ) -> Self {
        Self {
            user_repo,
            group_repo,
            resource_access_repo,
            user_policy_repo,
            member_repo,
        }
    }

    pub async fn resolve_organization(&self, user_id: i64) -> Result<Option<i64>, sqlx::Error> {
        self.member_repo.first_organization_id(user_id).await
    }

    /// Check if user has permission for a specific PolicyAction
    pub async fn check_permission(
        &self,
        user_id: i64,
        org_id: i64,
        action: PolicyAction,
    ) -> Result<bool, sqlx::Error> {
        // Platform owners retain global access.
        if self.is_platform_owner(user_id).await? {
            return Ok(true);
        }

        let member_role = self.member_repo.role(user_id, org_id).await?;

        if member_role.is_none() {
            return Ok(false);
        }

        match member_role.as_deref() {
            Some("ADMIN") => return Ok(true),
            Some("MEMBER") => {}
            _ => return Ok(false),
        }

        // 2. Fetch final permissions via GroupRepository (evaluates user_policy & group_policy)
        let perms = self
            .group_repo
            .get_user_final_permissions(user_id, org_id)
            .await?;
        let action_str = action.as_str();

        Ok(perms.iter().any(|p| p == &action_str))
    }

    /// Check if user has access to a specific granular resource
    pub async fn check_resource_access(
        &self,
        user_id: i64,
        org_id: i64,
        resource_type: ResourceType,
        resource_id: i64,
    ) -> Result<bool, sqlx::Error> {
        // Check OWNER status first
        if self.is_platform_owner(user_id).await? {
            return Ok(true);
        }

        self.resource_access_repo
            .check_access(user_id, org_id, resource_type.as_str(), resource_id)
            .await
    }

    /// Set an explicit GRANT or DENY policy override for a user
    pub async fn set_user_policy_override(
        &self,
        user_id: i64,
        org_id: i64,
        policy_id: i64,
        effect: crate::services::permission::types::PolicyEffect,
    ) -> Result<crate::db::models::user_policy::UserPolicy, sqlx::Error> {
        self.user_policy_repo
            .upsert(user_id, org_id, policy_id, effect.as_str())
            .await
    }

    /// Remove an explicit user policy override
    pub async fn remove_user_policy_override(
        &self,
        user_id: i64,
        org_id: i64,
        policy_id: i64,
    ) -> Result<(), sqlx::Error> {
        self.user_policy_repo
            .delete(user_id, org_id, policy_id)
            .await
    }
}
