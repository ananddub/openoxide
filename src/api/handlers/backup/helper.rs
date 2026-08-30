use connectrpc::{ConnectError, ErrorCode};
use serde::Deserialize;

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::{Backups, BackupExecutions, VolumeBackups};

pub async fn refresh_schedule_runner() {
    if let Ok(runner) = auto_di::resolve::<crate::services::automation::schedule::ScheduleRunner>().await {
        if let Err(error) = runner.refresh_jobs().await {
            tracing::error!(%error, "could not refresh scheduler after backup mutation");
        }
    }
}

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("backup not found"),
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "backup operation failed");
            ConnectError::new(ErrorCode::Internal, "backup operation failed")
        }
    }
}

pub fn db_backup_to_resource(dto: BackupResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "database_backup".into(),
        payload,
        ..Default::default()
    }
}

pub fn volume_backup_to_resource(dto: VolumeBackupResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "volume_backup".into(),
        payload,
        ..Default::default()
    }
}

pub fn execution_to_resource(dto: BackupExecutionResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "backup_execution".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, Deserialize)]
pub struct ComposeConfigBackupDto {
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RestorePanelBackupDto {
    pub archive: String,
    pub checksum_sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RollbackPanelRestoreDto {
    pub recovery_database: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadBackupFileDto {
    pub destination_id: i64,
    pub object_key: String,
}

#[derive(Debug, Deserialize)]
pub struct RetentionPreviewQueryDto {
    pub destination_id: i64,
    pub prefix: Option<String>,
    pub keep_latest: i64,
}

#[derive(Debug, Deserialize)]
pub struct VerifyBackupFileDto {
    pub destination_id: i64,
    pub object_key: String,
    pub checksum_sha256: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackupExecutionResponseDto {
    pub id: i64,
    pub backup_kind: crate::services::automation::backup::types::BackupKind,
    pub operation: crate::services::automation::backup::types::BackupOperation,
    pub backup_id: Option<i64>,
    pub status: crate::services::automation::backup::types::BackupExecutionStatus,
    pub object_key: Option<String>,
    pub checksum_sha256: Option<String>,
    pub size_bytes: Option<i64>,
    pub attempt: i64,
    pub error: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}

impl From<BackupExecutions> for BackupExecutionResponseDto {
    fn from(value: BackupExecutions) -> Self {
        Self {
            id: value.id,
            backup_kind: crate::services::automation::backup::types::BackupKind::from_str(
                value.backup_kind.as_str(),
            )
            .expect("database enforces backup kind"),
            operation: crate::services::automation::backup::types::BackupOperation::from_str(
                value.operation.as_str(),
            )
            .expect("database enforces backup operation"),
            backup_id: value.backup_id,
            status: crate::services::automation::backup::types::BackupExecutionStatus::from_str(
                value.status.as_str(),
            )
            .expect("database enforces backup status"),
            object_key: value.object_key,
            checksum_sha256: value.sha256_checksum,
            size_bytes: value.size_bytes,
            attempt: value.attempt,
            error: value.error,
            started_at: value.started_at,
            finished_at: value.finished_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateBackupDto {
    pub app_name: String,
    pub schedule: String,
    pub database_name: String,
    pub prefix: String,
    pub service_name: Option<String>,
    pub keep_latest_count: Option<i64>,
    pub backup_type: String,
    pub database_type: String,
    pub metadata: Option<String>,
    pub service_id: Option<i64>,
    pub destination_id: i64,
    pub organization_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PatchBackupDto {
    pub app_name: Option<String>,
    pub schedule: Option<String>,
    pub database_name: Option<String>,
    pub prefix: Option<String>,
    pub service_name: Option<String>,
    pub keep_latest_count: Option<i64>,
    pub backup_type: Option<String>,
    pub database_type: Option<String>,
    pub metadata: Option<String>,
    pub destination_id: Option<i64>,
    pub enabled: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BackupResponseDto {
    pub id: i64,
    pub app_name: String,
    pub schedule: String,
    pub enabled: i64,
    pub database_name: String,
    pub prefix: String,
    pub service_name: Option<String>,
    pub keep_latest_count: Option<i64>,
    pub backup_type: String,
    pub database_type: Option<String>,
    pub metadata: Option<String>,
    pub service_id: Option<i64>,
    pub destination_id: i64,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Backups> for BackupResponseDto {
    fn from(value: Backups) -> Self {
        Self {
            id: value.id,
            app_name: value.app_name,
            schedule: value.schedule,
            enabled: value.enabled,
            database_name: value.database_name,
            prefix: value.prefix,
            service_name: value.service_name,
            keep_latest_count: value.keep_latest_count,
            backup_type: value.backup_type,
            database_type: value.database_type,
            metadata: value.metadata,
            service_id: value.service_id,
            destination_id: value.destination_id,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateVolumeBackupDto {
    pub name: String,
    pub volume_name: String,
    pub prefix: String,
    pub service_type: String,
    pub app_name: String,
    pub service_name: Option<String>,
    pub turn_off: i64,
    pub cron_expression: String,
    pub keep_latest_count: Option<i64>,
    pub destination_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub service_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchVolumeBackupDto {
    pub name: Option<String>,
    pub volume_name: Option<String>,
    pub prefix: Option<String>,
    pub service_type: Option<String>,
    pub app_name: Option<String>,
    pub service_name: Option<String>,
    pub turn_off: Option<i64>,
    pub cron_expression: Option<String>,
    pub keep_latest_count: Option<i64>,
    pub destination_id: Option<i64>,
    pub enabled: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct VolumeBackupResponseDto {
    pub id: i64,
    pub name: String,
    pub volume_name: String,
    pub prefix: String,
    pub service_type: String,
    pub app_name: String,
    pub service_name: Option<String>,
    pub turn_off: i64,
    pub cron_expression: String,
    pub keep_latest_count: Option<i64>,
    pub enabled: i64,
    pub destination_id: i64,
    pub organization_id: i64,
    pub service_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<VolumeBackups> for VolumeBackupResponseDto {
    fn from(value: VolumeBackups) -> Self {
        Self {
            id: value.id,
            name: value.name,
            volume_name: value.volume_name,
            prefix: value.prefix,
            service_type: value.service_type,
            app_name: value.app_name,
            service_name: value.service_name,
            turn_off: value.turn_off,
            cron_expression: value.cron_expression,
            keep_latest_count: value.keep_latest_count,
            enabled: value.enabled,
            destination_id: value.destination_id,
            organization_id: value.organization_id,
            service_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
