use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::projects::Project;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateProjectDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[serde(default)]
    pub env_var: String,
    pub organization_id: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchProjectDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub env_var: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ProjectResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub env_var: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Project> for ProjectResponseDto {
    fn from(value: Project) -> Self {
        Self {
            id: value.id.expect("persisted project must have an id"),
            name: value.name,
            description: value.description,
            env_var: value.env_var,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
