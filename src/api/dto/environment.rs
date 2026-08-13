use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::environments::Environment;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateEnvironmentDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[serde(default)]
    pub env_var: String,
    #[serde(default)]
    pub is_default: bool,
    pub project_id: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchEnvironmentDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct EnvironmentResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub env_var: String,
    pub is_default: bool,
    pub project_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Environment> for EnvironmentResponseDto {
    fn from(value: Environment) -> Self {
        Self {
            id: value.id.expect("persisted environment must have an id"),
            name: value.name,
            description: value.description,
            env_var: value.env_var,
            is_default: value.is_default != 0,
            project_id: value.project_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
