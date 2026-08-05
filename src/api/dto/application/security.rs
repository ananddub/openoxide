use crate::db::models::security::Security;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertApplicationSecurityDto {
    #[validate(length(min = 1, max = 255))]
    pub username: String,
    #[validate(length(min = 8, max = 1_000))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationSecurityResponseDto {
    pub id: i64,
    pub username: String,
    pub has_password: bool,
    pub application_id: i64,
    pub created_at: i64,
}
impl From<Security> for ApplicationSecurityResponseDto {
    fn from(value: Security) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            username: value.username,
            has_password: !value.password.is_empty(),
            application_id: value.application_id,
            created_at: value.created_at,
        }
    }
}
