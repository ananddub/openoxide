use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    BackupDatabase, BackupExecution, BackupExecutionStatus as ProtoBackupExecutionStatus,
    BackupKind as ProtoBackupKind, BackupOperation as ProtoBackupOperation, BackupVolume,
};
use crate::flash_gen::models::{BackupExecutions, Backups, VolumeBackups};

pub async fn refresh_schedule_runner() {
    if let Ok(runner) =
        auto_di::resolve::<crate::services::automation::schedule::ScheduleRunner>().await
        && let Err(error) = runner.refresh_jobs().await
    {
        tracing::error!(%error, "could not refresh scheduler after backup mutation");
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

fn kind_to_proto(value: &str) -> ProtoBackupKind {
    match value {
        "DATABASE" => ProtoBackupKind::Database,
        "VOLUME" => ProtoBackupKind::Volume,
        "PANEL" => ProtoBackupKind::Panel,
        "COMPOSE_CONFIG" => ProtoBackupKind::ComposeConfig,
        _ => ProtoBackupKind::Unspecified,
    }
}

fn operation_to_proto(value: &str) -> ProtoBackupOperation {
    match value {
        "BACKUP" => ProtoBackupOperation::Backup,
        "RESTORE" => ProtoBackupOperation::Restore,
        _ => ProtoBackupOperation::Unspecified,
    }
}

fn status_to_proto(value: &str) -> ProtoBackupExecutionStatus {
    match value {
        "RUNNING" => ProtoBackupExecutionStatus::Running,
        "SUCCEEDED" => ProtoBackupExecutionStatus::Succeeded,
        "FAILED" => ProtoBackupExecutionStatus::Failed,
        _ => ProtoBackupExecutionStatus::Unspecified,
    }
}

pub fn db_backup_to_proto(value: Backups) -> BackupDatabase {
    BackupDatabase {
        id: value.id,
        app_name: value.app_name,
        schedule: value.schedule,
        enabled: value.enabled != 0,
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

pub fn volume_backup_to_proto(value: VolumeBackups) -> BackupVolume {
    // service_type is consumed by the literal below; keep a borrow for
    // the typed linkage mirrors.
    let service_type = value.service_type.clone();
    let on = |target: &str| {
        (service_type == target)
            .then_some(value.service_id)
            .flatten()
    };
    BackupVolume {
        id: value.id,
        name: value.name,
        volume_name: value.volume_name,
        prefix: value.prefix,
        service_type: value.service_type,
        app_name: value.app_name,
        service_name: value.service_name,
        turn_off: value.turn_off != 0,
        cron_expression: value.cron_expression,
        keep_latest_count: value.keep_latest_count,
        enabled: value.enabled != 0,
        destination_id: value.destination_id,
        organization_id: value.organization_id,
        service_id: value.service_id,
        application_id: on("APPLICATION").or_else(|| on("standalone")),
        compose_id: on("COMPOSE"),
        postgres_id: on("POSTGRES"),
        mysql_id: on("MYSQL"),
        mariadb_id: on("MARIADB"),
        mongo_id: on("MONGO").or_else(|| on("MONGODB")),
        redis_id: on("REDIS"),
        libsql_id: on("LIBSQL"),
        created_at: value.created_at,
        updated_at: value.updated_at,
    }
}

pub fn execution_to_proto(value: BackupExecutions) -> BackupExecution {
    BackupExecution {
        id: value.id,
        backup_kind: ::buffa::EnumValue::Known(kind_to_proto(&value.backup_kind)),
        operation: ::buffa::EnumValue::Known(operation_to_proto(&value.operation)),
        backup_id: value.backup_id,
        status: ::buffa::EnumValue::Known(status_to_proto(&value.status)),
        object_key: value.object_key,
        checksum_sha256: value.sha256_checksum,
        size_bytes: value.size_bytes,
        attempt: value.attempt,
        error: value.error,
        started_at: value.started_at,
        finished_at: value.finished_at,
    }
}
