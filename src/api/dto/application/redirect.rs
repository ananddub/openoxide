use crate::db::models::redirects::Redirect;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertApplicationRedirectDto {
    #[validate(length(min = 1, max = 2_000))]
    pub regex: String,
    #[validate(length(min = 1, max = 2_000))]
    pub replacement: String,
    pub permanent: i64,
    pub unique_config_key: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationRedirectResponseDto {
    pub id: i64,
    pub regex: String,
    pub replacement: String,
    pub permanent: i64,
    pub unique_config_key: Option<i64>,
    pub application_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Redirect> for ApplicationRedirectResponseDto {
    fn from(value: Redirect) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            regex: value.regex,
            replacement: value.replacement,
            permanent: value.permanent,
            unique_config_key: value.unique_config_key,
            application_id: value.application_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
