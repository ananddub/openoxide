use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct SavePermissionGroupDto {
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct PermissionGroupDto {
    pub id: i64,
    pub name: String,
    pub is_system: bool,
    pub organization_id: Option<i64>,
    pub actions: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct PermissionPolicyDto {
    pub id: i64,
    pub action: String,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct AssignPermissionGroupDto {
    #[validate(range(min = 1))]
    pub group_id: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct UserPolicyEntryDto {
    pub policy_id: i64,
    pub effect: String,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct ReplaceUserPoliciesDto {
    pub policies: Vec<UserPolicyEntryDto>,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct UpdateMemberRoleDto {
    pub role: String,
}

#[derive(Debug, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct OrganizationMemberDto {
    pub id: Option<i64>,
    pub user_id: i64,
    pub organization_id: i64,
    pub role: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct CreateOrganizationInviteDto {
    pub email: String,
    pub role: String,
    pub group_id: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct OrganizationInviteDto {
    pub id: Option<i64>,
    pub email: String,
    pub role: Option<String>,
    pub group_id: i64,
    pub organization_id: i64,
    pub invited_by: i64,
    pub expired_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct AcceptOrganizationInviteDto {
    pub token: String,
}

impl From<crate::db::models::organization_invites::OrganizationInvite> for OrganizationInviteDto {
    fn from(value: crate::db::models::organization_invites::OrganizationInvite) -> Self {
        Self {
            id: value.id,
            email: value.email,
            role: value.role,
            group_id: value.group_id,
            organization_id: value.organization_id,
            invited_by: value.invited_by,
            expired_at: value.expired_at,
            created_at: value.created_at,
        }
    }
}
