use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    body::Body,
    extract::{Path, Query},
    http::{HeaderValue, Response, StatusCode, header},
};
use serde::Deserialize;

use crate::{
    api::dto::backup::{
        BackupExecutionQueryDto, BackupExecutionResponseDto, BackupFilesQueryDto,
        BackupResponseDto, ComposeConfigBackupDto, CreateBackupDto, CreateVolumeBackupDto,
        DownloadBackupFileDto, PanelBackupResponseDto, PatchBackupDto, PatchVolumeBackupDto,
        RestorePanelBackupDto, RetentionPreviewDto, RetentionPreviewQueryDto, StagePanelRestoreDto,
        VerifyBackupFileDto, VolumeBackupResponseDto,
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
        BackupExecutionRepository, backups::BackupRepository, destinations::DestinationRepository,
        organization::OrganizationRepository, volume_backups::VolumeBackupRepository,
    },
    services::{
        backup::{BackupFileService, ComposeConfigBackupService, PanelBackupService},
        schedule::ScheduleService,
    },
};

type ApiError = (StatusCode, String);

pub struct BackupController {
    db: Arc<sqlx::SqlitePool>,
    service: Arc<ScheduleService>,
    repo_backup: Arc<BackupRepository>,
    repo_execution: Arc<BackupExecutionRepository>,
    panel_backup: Arc<PanelBackupService>,
    backup_files: Arc<BackupFileService>,
    compose_config_backup: Arc<ComposeConfigBackupService>,
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
        repo_execution: Arc<BackupExecutionRepository>,
        panel_backup: Arc<PanelBackupService>,
        backup_files: Arc<BackupFileService>,
        compose_config_backup: Arc<ComposeConfigBackupService>,
        repo_volume: Arc<VolumeBackupRepository>,
        repo_dest: Arc<DestinationRepository>,
        repo_org: Arc<OrganizationRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            db,
            service,
            repo_backup,
            repo_execution,
            panel_backup,
            backup_files,
            compose_config_backup,
            repo_volume,
            repo_dest,
            repo_org,
            cache,
        }
    }

    #[post("/compose/{compose_id}/config/run")]
    async fn run_compose_config_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(compose_id): Path<i64>,
        Json(body): Json<ComposeConfigBackupDto>,
    ) -> Result<(StatusCode, Json<PanelBackupResponseDto>), ApiError> {
        self.compose_config_backup
            .create(compose_id, body.include_secrets.unwrap_or(false))
            .await
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(map_sqlx_error)
    }

    #[post("/panel/run")]
    async fn run_panel_backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
    ) -> Result<(StatusCode, Json<PanelBackupResponseDto>), ApiError> {
        self.panel_backup
            .create()
            .await
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(map_sqlx_error)
    }

    #[post("/panel/restore/stage")]
    async fn stage_panel_restore(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Json(body): Json<RestorePanelBackupDto>,
    ) -> Result<Json<StagePanelRestoreDto>, ApiError> {
        self.panel_backup
            .stage_restore(&body.archive, body.checksum_sha256.as_deref())
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/files")]
    async fn list_backup_files(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Query(query): Query<BackupFilesQueryDto>,
    ) -> Result<Json<Vec<crate::api::dto::backup::BackupFileDto>>, ApiError> {
        self.backup_files
            .list(query.destination_id, query.prefix.as_deref().unwrap_or(""))
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/files/download")]
    async fn download_backup_file(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Query(query): Query<DownloadBackupFileDto>,
    ) -> Result<Response<Body>, ApiError> {
        let bytes = self
            .backup_files
            .download(query.destination_id, &query.object_key)
            .await
            .map_err(map_sqlx_error)?;
        let filename = query
            .object_key
            .rsplit('/')
            .next()
            .unwrap_or("backup.bin")
            .replace(['\r', '\n', '"'], "_");
        let mut response = Response::new(Body::from(bytes));
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
                .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?,
        );
        Ok(response)
    }

    #[get("/retention/preview")]
    async fn preview_retention(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Query(query): Query<RetentionPreviewQueryDto>,
    ) -> Result<Json<RetentionPreviewDto>, ApiError> {
        self.backup_files
            .retention_preview(
                query.destination_id,
                query.prefix.as_deref().unwrap_or(""),
                query.keep_latest,
            )
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/files/verify")]
    async fn verify_backup_file(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Json(body): Json<VerifyBackupFileDto>,
    ) -> Result<Json<crate::api::dto::backup::BackupIntegrityDto>, ApiError> {
        self.backup_files
            .verify(body.destination_id, &body.object_key, &body.checksum_sha256)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/executions")]
    async fn list_executions(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Query(query): Query<BackupExecutionQueryDto>,
    ) -> Result<Json<Vec<BackupExecutionResponseDto>>, ApiError> {
        let limit = query.limit.unwrap_or(100).clamp(1, 500);
        self.repo_execution
            .list(
                query.backup_kind.map(|kind| kind.as_str()),
                query.backup_id,
                limit,
            )
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_sqlx_error)
    }

    #[post("/executions/{id}/retry")]
    async fn retry_execution(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        let execution = self
            .repo_execution
            .get(id)
            .await
            .map_err(map_sqlx_error)?
            .ok_or(sqlx::Error::RowNotFound)
            .map_err(map_sqlx_error)?;
        let status = crate::services::backup::types::BackupExecutionStatus::try_from(
            execution.status.as_str(),
        )
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;
        if status == crate::services::backup::types::BackupExecutionStatus::Running {
            return Err((
                StatusCode::CONFLICT,
                "backup execution is still running".into(),
            ));
        }
        let backup_id = execution.backup_id.ok_or((
            StatusCode::BAD_REQUEST,
            "execution is not linked to a retryable backup job".into(),
        ))?;
        let kind =
            crate::services::backup::types::BackupKind::try_from(execution.backup_kind.as_str())
                .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;
        let operation =
            crate::services::backup::types::BackupOperation::try_from(execution.operation.as_str())
                .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;
        let result = match (kind, operation) {
            (
                crate::services::backup::types::BackupKind::Database,
                crate::services::backup::types::BackupOperation::Backup,
            ) => self.service.run_database_backup(backup_id).await,
            (
                crate::services::backup::types::BackupKind::Volume,
                crate::services::backup::types::BackupOperation::Backup,
            ) => self.service.run_volume_backup(backup_id).await,
            (
                crate::services::backup::types::BackupKind::Database,
                crate::services::backup::types::BackupOperation::Restore,
            ) => {
                let object_key = execution.object_key.ok_or((
                    StatusCode::BAD_REQUEST,
                    "restore execution has no object key".into(),
                ))?;
                self.service
                    .restore_database_backup(backup_id, &object_key)
                    .await
            }
            (
                crate::services::backup::types::BackupKind::Volume,
                crate::services::backup::types::BackupOperation::Restore,
            ) => {
                let object_key = execution.object_key.ok_or((
                    StatusCode::BAD_REQUEST,
                    "restore execution has no object key".into(),
                ))?;
                self.service
                    .restore_volume_backup(backup_id, &object_key)
                    .await
            }
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "execution kind cannot be retried from this endpoint".into(),
                ));
            }
        };
        result.map(|_| StatusCode::ACCEPTED).map_err(map_sqlx_error)
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
