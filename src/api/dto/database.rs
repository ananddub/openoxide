use serde::{Deserialize, Serialize};
use validator::Validate;

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

impl From<DatabaseOperationResult> for DatabaseOperationResponseDto {
    fn from(value: DatabaseOperationResult) -> Self {
        Self {
            database: DatabaseResponseDto::from(value.database),
            operation: value.operation.as_str().into(),
        }
    }
}
