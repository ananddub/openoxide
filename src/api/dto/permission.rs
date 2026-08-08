use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct SavePermissionGroupDto {
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct PermissionGroupDto {
    pub id: i64,
    pub name: String,
    pub is_system: bool,
    pub organization_id: Option<i64>,
    pub actions: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
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
