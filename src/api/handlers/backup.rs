use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use serde::Deserialize;

use crate::{
    api::dto::backup::{
        BackupResponseDto, CreateBackupDto, CreateVolumeBackupDto, PatchBackupDto,
        PatchVolumeBackupDto, VolumeBackupResponseDto,
    },
    core::middleware::{
        permission::{
            AppDeployPermission, DatabaseCreatePermission, DatabaseDeletePermission,
            DatabaseReadPermission, RequirePermission,
        },
        validator::ValidatedJson,
    },
    db::models::{backups::Backup, volume_backups::VolumeBackup},
    repository::{backups::BackupRepository, volume_backups::VolumeBackupRepository},
    services::schedule::ScheduleService,
};

type ApiError = (StatusCode, String);

pub struct BackupController {
    service: Arc<ScheduleService>,
    repo_backup: Arc<BackupRepository>,
    repo_volume: Arc<VolumeBackupRepository>,
}

#[derive(Deserialize, poem_openapi::Object)]
pub struct RestoreBackupDto {
    pub backup_file: String,
}

#[controller("/backups")]
impl BackupController {
    fn new(
        service: Arc<ScheduleService>,
        repo_backup: Arc<BackupRepository>,
        repo_volume: Arc<VolumeBackupRepository>,
    ) -> Self {
        Self {
            service,
            repo_backup,
            repo_volume,
        }
    }

    #[get("/database")]
    async fn list_database_backups(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
    ) -> Result<Json<Vec<BackupResponseDto>>, ApiError> {
        self.repo_backup
            .get_all()
            .await
            .map(|items| items.into_iter().map(BackupResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/database/{id}")]
    async fn get_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<BackupResponseDto>, ApiError> {
        self.repo_backup
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(BackupResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/database")]
    async fn create_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(dto): ValidatedJson<CreateBackupDto>,
    ) -> Result<(StatusCode, Json<BackupResponseDto>), ApiError> {
        let item = Backup {
            id: None,
            app_name: dto.app_name,
            schedule: dto.schedule,
            enabled: 1,
            database_name: dto.database_name,
            prefix: dto.prefix,
            service_name: dto.service_name,
            keep_latest_count: dto.keep_latest_count,
            backup_type: dto.backup_type,
            database_type: dto.database_type,
            metadata: dto.metadata,
            compose_id: dto.compose_id,
            postgres_id: dto.postgres_id,
            mysql_id: dto.mysql_id,
            mariadb_id: dto.mariadb_id,
            mongo_id: dto.mongo_id,
            redis_id: dto.redis_id,
            libsql_id: dto.libsql_id,
            destination_id: dto.destination_id,
            organization_id: dto.organization_id,
            created_at: 0,
            updated_at: 0,
        };
        let id = self
            .repo_backup
            .create(&item)
            .await
            .map_err(map_sqlx_error)?;
        self.repo_backup
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(BackupResponseDto::from)
            .map(|b| (StatusCode::CREATED, Json(b)))
            .map_err(map_sqlx_error)
    }

    #[patch("/database/{id}")]
    async fn patch_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(dto): ValidatedJson<PatchBackupDto>,
    ) -> Result<Json<BackupResponseDto>, ApiError> {
        let mut item = self
            .repo_backup
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map_err(map_sqlx_error)?;
        if let Some(v) = dto.app_name {
            item.app_name = v;
        }
        if let Some(v) = dto.schedule {
            item.schedule = v;
        }
        if let Some(v) = dto.database_name {
            item.database_name = v;
        }
        if let Some(v) = dto.prefix {
            item.prefix = v;
        }
        if let Some(v) = dto.service_name {
            item.service_name = Some(v);
        }
        if let Some(v) = dto.keep_latest_count {
            item.keep_latest_count = Some(v);
        }
        if let Some(v) = dto.backup_type {
            item.backup_type = v;
        }
        if let Some(v) = dto.database_type {
            item.database_type = v;
        }
        if let Some(v) = dto.metadata {
            item.metadata = Some(v);
        }
        if let Some(v) = dto.destination_id {
            item.destination_id = v;
        }
        if let Some(v) = dto.enabled {
            item.enabled = v;
        }

        self.repo_backup
            .update(id, &item)
            .await
            .map_err(map_sqlx_error)?;

        self.repo_backup
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(BackupResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/database/{id}")]
    async fn delete_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.repo_backup.delete(id).await.map_err(map_sqlx_error)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/database/{id}/run")]
    async fn run_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .run_database_backup(id)
            .await
            .map(|_| StatusCode::ACCEPTED)
            .map_err(map_sqlx_error)
    }

    #[get("/volume")]
    async fn list_volume_backups(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
    ) -> Result<Json<Vec<VolumeBackupResponseDto>>, ApiError> {
        self.repo_volume
            .get_all()
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(VolumeBackupResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/volume/{id}")]
    async fn get_volume_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<VolumeBackupResponseDto>, ApiError> {
        self.repo_volume
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(VolumeBackupResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/volume")]
    async fn create_volume_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(dto): ValidatedJson<CreateVolumeBackupDto>,
    ) -> Result<(StatusCode, Json<VolumeBackupResponseDto>), ApiError> {
        let item = VolumeBackup {
            id: None,
            name: dto.name,
            volume_name: dto.volume_name,
            prefix: dto.prefix,
            service_type: dto.service_type,
            app_name: dto.app_name,
            service_name: dto.service_name,
            turn_off: dto.turn_off,
            cron_expression: dto.cron_expression,
            keep_latest_count: dto.keep_latest_count,
            enabled: 1,
            destination_id: dto.destination_id,
            organization_id: dto.organization_id,
            application_id: dto.application_id,
            postgres_id: dto.postgres_id,
            mysql_id: dto.mysql_id,
            mariadb_id: dto.mariadb_id,
            mongo_id: dto.mongo_id,
            redis_id: dto.redis_id,
            libsql_id: dto.libsql_id,
            compose_id: dto.compose_id,
            created_at: 0,
            updated_at: 0,
        };
        let id = self
            .repo_volume
            .create(&item)
            .await
            .map_err(map_sqlx_error)?;
        self.repo_volume
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(VolumeBackupResponseDto::from)
            .map(|b| (StatusCode::CREATED, Json(b)))
            .map_err(map_sqlx_error)
    }

    #[patch("/volume/{id}")]
    async fn patch_volume_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(dto): ValidatedJson<PatchVolumeBackupDto>,
    ) -> Result<Json<VolumeBackupResponseDto>, ApiError> {
        let mut item = self
            .repo_volume
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map_err(map_sqlx_error)?;
        if let Some(v) = dto.name {
            item.name = v;
        }
        if let Some(v) = dto.volume_name {
            item.volume_name = v;
        }
        if let Some(v) = dto.prefix {
            item.prefix = v;
        }
        if let Some(v) = dto.service_type {
            item.service_type = v;
        }
        if let Some(v) = dto.app_name {
            item.app_name = v;
        }
        if let Some(v) = dto.service_name {
            item.service_name = Some(v);
        }
        if let Some(v) = dto.turn_off {
            item.turn_off = v;
        }
        if let Some(v) = dto.cron_expression {
            item.cron_expression = v;
        }
        if let Some(v) = dto.keep_latest_count {
            item.keep_latest_count = Some(v);
        }
        if let Some(v) = dto.destination_id {
            item.destination_id = v;
        }
        if let Some(v) = dto.enabled {
            item.enabled = v;
        }

        self.repo_volume
            .update(id, &item)
            .await
            .map_err(map_sqlx_error)?;

        self.repo_volume
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(VolumeBackupResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/volume/{id}")]
    async fn delete_volume_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.repo_volume.delete(id).await.map_err(map_sqlx_error)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/volume/{id}/run")]
    async fn run_volume_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .run_volume_backup(id)
            .await
            .map(|_| StatusCode::ACCEPTED)
            .map_err(map_sqlx_error)
    }

    #[post("/database/{id}/restore")]
    async fn restore_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
        Json(body): Json<RestoreBackupDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .restore_database_backup(id, &body.backup_file)
            .await
            .map(|_| StatusCode::ACCEPTED)
            .map_err(map_sqlx_error)
    }

    #[post("/volume/{id}/restore")]
    async fn restore_volume_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
        Json(body): Json<RestoreBackupDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .restore_volume_backup(id, &body.backup_file)
            .await
            .map(|_| StatusCode::ACCEPTED)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "backup not found".into()),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "backup operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "backup operation failed".into(),
            )
        }
    }
}
