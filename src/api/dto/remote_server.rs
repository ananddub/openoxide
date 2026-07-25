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

fn default_port() -> i64 {
    22
}

fn default_username() -> String {
    "root".into()
}

fn default_server_type() -> String {
    "DEPLOY".into()
}
