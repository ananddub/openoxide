use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::patches::Patch;

use os::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, poem_openapi::Enum)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ApplicationPatchType {
        default = Create;

        Create => "CREATE",
        Update => "UPDATE",
        Delete => "DELETE",
    }
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertApplicationPatchDto {
    pub patch_type: ApplicationPatchType,
    #[validate(length(min = 1, max = 4_096))]
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ApplicationPatchResponseDto {
    pub id: i64,
    pub patch_type: String,
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
    pub application_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Patch> for ApplicationPatchResponseDto {
    fn from(value: Patch) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            patch_type: value.patch_type,
            file_path: value.file_path,
            enabled: value.enabled != 0,
            content: value.content,
            application_id: value.application_id.unwrap_or_default(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
