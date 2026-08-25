use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use validator::Validate;

use os::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum PostgresReplicationRole {
        default = Standalone;

        Standalone => "STANDALONE",
        Primary => "PRIMARY",
        Replica => "REPLICA",
    }
}

use crate::services::database::{DatabaseOperationResult, DatabaseRecord};
use crate::utils::builder::spec::MountType;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateDatabaseDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub docker_image: Option<String>,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub database_root_password: Option<String>,
    pub external_port: Option<i64>,
    pub external_grpc_port: Option<i64>,
    pub external_admin_port: Option<i64>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env_var: Option<String>,
    pub replica_sets: Option<i64>,
    pub sqld_node: Option<String>,
    pub sqld_primary_url: Option<String>,
    pub enable_namespaces: Option<i64>,
    pub network_ids: Option<Vec<String>>,
    pub detach_rustploy_network: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchDatabaseDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub docker_image: Option<String>,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub external_port: Option<i64>,
    pub external_grpc_port: Option<i64>,
    pub external_admin_port: Option<i64>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: Option<i64>,
    pub server_id: Option<i64>,
    pub network_ids: Option<Vec<String>>,
    pub detach_rustploy_network: Option<i64>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Validate, sqlx::FromRow, poem_openapi::Object, ts_rs::TS,
)]
pub struct DatabaseSwarmSettingsDto {
    pub health_check_swarm: Option<String>,
    pub restart_policy_swarm: Option<String>,
    pub placement_swarm: Option<String>,
    pub update_config_swarm: Option<String>,
    pub rollback_config_swarm: Option<String>,
    pub mode_swarm: Option<String>,
    pub labels_swarm: Option<String>,
    pub network_swarm: Option<String>,
    pub endpoint_spec_swarm: Option<String>,
    pub ulimits_swarm: Option<String>,
    pub stop_grace_period_swarm: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct DatabaseResponseDto {
    pub kind: crate::services::database::DatabaseKind,
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub docker_image: String,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub external_port: Option<i64>,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
    pub app_status: String,
    pub last_error: Option<String>,
    pub status: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertDatabaseMountDto {
    pub mount_type: MountType,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    #[validate(length(min = 1, max = 4_096))]
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct DatabaseMountResponseDto {
    pub id: i64,
    pub mount_type: String,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub mount_path: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<crate::db::models::mounts::Mount> for DatabaseMountResponseDto {
    fn from(value: crate::db::models::mounts::Mount) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            mount_type: value.mount_type,
            host_path: value.host_path,
            volume_name: value.volume_name,
            file_path: value.file_path,
            content: value.content,
            mount_path: value.mount_path,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<DatabaseRecord> for DatabaseResponseDto {
    fn from(value: DatabaseRecord) -> Self {
        Self {
            kind: value.kind,
            id: value.id,
            name: value.name,
            app_name: value.app_name,
            description: value.description,
            docker_image: value.docker_image,
            database_name: value.database_name,
            database_user: value.database_user,
            database_password: value.database_password,
            external_port: value.external_port,
            command: value.command,
            args: parse_json_string_vec(value.args.as_deref().unwrap_or("[]")),
            env_var: value.env_var,
            memory_reservation: value.memory_reservation,
            memory_limit: value.memory_limit,
            cpu_reservation: value.cpu_reservation,
            cpu_limit: value.cpu_limit,
            replicas: value.replicas,
            network_ids: parse_json_string_vec(&value.network_ids),
            detach_rustploy_network: value.detach_rustploy_network,
            status: value.app_status.clone(),
            app_status: value.app_status,
            last_error: value.last_error,
            environment_id: value.environment_id,
            server_id: value.server_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub(crate) fn serialize_json_string_vec(
    value: Option<&Vec<String>>,
) -> sqlx::Result<Option<String>> {
    value
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))
}

pub(crate) fn parse_json_string_vec(value: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(value).unwrap_or_default()
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DatabaseOperationResponseDto {
    pub database: DatabaseResponseDto,
    pub operation: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct DatabaseConnectionDto {
    pub kind: crate::services::database::DatabaseKind,
    pub host: String,
    pub port: i64,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: String,
    pub internal_url: String,
    pub external_url: Option<String>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct RotateDatabaseCredentialsDto {
    #[validate(length(min = 16, max = 512))]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DatabaseCredentialRotationDto {
    pub password: String,
    pub redeploy_required: bool,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DatabaseValidationDto {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PostgresAdvancedConfigDto {
    pub settings: BTreeMap<String, String>,
    pub replication_role: PostgresReplicationRole,
    pub primary_host: Option<String>,
    pub primary_port: Option<i64>,
    pub replication_user: Option<String>,
    #[validate(length(min = 16, max = 512))]
    pub replication_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct PostgresAdvancedConfigResponseDto {
    pub settings: BTreeMap<String, String>,
    pub replication_role: PostgresReplicationRole,
    pub primary_host: Option<String>,
    pub primary_port: Option<i64>,
    pub replication_user: Option<String>,
    pub replication_password_configured: bool,
    pub redeploy_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseExportBundleDto {
    pub schema_version: u32,
    pub exported_at: i64,
    pub secrets_included: bool,
    pub kind: crate::services::database::DatabaseKind,
    pub name: String,
    pub description: Option<String>,
    pub docker_image: String,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub external_port: Option<i64>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct DatabaseExportQueryDto {
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DatabaseArchiveDto {
    pub format: String,
    pub schema_version: i64,
    pub archive: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct ImportDatabaseDto {
    #[validate(length(min = 2))]
    pub archive: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
}

impl From<DatabaseOperationResult> for DatabaseOperationResponseDto {
    fn from(value: DatabaseOperationResult) -> Self {
        Self {
            database: DatabaseResponseDto::from(value.database),
            operation: value.operation.as_str().into(),
        }
    }
}
