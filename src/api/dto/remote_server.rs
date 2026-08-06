use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::servers::Server;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateRemoteServerDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub ip_address: String,
    #[serde(default = "default_port")]
    pub port: i64,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_server_type")]
    pub server_type: String,
    pub ssh_key_id: Option<i64>,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TestRemoteServerConnectionDto {
    #[validate(length(min = 1, max = 255))]
    pub ip_address: String,
    #[serde(default = "default_port")]
    pub port: i64,
    #[serde(default = "default_username")]
    pub username: String,
    pub ssh_key_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchRemoteServerDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub port: Option<i64>,
    pub username: Option<String>,
    pub server_status: Option<String>,
    pub server_type: Option<String>,
    pub enable_docker_cleanup: Option<i64>,
    pub log_cleanup_cron: Option<String>,
    pub command: Option<String>,
    pub metrics_config: Option<String>,
    pub ssh_key_id: Option<i64>,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RemoteServerResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub ip_address: String,
    pub port: i64,
    pub username: String,
    pub app_name: String,
    pub server_status: String,
    pub server_type: String,
    pub enable_docker_cleanup: i64,
    pub log_cleanup_cron: Option<String>,
    pub command: String,
    pub metrics_config: String,
    pub ssh_key_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

impl From<Server> for RemoteServerResponseDto {
    fn from(value: Server) -> Self {
        Self {
            id: value.id.expect("persisted server must have an id"),
            name: value.name,
            description: value.description,
            ip_address: value.ip_address,
            port: value.port,
            username: value.username,
            app_name: value.app_name,
            server_status: value.server_status,
            server_type: value.server_type,
            enable_docker_cleanup: value.enable_docker_cleanup,
            log_cleanup_cron: value.log_cleanup_cron,
            command: value.command,
            metrics_config: value.metrics_config,
            ssh_key_id: value.ssh_key_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            build_memory_limit: value.build_memory_limit,
            build_cpu_limit: value.build_cpu_limit,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RemoteServerActionResponseDto {
    pub server: RemoteServerResponseDto,
    pub action: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct MigrateServerDependenciesDto {
    #[validate(range(min = 1))]
    pub target_server_id: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ServerDependencyMigrationDto {
    pub migration_id: String,
    pub status: String,
    pub source_server_id: i64,
    pub target_server_id: i64,
    pub applications: i64,
    pub build_assignments: i64,
    pub compose_projects: i64,
    pub databases: i64,
    pub certificates: i64,
    pub schedules: i64,
    pub queued_applications: i64,
    pub queued_compose_projects: i64,
    pub error: Option<String>,
}

impl TryFrom<crate::repository::ServerMigration> for ServerDependencyMigrationDto {
    type Error = sqlx::Error;

    fn try_from(value: crate::repository::ServerMigration) -> Result<Self, Self::Error> {
        let applications: Vec<i64> = serde_json::from_str(&value.application_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let build: Vec<i64> = serde_json::from_str(&value.build_application_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let compose: Vec<i64> = serde_json::from_str(&value.compose_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let certificates: Vec<i64> = serde_json::from_str(&value.certificate_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let schedules: Vec<i64> = serde_json::from_str(&value.schedule_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(Self {
            migration_id: value.id,
            status: value.status,
            source_server_id: value.source_server_id,
            target_server_id: value.target_server_id,
            applications: applications.len() as i64,
            build_assignments: build.len() as i64,
            compose_projects: compose.len() as i64,
            databases: 0,
            certificates: certificates.len() as i64,
            schedules: schedules.len() as i64,
            queued_applications: value.queued_applications,
            queued_compose_projects: value.queued_compose_projects,
            error: value.error,
        })
    }
}

fn default_port() -> i64 {
    22
}

fn default_username() -> String {
    "root".into()
}

fn default_server_type() -> String {
    "DEPLOY".into()
}
