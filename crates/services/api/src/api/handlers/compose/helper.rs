use connectrpc::{ConnectError, ErrorCode};
use serde::Serialize;

use crate::api::connect::generated::openoxide::api::v1::{
    Compose, ComposeMount, ComposeMountType, ComposePatch, ComposePatchType,
};
use crate::flash_gen::models::{Mounts, Patches};
use crate::services::compute::compose::ComposeService;
use crate::services::compute::compose::crud::ComposeResponseDto;

/// Compose projects (and their mounts/patches) use global service ids, so
/// every RPC must confirm the row belongs to the caller's organization.
/// Mismatches report not-found to avoid leaking another tenant's ids.
pub async fn ensure_compose_org(compose_id: i64, organization_id: i64) -> Result<(), ConnectError> {
    let composes = auto_di::resolve::<ComposeService>()
        .await
        .map_err(|error| {
            tracing::error!(%error, "compose service unavailable");
            ConnectError::new(ErrorCode::Internal, "compose service unavailable")
        })?;
    composes
        .ensure_organization_access(compose_id, organization_id)
        .await
        .map_err(map_error)
}

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
        // Plain lifecycle status: the wire has no compose_type field, and
        // every frontend consumer (header badge, deploy card, project list)
        // compares status against bare values like RUNNING/STOPPED — the old
        // "STATUS:TYPE" composite matched nothing and pinned the UI to STOPPED.
        status: dto.compose_status.clone(),
        source_type: Some(dto.source_type.clone()),
        repository: dto.repository.clone(),
        branch: dto.branch.clone(),
        compose_file: Some(dto.compose_file.clone()),
        created_at: dto.created_at,
        updated_at: dto.updated_at,
        owner: dto.owner.clone(),
        gitlab_repository: dto.gitlab_repository.clone(),
        gitlab_owner: dto.gitlab_owner.clone(),
        gitlab_branch: dto.gitlab_branch.clone(),
        gitea_repository: dto.gitea_repository.clone(),
        gitea_owner: dto.gitea_owner.clone(),
        gitea_branch: dto.gitea_branch.clone(),
        bitbucket_repository: dto.bitbucket_repository.clone(),
        bitbucket_owner: dto.bitbucket_owner.clone(),
        bitbucket_branch: dto.bitbucket_branch.clone(),
        custom_git_url: dto.custom_git_url.clone(),
        custom_git_branch: dto.custom_git_branch.clone(),
        build_path: dto.build_path.clone(),
        custom_git_ssh_key_id: dto.custom_git_ssh_key_id,
        command: Some(dto.command.clone()),
        compose_path: Some(dto.compose_path.clone()),
        compose_type: Some(dto.compose_type.clone()),
        last_error: dto.last_error.clone(),
        randomize: dto.randomize,
        suffix: dto.suffix.clone(),
        // Surface the resolved environment (including template-generated
        // secrets) so the GUI env editor can display and edit it instead of
        // showing a blank box.
        env_var: dto.env_var.clone(),
    }
}

// ── Enum coercion helpers ───────────────────────────────────────────

/// proto mount type → the SCREAMING spec string the service layer parses
/// (`crate::utils::builder::spec::MountType::from_str`).
pub fn compose_mount_type_to_spec_str(value: buffa::EnumValue<ComposeMountType>) -> String {
    let s = match value {
        buffa::EnumValue::Known(ComposeMountType::Bind) => "BIND",
        buffa::EnumValue::Known(ComposeMountType::File) => "FILE",
        _ => "VOLUME",
    };
    s.to_string()
}

fn compose_mount_type_str_to_proto(s: &str) -> buffa::EnumValue<ComposeMountType> {
    (match s {
        "BIND" => ComposeMountType::Bind,
        "FILE" => ComposeMountType::File,
        _ => ComposeMountType::Volume,
    })
    .into()
}

fn compose_patch_type_str_to_proto(s: &str) -> buffa::EnumValue<ComposePatchType> {
    (match s {
        "DELETE" => ComposePatchType::Delete,
        _ => ComposePatchType::Update,
    })
    .into()
}

pub fn mount_to_proto(dto: ComposeMountResponseDto) -> ComposeMount {
    ComposeMount {
        id: dto.id,
        mount_type: compose_mount_type_str_to_proto(&dto.mount_type),
        host_path: dto.host_path,
        volume_name: dto.volume_name,
        file_path: dto.file_path,
        content: dto.content,
        mount_path: dto.mount_path,
        compose_id: dto.compose_id,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
    }
}

pub fn patch_to_proto(dto: ComposePatchResponseDto) -> ComposePatch {
    ComposePatch {
        id: dto.id,
        patch_type: compose_patch_type_str_to_proto(&dto.patch_type),
        file_path: dto.file_path,
        enabled: dto.enabled,
        content: dto.content,
        compose_id: dto.compose_id,
        created_at: dto.created_at,
        updated_at: dto.updated_at,
    }
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
