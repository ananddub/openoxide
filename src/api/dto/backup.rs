use crate::db::models::{
    backup_executions::BackupExecution, backups::Backup, volume_backups::VolumeBackup,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct PanelBackupResponseDto {
    pub path: String,
    pub checksum_sha256: String,
    pub size_bytes: i64,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RestorePanelBackupDto {
    pub archive: String,
    pub checksum_sha256: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RollbackPanelRestoreDto {
    pub recovery_database: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct StagePanelRestoreDto {
    pub restore_id: String,
    pub checksum_sha256: String,
    pub restart_required: bool,
    pub pending_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct PanelRestoreStatusDto {
    pub restore_id: String,
    pub status: String,
    pub message: String,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct BackupFilesQueryDto {
    pub destination_id: i64,
    pub prefix: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct DownloadBackupFileDto {
    pub destination_id: i64,
    pub object_key: String,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RetentionPreviewQueryDto {
    pub destination_id: i64,
    pub prefix: Option<String>,
    pub keep_latest: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct BackupFileDto {
    pub name: String,
    pub path: String,
    pub size_bytes: i64,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RetentionPreviewDto {
    pub keep: Vec<BackupFileDto>,
    pub delete: Vec<BackupFileDto>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct ComposeConfigBackupDto {
    pub include_secrets: Option<bool>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct VerifyBackupFileDto {
    pub destination_id: i64,
    pub object_key: String,
    pub checksum_sha256: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct BackupIntegrityDto {
    pub valid: bool,
    pub expected_sha256: String,
    pub actual_sha256: String,
    pub size_bytes: i64,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct BackupExecutionQueryDto {
    pub backup_kind: Option<crate::services::backup::types::BackupKind>,
    pub backup_id: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct BackupExecutionResponseDto {
    pub id: i64,
    pub backup_kind: crate::services::backup::types::BackupKind,
    pub operation: crate::services::backup::types::BackupOperation,
    pub backup_id: Option<i64>,
    pub status: crate::services::backup::types::BackupExecutionStatus,
    pub object_key: Option<String>,
    pub checksum_sha256: Option<String>,
    pub size_bytes: Option<i64>,
    pub attempt: i64,
    pub error: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}

impl From<BackupExecution> for BackupExecutionResponseDto {
    fn from(value: BackupExecution) -> Self {
        Self {
            id: value.id,
            backup_kind: crate::services::backup::types::BackupKind::try_from(
                value.backup_kind.as_str(),
            )
            .expect("database enforces backup kind"),
            operation: crate::services::backup::types::BackupOperation::try_from(
                value.operation.as_str(),
            )
            .expect("database enforces backup operation"),
            backup_id: value.backup_id,
            status: crate::services::backup::types::BackupExecutionStatus::try_from(
                value.status.as_str(),
            )
            .expect("database enforces backup status"),
            object_key: value.object_key,
            checksum_sha256: value.checksum_sha256,
            size_bytes: value.size_bytes,
            attempt: value.attempt,
            error: value.error,
            started_at: value.started_at,
            finished_at: value.finished_at,
        }
    }
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
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
    pub compose_id: Option<i64>,
    pub postgres_id: Option<i64>,
    pub mysql_id: Option<i64>,
    pub mariadb_id: Option<i64>,
    pub mongo_id: Option<i64>,
    pub redis_id: Option<i64>,
    pub libsql_id: Option<i64>,
    pub destination_id: i64,
    pub organization_id: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
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
    pub database_type: String,
    pub metadata: Option<String>,
    pub compose_id: Option<i64>,
    pub postgres_id: Option<i64>,
    pub mysql_id: Option<i64>,
    pub mariadb_id: Option<i64>,
    pub mongo_id: Option<i64>,
    pub redis_id: Option<i64>,
    pub libsql_id: Option<i64>,
    pub destination_id: i64,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Backup> for BackupResponseDto {
    fn from(value: Backup) -> Self {
        Self {
            id: value.id.unwrap_or(0),
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
            compose_id: value.compose_id,
            postgres_id: value.postgres_id,
            mysql_id: value.mysql_id,
            mariadb_id: value.mariadb_id,
            mongo_id: value.mongo_id,
            redis_id: value.redis_id,
            libsql_id: value.libsql_id,
            destination_id: value.destination_id,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
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
    pub application_id: Option<i64>,
    pub postgres_id: Option<i64>,
    pub mysql_id: Option<i64>,
    pub mariadb_id: Option<i64>,
    pub mongo_id: Option<i64>,
    pub redis_id: Option<i64>,
    pub libsql_id: Option<i64>,
    pub compose_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
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
    pub application_id: Option<i64>,
    pub postgres_id: Option<i64>,
    pub mysql_id: Option<i64>,
    pub mariadb_id: Option<i64>,
    pub mongo_id: Option<i64>,
    pub redis_id: Option<i64>,
    pub libsql_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<VolumeBackup> for VolumeBackupResponseDto {
    fn from(value: VolumeBackup) -> Self {
        Self {
            id: value.id.unwrap_or(0),
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
            application_id: value.application_id,
            postgres_id: value.postgres_id,
            mysql_id: value.mysql_id,
            mariadb_id: value.mariadb_id,
            mongo_id: value.mongo_id,
            redis_id: value.redis_id,
            libsql_id: value.libsql_id,
            compose_id: value.compose_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
