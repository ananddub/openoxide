use auto_di::singleton;
use std::sync::Arc;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        groups::GroupsGetUserFinalPermissionsParams,
        models::UserPolicy,
        resource_access::ResourceAccessCheckAccessParams,
        user_policy::{UserPolicyDeleteParams, UserPolicyUpsertParams},
    },
};
use crate::services::permission::types::{PolicyAction, ResourceType, UserRole};

pub struct PermissionService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl PermissionService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn is_platform_owner(&self, user_id: i64) -> Result<bool, sqlx::Error> {
        match self.queries.get_user_by_i_d(user_id).await {
            Ok(user) => Ok(user
                .role
                .is_some_and(|role| UserRole::from(role.as_str()) == UserRole::Owner)),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
    }

    pub async fn resolve_organization(&self, user_id: i64) -> Result<Option<i64>, sqlx::Error> {
        match self
            .queries
            .organization_members_first_organization_id(user_id)
            .await
        {
            Ok(organization_id) => Ok(Some(organization_id)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
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

        let member_role = match self
            .queries
            .organization_members_role(user_id, org_id)
            .await
        {
            Ok(role) => role,
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error),
        };

        if let Some(role) = member_role.as_deref() {
            let role_upper = role.to_uppercase();
            if role_upper == "ADMIN" || role_upper == "OWNER" {
                return Ok(true);
            }
        } else {
            return Ok(false);
        }

        // 2. Fetch final permissions (evaluates user_policy & group_policy)
        let perms = self
            .queries
            .groups_get_user_final_permissions(&GroupsGetUserFinalPermissionsParams {
                user_id: user_id.to_string(),
                org_id: org_id.to_string(),
                user_id2: user_id.to_string(),
                organization_id: org_id.to_string(),
            })
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

        let hits = self
            .queries
            .resource_access_check_access(&ResourceAccessCheckAccessParams {
                user_id,
                org_id,
                resource_type: resource_type.as_str().to_string(),
                resource_id,
            })
            .await?;
        Ok(hits > 0)
    }

    /// Set an explicit GRANT or DENY policy override for a user
    pub async fn set_user_policy_override(
        &self,
        user_id: i64,
        org_id: i64,
        policy_id: i64,
        effect: crate::services::permission::types::PolicyEffect,
    ) -> Result<UserPolicy, sqlx::Error> {
        let policy = self
            .queries
            .user_policy_upsert(&UserPolicyUpsertParams {
                user_id,
                org_id,
                policy_id,
                effect: effect.as_str().to_string(),
            })
            .await?;
        Ok(UserPolicy {
            id: policy.id,
            user_id: policy.user_id,
            org_id: policy.org_id,
            policy_id: policy.policy_id,
            effect: policy.effect,
            created_at: policy.created_at,
        })
    }

    /// Remove an explicit user policy override
    pub async fn remove_user_policy_override(
        &self,
        user_id: i64,
        org_id: i64,
        policy_id: i64,
    ) -> Result<(), sqlx::Error> {
        self.queries
            .user_policy_delete(&UserPolicyDeleteParams {
                user_id,
                org_id,
                policy_id,
            })
            .await
    }
}
