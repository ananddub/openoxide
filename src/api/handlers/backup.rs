use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use serde::Deserialize;

use crate::{
    api::dto::backup::{
        BackupResponseDto, CreateBackupDto, CreateVolumeBackupDto, PatchBackupDto,
        PatchVolumeBackupDto, VolumeBackupResponseDto,
    },
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    core::middleware::{
        permission::{
            AppDeployPermission, DatabaseCreatePermission, DatabaseDeletePermission,
            DatabaseReadPermission, RequirePermission,
        },
        validator::ValidatedJson,
    },
    db::models::{backups::Backup, destinations::Destination, volume_backups::VolumeBackup},
    repository::{
        backups::BackupRepository, destinations::DestinationRepository,
        organization::OrganizationRepository, volume_backups::VolumeBackupRepository,
    },
    services::schedule::ScheduleService,
};

type ApiError = (StatusCode, String);

pub struct BackupController {
    db: Arc<sqlx::SqlitePool>,
    service: Arc<ScheduleService>,
    repo_backup: Arc<BackupRepository>,
    repo_volume: Arc<VolumeBackupRepository>,
    repo_dest: Arc<DestinationRepository>,
    repo_org: Arc<OrganizationRepository>,
    cache: Arc<AppStateCache>,
}

#[derive(Deserialize, poem_openapi::Object)]
pub struct RestoreBackupDto {
    pub backup_file: String,
}

#[controller("/backups")]
impl BackupController {
    fn new(
        db: Arc<sqlx::SqlitePool>,
        service: Arc<ScheduleService>,
        repo_backup: Arc<BackupRepository>,
        repo_volume: Arc<VolumeBackupRepository>,
        repo_dest: Arc<DestinationRepository>,
        repo_org: Arc<OrganizationRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            db,
            service,
            repo_backup,
            repo_volume,
            repo_dest,
            repo_org,
            cache,
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

    async fn resolve_organization_id(&self, requested_id: Option<i64>) -> i64 {
        if let Some(id) = requested_id {
            if id > 0 {
                if let Ok(Some(_)) = self.repo_org.get_by_id(id).await {
                    return id;
                }
            }
        }

        if let Ok(orgs) = self.repo_org.get_all().await {
            if let Some(first) = orgs.first() {
                if let Some(id) = first.id {
                    return id;
                }
            }
        }

        1
    }

    async fn resolve_destination_id(&self, requested_id: Option<i64>) -> i64 {
        if let Some(id) = requested_id {
            if id > 0 {
                if let Ok(Some(_)) = self.repo_dest.get_by_id(id).await {
                    return id;
                }
            }
        }

        if let Ok(dests) = self.repo_dest.get_all().await {
            if let Some(first) = dests.first() {
                if let Some(id_str) = &first.id {
                    if let Ok(parsed) = id_str.parse::<i64>() {
                        return parsed;
                    }
                }
            }
        }

        let org_id = self.resolve_organization_id(None).await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let default_dest = Destination {
            id: None,
            name: "Local Storage".to_string(),
            provider: "LOCAL".to_string(),
            access_key: "".to_string(),
            secret_access_key: "".to_string(),
            bucket: "backups".to_string(),
            region: "local".to_string(),
            endpoint: "".to_string(),
            additional_flags: None,
            organization_id: org_id,
            created_at: now,
            updated_at: now,
        };

        if let Ok(new_id) = self.repo_dest.create(&default_dest).await {
            return new_id;
        }

        1
    }

    async fn verify_fk_exists(&self, table: &str, id: Option<i64>) -> Option<i64> {
        let target_id = match id {
            Some(v) if v > 0 => v,
            _ => return None,
        };

        let res: Result<Option<i64>, _> = match table {
            "applications" => {
                sqlx::query_scalar!(r#"SELECT id FROM applications WHERE id = ?"#, target_id)
                    .fetch_optional(self.db.as_ref())
                    .await
            }
            "postgres" => {
                sqlx::query_scalar!(
                    r#"SELECT CAST(id AS INTEGER) AS "id!: i64" FROM postgres_dbs WHERE id = ?"#,
                    target_id
                )
                .fetch_optional(self.db.as_ref())
                .await
            }
            "mysql" => {
                sqlx::query_scalar!(
                    r#"SELECT CAST(id AS INTEGER) AS "id!: i64" FROM mysql_dbs WHERE id = ?"#,
                    target_id
                )
                .fetch_optional(self.db.as_ref())
                .await
            }
            "mariadb" => {
                sqlx::query_scalar!(
                    r#"SELECT CAST(id AS INTEGER) AS "id!: i64" FROM mariadb_dbs WHERE id = ?"#,
                    target_id
                )
                .fetch_optional(self.db.as_ref())
                .await
            }
            "mongo" => {
                sqlx::query_scalar!(
                    r#"SELECT CAST(id AS INTEGER) AS "id!: i64" FROM mongo_dbs WHERE id = ?"#,
                    target_id
                )
                .fetch_optional(self.db.as_ref())
                .await
            }
            "redis" => {
                sqlx::query_scalar!(
                    r#"SELECT CAST(id AS INTEGER) AS "id!: i64" FROM redis_dbs WHERE id = ?"#,
                    target_id
                )
                .fetch_optional(self.db.as_ref())
                .await
            }
            "libsql" => {
                sqlx::query_scalar!(
                    r#"SELECT CAST(id AS INTEGER) AS "id!: i64" FROM libsql_dbs WHERE id = ?"#,
                    target_id
                )
                .fetch_optional(self.db.as_ref())
                .await
            }
            "compose" => {
                sqlx::query_scalar!(r#"SELECT id FROM compose_projects WHERE id = ?"#, target_id)
                    .fetch_optional(self.db.as_ref())
                    .await
            }
            _ => Ok(None),
        };

        match res {
            Ok(Some(_)) => Some(target_id),
            _ => None,
        }
    }

    #[post("/database")]
    async fn create_database_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(dto): ValidatedJson<CreateBackupDto>,
    ) -> Result<(StatusCode, Json<BackupResponseDto>), ApiError> {
        let org_id = self
            .resolve_organization_id(Some(dto.organization_id))
            .await;
        let dest_id = self.resolve_destination_id(Some(dto.destination_id)).await;
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
            compose_id: self.verify_fk_exists("compose", dto.compose_id).await,
            postgres_id: self.verify_fk_exists("postgres", dto.postgres_id).await,
            mysql_id: self.verify_fk_exists("mysql", dto.mysql_id).await,
            mariadb_id: self.verify_fk_exists("mariadb", dto.mariadb_id).await,
            mongo_id: self.verify_fk_exists("mongo", dto.mongo_id).await,
            redis_id: self.verify_fk_exists("redis", dto.redis_id).await,
            libsql_id: self.verify_fk_exists("libsql", dto.libsql_id).await,
            destination_id: dest_id,
            organization_id: org_id,
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
        if let Some(CacheEnum::VolumeBackups(cached_items)) =
            self.cache.get(&CacheKey::VolumeBackups).await
        {
            return Ok(Json(
                cached_items
                    .into_iter()
                    .map(VolumeBackupResponseDto::from)
                    .collect(),
            ));
        }

        let items = self.repo_volume.get_all().await.map_err(map_sqlx_error)?;
        self.cache
            .insert(
                CacheKey::VolumeBackups,
                CacheEnum::VolumeBackups(items.clone()),
            )
            .await;

        Ok(Json(
            items
                .into_iter()
                .map(VolumeBackupResponseDto::from)
                .collect(),
        ))
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
        let org_id = self.resolve_organization_id(dto.organization_id).await;
        let dest_id = self.resolve_destination_id(dto.destination_id).await;

        let mut item = VolumeBackup {
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
            destination_id: dest_id,
            organization_id: org_id,
            application_id: self
                .verify_fk_exists("applications", dto.application_id)
                .await,
            postgres_id: self.verify_fk_exists("postgres", dto.postgres_id).await,
            mysql_id: self.verify_fk_exists("mysql", dto.mysql_id).await,
            mariadb_id: self.verify_fk_exists("mariadb", dto.mariadb_id).await,
            mongo_id: self.verify_fk_exists("mongo", dto.mongo_id).await,
            redis_id: self.verify_fk_exists("redis", dto.redis_id).await,
            libsql_id: self.verify_fk_exists("libsql", dto.libsql_id).await,
            compose_id: self.verify_fk_exists("compose", dto.compose_id).await,
            created_at: 0,
            updated_at: 0,
        };

        if item.compose_id.is_some() {
            item.service_type = "COMPOSE".to_string();
        } else if item.application_id.is_some() {
            item.service_type = "APPLICATION".to_string();
        } else if item.postgres_id.is_some() {
            item.service_type = "POSTGRES".to_string();
        } else if item.mysql_id.is_some() {
            item.service_type = "MYSQL".to_string();
        } else if item.mariadb_id.is_some() {
            item.service_type = "MARIADB".to_string();
        } else if item.mongo_id.is_some() {
            item.service_type = "MONGO".to_string();
        } else if item.redis_id.is_some() {
            item.service_type = "REDIS".to_string();
        } else if item.libsql_id.is_some() {
            item.service_type = "LIBSQL".to_string();
        }
        let id = self
            .repo_volume
            .create(&item)
            .await
            .map_err(map_sqlx_error)?;
        let created = self
            .repo_volume
            .get_by_id(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map(VolumeBackupResponseDto::from)
            .map(|b| (StatusCode::CREATED, Json(b)))
            .map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::VolumeBackups).await;
        Ok(created)
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

        self.cache.invalidate(&CacheKey::VolumeBackups).await;

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
        self.cache.invalidate(&CacheKey::VolumeBackups).await;
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
