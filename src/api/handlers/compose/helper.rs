use connectrpc::{ConnectError, ErrorCode};
use serde::Serialize;

use crate::api::connect::generated::openoxide::api::v1::{Compose, ResourceResponse};
use crate::flash_gen::models::{Mounts, Patches};
use crate::services::compose::crud::ComposeResponseDto;

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("compose project not found"),
        sqlx::Error::Protocol(msg) if msg.contains("already running") => {
            ConnectError::new(ErrorCode::AlreadyExists, msg.clone())
        }
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("related resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_check_violation() => {
            ConnectError::invalid_argument(e.message())
        }
        other => {
            tracing::error!(error = %other, "compose database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn compose_to_proto(dto: &ComposeResponseDto) -> Compose {
    Compose {
        id: dto.id,
        name: dto.name.clone(),
        description: dto.description.clone(),
        environment_id: dto.environment_id,
        server_id: dto.server_id,
        status: format!("{}:{}", dto.compose_status, dto.compose_type),
        source_type: Some(dto.source_type.clone()),
        repository: dto.repository.clone(),
        branch: dto.branch.clone(),
        compose_file: Some(dto.compose_file.clone()),
        created_at: dto.created_at,
        updated_at: dto.updated_at,
        ..Default::default()
    }
}

pub fn mount_to_resource(dto: ComposeMountResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse { id: dto.id, kind: "compose_mount".into(), payload, ..Default::default() }
}

pub fn patch_to_resource(dto: ComposePatchResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse { id: dto.id, kind: "compose_patch".into(), payload, ..Default::default() }
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Serialize)]
pub struct ComposeMountResponseDto {
    pub id: i64,
    pub mount_type: String,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub mount_path: String,
    pub compose_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Mounts> for ComposeMountResponseDto {
    fn from(value: Mounts) -> Self {
        // Flash bind params for nullable columns are non-Option; "" is the
        // NULL encoding and is filtered back to None here.
        let normalize = |value: Option<String>| match value {
            Some(text) if text.is_empty() => None,
            other => other,
        };
        Self {
            id: value.id,
            mount_type: value.mount_type,
            host_path: normalize(value.host_path),
            volume_name: normalize(value.volume_name),
            file_path: normalize(value.file_path),
            content: normalize(value.content),
            mount_path: value.mount_path,
            compose_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ComposePatchResponseDto {
    pub id: i64,
    pub patch_type: String,
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
    pub compose_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Patches> for ComposePatchResponseDto {
    fn from(value: Patches) -> Self {
        Self {
            id: value.id,
            patch_type: value.patch_type,
            file_path: value.file_path,
            enabled: value.enabled != 0,
            content: value.content,
            compose_id: value.service_id.unwrap_or_default(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
