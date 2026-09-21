use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    BackupOperationsService, BackupOperationsServiceExt, Empty, OperationResult,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::flash_gen::backups::{
    BackupsUpdateParams, CreateBackupParams, CreateVolumeBackupParams,
};
use crate::flash_gen::models::{Backups, BackupExecutions, VolumeBackups};
use crate::flash_gen::volume_backups::VolumeBackupsUpdateParams;

use super::helper::*;
use super::init::BackupController;

#[auto_grpc::service(connect = true, auth = true)]
impl BackupOperationsService for BackupController {
    #[permission(Application, CanDeploy)]
    async fn run_compose_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: ComposeConfigBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.compose_config_backup
            .create(request.id, dto.include_secrets.unwrap_or(false))
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "run_compose_backup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn run_panel_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let result = self.panel_backup.create().await.map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "run_panel_backup".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn stage_restore<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: RestorePanelBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.panel_backup
            .stage_restore(&dto.archive, dto.checksum_sha256.as_deref())
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "stage_restore".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn get_restore_status<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let restore_id = request.payload;
        let status = self.panel_backup.restore_status(&restore_id).await.map_err(map_error)?;
        let payload = serde_json::to_string(&status).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "panel_restore_status".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn rollback_restore<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: RollbackPanelRestoreDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.panel_backup
            .stage_rollback(&dto.recovery_database)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "rollback_restore".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn list_files<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let destination_id = request.organization_id.unwrap_or(0);
        let prefix = request.query.as_deref().unwrap_or("");
        let files = self.backup_files.list(destination_id, prefix).await.map_err(map_error)?;
        let items = files.into_iter().map(|f| {
            let payload = serde_json::to_string(&f).unwrap_or_default();
            ResourceResponse { id: 0, kind: "backup_file".into(), payload, ..Default::default() }
        }).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn download_file<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: DownloadBackupFileDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let bytes = self.backup_files.download(dto.destination_id, &dto.object_key).await.map_err(map_error)?;
        let payload = serde_json::to_string(&serde_json::json!({
            "size": bytes.len(),
            "object_key": dto.object_key,
        })).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "backup_file_download".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn preview_retention<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: RetentionPreviewQueryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.backup_files
            .retention_preview(dto.destination_id, dto.prefix.as_deref().unwrap_or(""), dto.keep_latest)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "retention_preview".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn verify_files<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: VerifyBackupFileDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.backup_files
            .verify(dto.destination_id, &dto.object_key, &dto.checksum_sha256)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "verify_files".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn list_executions<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let limit = request.limit.unwrap_or(100).clamp(1, 500);
        let items: Vec<BackupExecutions> = sqlx::query_as(
            "SELECT * FROM backup_executions ORDER BY started_at DESC, id DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.queries.pool)
        .await
        .map_err(map_error)?;
        let items = items.into_iter().map(BackupExecutionResponseDto::from).map(execution_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanDeploy)]
    async fn retry_execution<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let execution = self.queries.backup_executions_get(request.id).await.map_err(|error| match error {
            sqlx::Error::RowNotFound => ConnectError::not_found("backup execution not found"),
            other => map_error(other),
        })?;
        let status = crate::services::automation::backup::types::BackupExecutionStatus::from_str(&execution.status)
            .ok_or_else(|| ConnectError::new(ErrorCode::Internal, format!("invalid backup execution status: {}", execution.status)))?;
        if status == crate::services::automation::backup::types::BackupExecutionStatus::Running {
            return Err(ConnectError::new(ErrorCode::FailedPrecondition, "backup execution is still running"));
        }
        let backup_id = execution.backup_id
            .ok_or_else(|| ConnectError::invalid_argument("execution is not linked to a retryable backup job"))?;
        let kind = crate::services::automation::backup::types::BackupKind::from_str(&execution.backup_kind)
            .ok_or_else(|| ConnectError::new(ErrorCode::Internal, format!("invalid backup kind: {}", execution.backup_kind)))?;
        let operation = crate::services::automation::backup::types::BackupOperation::from_str(&execution.operation)
            .ok_or_else(|| ConnectError::new(ErrorCode::Internal, format!("invalid backup operation: {}", execution.operation)))?;
        let result = match (kind, operation) {
            (crate::services::automation::backup::types::BackupKind::Database, crate::services::automation::backup::types::BackupOperation::Backup) => {
                self.service.run_database_backup(backup_id).await
            }
            (crate::services::automation::backup::types::BackupKind::Volume, crate::services::automation::backup::types::BackupOperation::Backup) => {
                self.service.run_volume_backup(backup_id).await
            }
            (crate::services::automation::backup::types::BackupKind::Database, crate::services::automation::backup::types::BackupOperation::Restore) => {
                let object_key = execution.object_key
                    .ok_or_else(|| ConnectError::invalid_argument("restore execution has no object key"))?;
                self.service.restore_database_backup(backup_id, &object_key).await
            }
            (crate::services::automation::backup::types::BackupKind::Volume, crate::services::automation::backup::types::BackupOperation::Restore) => {
                let object_key = execution.object_key
                    .ok_or_else(|| ConnectError::invalid_argument("restore execution has no object key"))?;
                self.service.restore_volume_backup(backup_id, &object_key).await
            }
            _ => return Err(ConnectError::invalid_argument("execution kind cannot be retried from this endpoint")),
        };
        result.map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "retry_execution".into(),
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn list_database_backups<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let rows = self.queries.backups_get_all().await.map_err(map_error)?;
        let items = rows.into_iter().map(|row| BackupResponseDto::from(Backups::from(row))).map(db_backup_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let item = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(db_backup_to_resource(BackupResponseDto::from(item)))
    }

    #[permission(Database, CanCreate)]
    async fn create_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let org_id = self.resolve_organization_id(Some(dto.organization_id)).await;
        let dest_id = self.require_destination_id(Some(dto.destination_id)).await?;
        let service_id = self.verify_service_exists(dto.service_id).await
            .ok_or_else(|| ConnectError::invalid_argument("valid backup resource is required"))?;
        let created = self.queries.create_backup(&CreateBackupParams {
            app_name: dto.app_name,
            schedule: dto.schedule,
            enabled: 1,
            database_name: dto.database_name,
            prefix: dto.prefix,
            service_name: dto.service_name.unwrap_or_default(),
            keep_latest_count: dto.keep_latest_count.unwrap_or(0),
            backup_type: dto.backup_type,
            database_type: dto.database_type,
            metadata: dto.metadata.unwrap_or_default(),
            service_id,
            destination_id: dest_id,
            organization_id: org_id,
        }).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        Response::ok(db_backup_to_resource(BackupResponseDto::from(created)))
    }

    #[permission(Database, CanCreate)]
    async fn update_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let mut item = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        if let Some(v) = dto.app_name { item.app_name = v; }
        if let Some(v) = dto.schedule { item.schedule = v; }
        if let Some(v) = dto.database_name { item.database_name = v; }
        if let Some(v) = dto.prefix { item.prefix = v; }
        if let Some(v) = dto.service_name { item.service_name = Some(v); }
        if let Some(v) = dto.keep_latest_count { item.keep_latest_count = Some(v); }
        if let Some(v) = dto.backup_type { item.backup_type = v; }
        if let Some(v) = dto.database_type { item.database_type = Some(v); }
        if let Some(v) = dto.metadata { item.metadata = Some(v); }
        if let Some(v) = dto.destination_id { item.destination_id = v; }
        if let Some(v) = dto.enabled { item.enabled = v; }
        self.queries.backups_update(&BackupsUpdateParams {
            app_name: item.app_name.clone(),
            schedule: item.schedule.clone(),
            enabled: item.enabled,
            database_name: item.database_name.clone(),
            prefix: item.prefix.clone(),
            service_name: item.service_name.clone().unwrap_or_default(),
            keep_latest_count: item.keep_latest_count.unwrap_or(0),
            backup_type: item.backup_type.clone(),
            database_type: item.database_type.clone().unwrap_or_default(),
            metadata: item.metadata.clone().unwrap_or_default(),
            service_id: item.service_id.unwrap_or(0),
            destination_id: item.destination_id,
            organization_id: item.organization_id,
            created_at: item.created_at,
            updated_at: chrono::Utc::now().timestamp(),
            id: request.id,
        }).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        let updated = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(db_backup_to_resource(BackupResponseDto::from(updated)))
    }

    #[permission(Database, CanDelete)]
    async fn delete_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.queries.backups_delete(request.id).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn run_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service.run_database_backup(request.id).await.map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "run_database_backup".into(),
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn list_volume_backups<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        if let Some(CacheEnum::VolumeBackups(cached_items)) = self.cache.get(&CacheKey::VolumeBackups).await {
            let items = cached_items.into_iter().map(VolumeBackupResponseDto::from).map(volume_backup_to_resource).collect();
            return Response::ok(ResourceListResponse { items, ..Default::default() });
        }
        let rows = self.queries.volume_backups_get_all().await.map_err(map_error)?;
        let items: Vec<VolumeBackups> = rows.into_iter().map(VolumeBackups::from).collect();
        self.cache.insert(CacheKey::VolumeBackups, CacheEnum::VolumeBackups(items.clone())).await;
        let items = items.into_iter().map(VolumeBackupResponseDto::from).map(volume_backup_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let item = self.queries.volume_backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(volume_backup_to_resource(VolumeBackupResponseDto::from(VolumeBackups::from(item))))
    }

    #[permission(Database, CanCreate)]
    async fn create_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateVolumeBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let org_id = self.resolve_organization_id(dto.organization_id).await;
        let dest_id = self.require_destination_id(dto.destination_id).await?;
        crate::services::automation::schedule::schedule::validate_backup_cron_expression(&dto.cron_expression)
            .map_err(map_error)?;
        let service_id = self
            .verify_service_exists(
                dto.service_id
                    .or(dto.application_id)
                    .or(dto.compose_id)
                    .or(dto.database_id)
                    .or(dto.postgres_id)
                    .or(dto.mysql_id)
                    .or(dto.mariadb_id)
                    .or(dto.mongo_id)
                    .or(dto.redis_id)
                    .or(dto.libsql_id),
            )
            .await
            .ok_or_else(|| ConnectError::invalid_argument("valid backup resource is required"))?;
        let created = self.queries.create_volume_backup(&CreateVolumeBackupParams {
            name: dto.name,
            volume_name: dto.volume_name,
            prefix: dto.prefix,
            service_type: dto.service_type,
            app_name: dto.app_name,
            service_name: dto.service_name.unwrap_or_default(),
            turn_off: dto.turn_off,
            cron_expression: dto.cron_expression,
            keep_latest_count: dto.keep_latest_count.unwrap_or(0),
            enabled: 1,
            destination_id: dest_id,
            organization_id: org_id,
            service_id,
        }).await.map_err(map_error)?;
        self.cache.invalidate(&CacheKey::VolumeBackups).await;
        refresh_schedule_runner().await;
        Response::ok(volume_backup_to_resource(VolumeBackupResponseDto::from(created)))
    }

    #[permission(Database, CanCreate)]
    async fn update_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchVolumeBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let mut item = VolumeBackups::from(self.queries.volume_backups_get_by_id(request.id).await.map_err(map_error)?);
        if let Some(v) = dto.name { item.name = v; }
        if let Some(v) = dto.volume_name { item.volume_name = v; }
        if let Some(v) = dto.prefix { item.prefix = v; }
        if let Some(v) = dto.service_type { item.service_type = v; }
        if let Some(v) = dto.app_name { item.app_name = v; }
        if let Some(v) = dto.service_name { item.service_name = Some(v); }
        if let Some(v) = dto.turn_off { item.turn_off = v; }
        if let Some(v) = dto.cron_expression { item.cron_expression = v; }
        if let Some(v) = dto.keep_latest_count { item.keep_latest_count = Some(v); }
        if let Some(v) = dto.destination_id { item.destination_id = v; }
        if let Some(v) = dto.enabled { item.enabled = v; }
        self.queries.volume_backups_update(&VolumeBackupsUpdateParams {
            name: item.name.clone(),
            volume_name: item.volume_name.clone(),
            prefix: item.prefix.clone(),
            service_type: item.service_type.clone(),
            app_name: item.app_name.clone(),
            service_name: item.service_name.clone().unwrap_or_default(),
            turn_off: item.turn_off,
            cron_expression: item.cron_expression.clone(),
            keep_latest_count: item.keep_latest_count.unwrap_or(0),
            enabled: item.enabled,
            destination_id: item.destination_id,
            organization_id: item.organization_id,
            service_id: item.service_id.unwrap_or(0),
            created_at: item.created_at,
            updated_at: chrono::Utc::now().timestamp(),
            id: request.id,
        }).await.map_err(map_error)?;
        self.cache.invalidate(&CacheKey::VolumeBackups).await;
        refresh_schedule_runner().await;
        let updated = self.queries.volume_backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(volume_backup_to_resource(VolumeBackupResponseDto::from(VolumeBackups::from(updated))))
    }

    #[permission(Database, CanDelete)]
    async fn delete_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.queries.volume_backups_delete(request.id).await.map_err(map_error)?;
        self.cache.invalidate(&CacheKey::VolumeBackups).await;
        refresh_schedule_runner().await;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn run_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service.run_volume_backup(request.id).await.map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "run_volume_backup".into(),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn restore_database<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        #[derive(serde::Deserialize)]
        struct RestoreBackupDto { backup_file: String }
        let dto: RestoreBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.restore_database_backup(request.id, &dto.backup_file).await.map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "restore_database".into(),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn restore_volume<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        #[derive(serde::Deserialize)]
        struct RestoreBackupDto { backup_file: String }
        let dto: RestoreBackupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.restore_volume_backup(request.id, &dto.backup_file).await.map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "restore_volume".into(),
            ..Default::default()
        })
    }
}
