use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::db::repository::{
    GroupRepository, ResourceAccessRepository, UserPolicyRepository, UserRepository,
};
use crate::services::permission::types::{PolicyAction, ResourceType, UserRole};

pub struct PermissionService {
    user_repo: Arc<UserRepository>,
    group_repo: Arc<GroupRepository>,
    resource_access_repo: Arc<ResourceAccessRepository>,
    user_policy_repo: Arc<UserPolicyRepository>,
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PermissionService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        group_repo: Arc<GroupRepository>,
        resource_access_repo: Arc<ResourceAccessRepository>,
        user_policy_repo: Arc<UserPolicyRepository>,
        pool: Arc<SqlitePool>,
    ) -> Self {
        Self {
            user_repo,
            group_repo,
            resource_access_repo,
            user_policy_repo,
            pool,
        }
    }

    pub async fn resolve_organization(&self, user_id: i64) -> Result<Option<i64>, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT organization_id FROM organization_members WHERE user_id=? ORDER BY id LIMIT 1",
        )
        .bind(user_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    /// Check if user has permission for a specific PolicyAction
    pub async fn check_permission(
        &self,
        user_id: i64,
        org_id: i64,
        action: PolicyAction,
    ) -> Result<bool, sqlx::Error> {
        // Platform owners retain global access.
        if let Some(user) = self.user_repo.get_by_id(user_id).await? {
            let role_str = user.role.as_deref().unwrap_or("MEMBER");
            let role = UserRole::from(role_str);
            if role == UserRole::Owner {
                return Ok(true); // OWNER bypass
            }
        }

        let member_role = sqlx::query_scalar::<_, String>(
            "SELECT role FROM organization_members WHERE user_id=? AND organization_id=?",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_optional(self.pool.as_ref())
        .await?;

        if member_role.is_none() {
            return Ok(false);
        }

        let legacy = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM permission_legacy_full_access WHERE user_id=?)",
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        if legacy != 0 {
            return Ok(true);
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

        Ok(perms.iter().any(|p| p == action_str))
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
        if let Some(user) = self.user_repo.get_by_id(user_id).await? {
            let role_str = user.role.as_deref().unwrap_or("MEMBER");
            let role = UserRole::from(role_str);
            if role == UserRole::Owner {
                return Ok(true);
            }
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
