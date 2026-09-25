use auto_di::singleton;
use std::sync::Arc;

use crate::services::identity::permission::types::{PolicyAction, ResourceType};
use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        groups::GroupsGetUserFinalPermissionsParams,
        models::UserPolicy,
        resource_access::ResourceAccessCheckAccessParams,
        user_policy::{UserPolicyDeleteParams, UserPolicyUpsertParams},
    },
};

pub struct PermissionService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl PermissionService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    /// The setup account (first registered user) is the only platform
    /// owner. Signup hardcodes role='OWNER' on every account, so the role
    /// alone must not grant cross-organization power — otherwise every
    /// invited member would wildcard into their inviter's workspace.
    pub async fn is_platform_owner(&self, user_id: i64) -> Result<bool, sqlx::Error> {
        self.queries.users_is_platform_owner(user_id as i32).await
    }

    /// The caller's effective role and granted actions inside one
    /// organization. Owners (platform or org) and admins get the `*`
    /// wildcard; members get their evaluated group/user policy actions.
    /// Memberships are checked first so a non-member never learns the
    /// organization's action catalog.
    pub async fn my_permissions(
        &self,
        user_id: i64,
        org_id: i64,
    ) -> Result<(String, Vec<String>), sqlx::Error> {
        if self.is_platform_owner(user_id).await? {
            return Ok(("OWNER".into(), vec!["*".into()]));
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
        let role = match member_role.as_deref() {
            Some(role) => role.to_ascii_uppercase(),
            None => return Ok(("NONE".into(), Vec::new())),
        };
        if role == "OWNER" || role == "ADMIN" {
            return Ok((role, vec!["*".into()]));
        }
        let actions = self
            .queries
            .groups_get_user_final_permissions(&GroupsGetUserFinalPermissionsParams {
                user_id: user_id.to_string(),
                org_id: org_id.to_string(),
                user_id2: user_id.to_string(),
                organization_id: org_id.to_string(),
            })
            .await?;
        Ok((role, actions))
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

    /// Whether the user owns or joined the organization (header scoping).
    pub async fn is_organization_member(
        &self,
        user_id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        if self.is_platform_owner(user_id).await? {
            return Ok(true);
        }
        if self
            .queries
            .organization_members_role(user_id, organization_id)
            .await
            .is_ok()
        {
            return Ok(true);
        }
        match self.queries.get_organization_by_i_d(organization_id).await {
            Ok(org) => Ok(org.owner_id == user_id),
            Err(sqlx::Error::RowNotFound) => Ok(false),
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
        effect: crate::services::identity::permission::types::PolicyEffect,
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
