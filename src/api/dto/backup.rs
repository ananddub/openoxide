use crate::db::models::{backups::Backup, volume_backups::VolumeBackup};
use serde::{Deserialize, Serialize};
use validator::Validate;

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
