use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use validator::Validate;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PostgresReplicationRole {
    Standalone,
    Primary,
    Replica,
}
impl PostgresReplicationRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standalone => "STANDALONE",
            Self::Primary => "PRIMARY",
            Self::Replica => "REPLICA",
        }
    }
}
impl TryFrom<&str> for PostgresReplicationRole {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_ascii_uppercase().as_str() {
            "STANDALONE" => Ok(Self::Standalone),
            "PRIMARY" => Ok(Self::Primary),
            "REPLICA" => Ok(Self::Replica),
            other => Err(format!("invalid PostgreSQL replication role: {other}")),
        }
    }
}

use crate::services::database::{DatabaseOperationResult, DatabaseRecord};

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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DatabaseResponseDto {
    pub kind: crate::services::database::DatabaseKind,
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub docker_image: String,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub external_port: Option<i64>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
    pub app_status: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
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
            external_port: value.external_port,
            env_var: value.env_var,
            memory_reservation: value.memory_reservation,
            memory_limit: value.memory_limit,
            cpu_reservation: value.cpu_reservation,
            cpu_limit: value.cpu_limit,
            replicas: value.replicas,
            network_ids: parse_json_string_vec(&value.network_ids),
            detach_rustploy_network: value.detach_rustploy_network,
            app_status: value.app_status,
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
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
