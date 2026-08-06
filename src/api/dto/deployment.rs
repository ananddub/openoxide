use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db::models::deployments::Deployment;
use crate::utils::builder::custom_type::{ActiveDeploySnapshot, IdType};

#[derive(Debug, Clone, Copy, Default, Deserialize, poem_openapi::Enum)]
pub enum DockerLogStream {
    #[default]
    All,
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Deserialize, poem_openapi::Object)]
pub struct DeploymentListQuery {
    pub status: Option<String>,
    pub state: Option<String>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub database_id: Option<i64>,
    pub server_id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DeploymentResponseDto {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub state: String,
    pub log_path: String,
    pub pid: Option<String>,
    pub error_message: Option<String>,
    pub is_preview_deployment: i64,
    pub started_at: Option<i64>,
    pub last_state_at: Option<i64>,
    pub finished_at: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub database_id: Option<i64>,
    pub database_kind: Option<String>,
    pub server_id: Option<i64>,
    pub created_at: i64,
}

impl From<Deployment> for DeploymentResponseDto {
    fn from(value: Deployment) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            status: value.status,
            state: value.state,
            log_path: value.log_path,
            pid: value.pid,
            error_message: value.error_message,
            is_preview_deployment: value.is_preview_deployment,
            started_at: value.started_at,
            last_state_at: value.last_state_at,
            finished_at: value.finished_at,
            application_id: value.application_id,
            compose_id: value.compose_id,
            database_id: value.database_id,
            database_kind: value.database_kind,
            server_id: value.server_id,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ActiveDeploymentDto {
    pub kind: String,
    pub id: i64,
    pub project_id: i64,
    pub environment_id: i64,
    pub state: String,
}

impl From<ActiveDeploySnapshot> for ActiveDeploymentDto {
    fn from(value: ActiveDeploySnapshot) -> Self {
        let (kind, id) = match value.id {
            IdType::AppId(id) => ("application", id),
            IdType::ComposeId(id) => ("compose", id),
            IdType::DatabaseId(id) => ("database", id),
        };

        Self {
            kind: kind.into(),
            id,
            project_id: value.project_id,
            environment_id: value.env_id,
            state: format!("{:?}", value.state),
        }
    }
}

#[derive(Debug, Clone, Deserialize, poem_openapi::Object)]
pub struct DockerLogQuery {
    pub server_id: Option<i64>,
    pub tail: Option<usize>,
    pub timestamps: Option<bool>,
    pub follow: Option<bool>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub stream: Option<DockerLogStream>,
}

#[derive(Debug, Clone, Deserialize, poem_openapi::Object)]
pub struct DockerStatsQuery {
    pub server_id: Option<i64>,
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, poem_openapi::Object)]
pub struct ComposeLogQuery {
    pub server_id: Option<i64>,
    pub project_name: Option<String>,
    pub project_dir: Option<String>,
    pub file: Option<String>,
    pub service: Option<String>,
    pub tail: Option<usize>,
    pub timestamps: Option<bool>,
    pub follow: Option<bool>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub stream: Option<DockerLogStream>,
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct DeploymentSseEventDto {
    pub r#type: String,
    pub line: Option<String>,
    pub state: Option<String>,
    pub message: Option<String>,
    pub skipped: Option<u64>,
    pub stats: Option<Value>,
}
