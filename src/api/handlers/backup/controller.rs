use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    BackupArchive, BackupCreateBackupRequest, BackupCreateBackupResponse,
    BackupCreateDatabaseBackupRequest, BackupCreateDatabaseBackupResponse,
    BackupCreateVolumeBackupRequest, BackupCreateVolumeBackupResponse,
    BackupDeleteBackupRequest, BackupDeleteDatabaseBackupRequest, BackupDeleteVolumeBackupRequest,
    BackupDownloadFileRequest, BackupDownloadFileResponse, BackupFile, BackupGetBackupRequest,
    BackupGetBackupResponse, BackupGetDatabaseBackupRequest, BackupGetDatabaseBackupResponse,
    BackupGetRestoreStatusRequest, BackupGetRestoreStatusResponse, BackupGetVolumeBackupRequest,
    BackupGetVolumeBackupResponse, BackupIntegrity, BackupListBackupsRequest,
    BackupListBackupsResponse,
    BackupListDatabaseBackupsRequest, BackupListDatabaseBackupsResponse, BackupListExecutionsRequest,
    BackupListExecutionsResponse, BackupListFilesRequest, BackupListFilesResponse,
    BackupListVolumeBackupsRequest, BackupListVolumeBackupsResponse, BackupPreviewRetentionRequest,
    BackupPreviewRetentionResponse, BackupRestoreBackupRequest, BackupRestoreDatabaseRequest,
    BackupRestoreStatus, BackupRestoreVolumeRequest, BackupRetentionPreview,
    BackupRetryExecutionRequest, BackupRollbackRestoreRequest,
    BackupRollbackRestoreResponse, BackupRunBackupRequest, BackupRunComposeBackupRequest,
    BackupRunComposeBackupResponse, BackupRunDatabaseBackupRequest, BackupRunPanelBackupRequest,
    BackupRunPanelBackupResponse, BackupRunVolumeBackupRequest, BackupStagedRestore,
    BackupStageRestoreRequest, BackupStageRestoreResponse, BackupUpdateBackupRequest,
    BackupUpdateBackupResponse,
    BackupUpdateDatabaseBackupRequest, BackupUpdateDatabaseBackupResponse,
    BackupUpdateVolumeBackupRequest, BackupUpdateVolumeBackupResponse, BackupValidateDestinationRequest,
    BackupValidateDestinationResponse, BackupVerifyFilesRequest, BackupVerifyFilesResponse,
    BackupService, BackupServiceExt, Empty,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::flash_gen::backups::{
    BackupsUpdateParams, CreateBackupParams, CreateVolumeBackupParams,
};
use crate::flash_gen::models::{Backups, BackupExecutions, VolumeBackups};
use crate::flash_gen::volume_backups::VolumeBackupsUpdateParams;
use crate::services::automation::backup::types::{
    BackupExecutionStatus, BackupKind, BackupOperation,
};

use super::helper::*;
use super::init::BackupController;

#[auto_grpc::service(connect = true, auth = true)]
impl BackupService for BackupController {
    // ── Owner service (database backup CRUD, mirrors the *_database_backup rpcs) ──

    #[permission(Database, CanRead)]
    async fn list_backups<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, BackupListBackupsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupListBackupsResponse> + Send + use<'a>> {
        let rows = self.queries.backups_get_all().await.map_err(map_error)?;
        let backups = rows.into_iter().map(|row| db_backup_to_proto(Backups::from(row))).collect();
        Response::ok(BackupListBackupsResponse { backups, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupGetBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupGetBackupResponse> + Send + use<'a>> {
        let item = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(BackupGetBackupResponse {
            backup: ::buffa::MessageField::some(db_backup_to_proto(item)),
            ..Default::default()
        })
    }

    #[permission(Database, CanCreate)]
    async fn create_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupCreateBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupCreateBackupResponse> + Send + use<'a>> {
        let org_id = self.resolve_organization_id(Some(request.organization_id)).await;
        let dest_id = self.require_destination_id(Some(request.destination_id)).await?;
        let service_id = self.verify_service_exists(request.service_id).await
            .ok_or_else(|| ConnectError::invalid_argument("valid backup resource is required"))?;
        let created = self.queries.create_backup(&CreateBackupParams {
            app_name: request.app_name.to_owned(),
            schedule: request.schedule.to_owned(),
            enabled: 1,
            database_name: request.database_name.to_owned(),
            prefix: request.prefix.to_owned(),
            service_name: request.service_name.unwrap_or_default().to_owned(),
            keep_latest_count: request.keep_latest_count.unwrap_or(0),
            backup_type: request.backup_type.to_owned(),
            database_type: request.database_type.to_owned(),
            metadata: request.metadata.unwrap_or_default().to_owned(),
            service_id,
            destination_id: dest_id,
            organization_id: org_id,
        }).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        Response::ok(BackupCreateBackupResponse {
            backup: ::buffa::MessageField::some(db_backup_to_proto(created)),
            ..Default::default()
        })
    }

    #[permission(Database, CanCreate)]
    async fn update_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupUpdateBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupUpdateBackupResponse> + Send + use<'a>> {
        let mut item = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        if let Some(v) = request.app_name { item.app_name = v.to_owned(); }
        if let Some(v) = request.schedule { item.schedule = v.to_owned(); }
        if let Some(v) = request.database_name { item.database_name = v.to_owned(); }
        if let Some(v) = request.prefix { item.prefix = v.to_owned(); }
        if let Some(v) = request.service_name { item.service_name = Some(v.to_owned()); }
        if let Some(v) = request.keep_latest_count { item.keep_latest_count = Some(v); }
        if let Some(v) = request.backup_type { item.backup_type = v.to_owned(); }
        if let Some(v) = request.database_type { item.database_type = Some(v.to_owned()); }
        if let Some(v) = request.metadata { item.metadata = Some(v.to_owned()); }
        if let Some(v) = request.destination_id { item.destination_id = v; }
        if let Some(v) = request.enabled { item.enabled = i64::from(v); }
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
        Response::ok(BackupUpdateBackupResponse {
            backup: ::buffa::MessageField::some(db_backup_to_proto(updated)),
            ..Default::default()
        })
    }

    #[permission(Database, CanDelete)]
    async fn delete_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupDeleteBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.queries.backups_delete(request.id).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn run_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRunBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.run_database_backup(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn restore_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRestoreBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.restore_database_backup(request.id, request.backup_file).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanRead)]
    async fn validate_destination<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupValidateDestinationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupValidateDestinationResponse> + Send + use<'a>> {
        match self.queries.get_destination_by_i_d(request.destination_id).await {
            Ok(_) => Response::ok(BackupValidateDestinationResponse {
                success: true,
                message: "destination is configured".into(),
                ..Default::default()
            }),
            Err(sqlx::Error::RowNotFound) => Response::ok(BackupValidateDestinationResponse {
                success: false,
                message: "selected S3 destination does not exist".into(),
                ..Default::default()
            }),
            Err(other) => Err(map_error(other)),
        }
    }

    // ── Panel/compose operations ──────────────────────────────────────

    #[permission(Application, CanDeploy)]
    async fn run_compose_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRunComposeBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupRunComposeBackupResponse> + Send + use<'a>> {
        let result = self.compose_config_backup
            .create(request.compose_id, request.include_secrets)
            .await
            .map_err(map_error)?;
        Response::ok(BackupRunComposeBackupResponse {
            archive: ::buffa::MessageField::some(BackupArchive {
                path: result.path,
                checksum_sha256: result.checksum_sha256,
                size_bytes: result.size_bytes,
                created_at: result.created_at,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn run_panel_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, BackupRunPanelBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupRunPanelBackupResponse> + Send + use<'a>> {
        let result = self.panel_backup.create().await.map_err(map_error)?;
        Response::ok(BackupRunPanelBackupResponse {
            archive: ::buffa::MessageField::some(BackupArchive {
                path: result.path,
                checksum_sha256: result.checksum_sha256,
                size_bytes: result.size_bytes,
                created_at: result.created_at,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn stage_restore<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupStageRestoreRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupStageRestoreResponse> + Send + use<'a>> {
        let result = self.panel_backup
            .stage_restore(request.archive, request.checksum_sha256)
            .await
            .map_err(map_error)?;
        Response::ok(BackupStageRestoreResponse {
            staged: ::buffa::MessageField::some(BackupStagedRestore {
                restore_id: result.restore_id,
                checksum_sha256: result.checksum_sha256,
                restart_required: result.restart_required,
                pending_marker: result.pending_marker,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn get_restore_status<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupGetRestoreStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupGetRestoreStatusResponse> + Send + use<'a>> {
        let status = self.panel_backup.restore_status(request.restore_id).await.map_err(map_error)?;
        Response::ok(BackupGetRestoreStatusResponse {
            status: ::buffa::MessageField::some(BackupRestoreStatus {
                restore_id: status.restore_id,
                status: status.status,
                message: status.message,
                updated_at: status.updated_at,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn rollback_restore<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRollbackRestoreRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupRollbackRestoreResponse> + Send + use<'a>> {
        let result = self.panel_backup
            .stage_rollback(request.recovery_database)
            .await
            .map_err(map_error)?;
        Response::ok(BackupRollbackRestoreResponse {
            staged: ::buffa::MessageField::some(BackupStagedRestore {
                restore_id: result.restore_id,
                checksum_sha256: result.checksum_sha256,
                restart_required: result.restart_required,
                pending_marker: result.pending_marker,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    // ── Remote destination files ──────────────────────────────────────

    #[permission(Database, CanRead)]
    async fn list_files<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupListFilesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupListFilesResponse> + Send + use<'a>> {
        let files = self.backup_files
            .list(request.destination_id, request.prefix.unwrap_or_default())
            .await
            .map_err(map_error)?;
        let files = files.into_iter().map(|f| BackupFile {
            name: f.name,
            path: f.path,
            size_bytes: f.size_bytes,
            modified_at: f.modified_at,
            ..Default::default()
        }).collect();
        Response::ok(BackupListFilesResponse { files, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn download_file<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupDownloadFileRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupDownloadFileResponse> + Send + use<'a>> {
        let bytes = self.backup_files
            .download(request.destination_id, request.object_key)
            .await
            .map_err(map_error)?;
        Response::ok(BackupDownloadFileResponse {
            size_bytes: bytes.len() as i64,
            object_key: request.object_key.to_owned(),
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn preview_retention<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupPreviewRetentionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupPreviewRetentionResponse> + Send + use<'a>> {
        let result = self.backup_files
            .retention_preview(request.destination_id, request.prefix.unwrap_or_default(), request.keep_latest)
            .await
            .map_err(map_error)?;
        Response::ok(BackupPreviewRetentionResponse {
            preview: ::buffa::MessageField::some(BackupRetentionPreview {
                keep: result.keep.into_iter().map(|f| BackupFile {
                    name: f.name,
                    path: f.path,
                    size_bytes: f.size_bytes,
                    modified_at: f.modified_at,
                    ..Default::default()
                }).collect(),
                delete: result.delete.into_iter().map(|f| BackupFile {
                    name: f.name,
                    path: f.path,
                    size_bytes: f.size_bytes,
                    modified_at: f.modified_at,
                    ..Default::default()
                }).collect(),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn verify_files<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupVerifyFilesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupVerifyFilesResponse> + Send + use<'a>> {
        let result = self.backup_files
            .verify(request.destination_id, request.object_key, request.checksum_sha256)
            .await
            .map_err(map_error)?;
        Response::ok(BackupVerifyFilesResponse {
            integrity: ::buffa::MessageField::some(BackupIntegrity {
                valid: result.valid,
                expected_sha256: result.expected_sha256,
                actual_sha256: result.actual_sha256,
                size_bytes: result.size_bytes,
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    // ── Executions ────────────────────────────────────────────────────

    #[permission(Database, CanRead)]
    async fn list_executions<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupListExecutionsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupListExecutionsResponse> + Send + use<'a>> {
        let limit = request.limit.unwrap_or(100).clamp(1, 500);
        let items: Vec<BackupExecutions> = sqlx::query_as(
            "SELECT * FROM backup_executions ORDER BY started_at DESC, id DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.queries.pool)
        .await
        .map_err(map_error)?;
        let executions = items.into_iter().map(execution_to_proto).collect();
        Response::ok(BackupListExecutionsResponse { executions, ..Default::default() })
    }

    #[permission(Application, CanDeploy)]
    async fn retry_execution<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRetryExecutionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let execution = self.queries.backup_executions_get(request.id).await.map_err(|error| match error {
            sqlx::Error::RowNotFound => ConnectError::not_found("backup execution not found"),
            other => map_error(other),
        })?;
        let status = BackupExecutionStatus::from_str(&execution.status)
            .ok_or_else(|| ConnectError::new(ErrorCode::Internal, format!("invalid backup execution status: {}", execution.status)))?;
        if status == BackupExecutionStatus::Running {
            return Err(ConnectError::new(ErrorCode::FailedPrecondition, "backup execution is still running"));
        }
        let backup_id = execution.backup_id
            .ok_or_else(|| ConnectError::invalid_argument("execution is not linked to a retryable backup job"))?;
        let kind = BackupKind::from_str(&execution.backup_kind)
            .ok_or_else(|| ConnectError::new(ErrorCode::Internal, format!("invalid backup kind: {}", execution.backup_kind)))?;
        let operation = BackupOperation::from_str(&execution.operation)
            .ok_or_else(|| ConnectError::new(ErrorCode::Internal, format!("invalid backup operation: {}", execution.operation)))?;
        let result = match (kind, operation) {
            (BackupKind::Database, BackupOperation::Backup) => {
                self.service.run_database_backup(backup_id).await
            }
            (BackupKind::Volume, BackupOperation::Backup) => {
                self.service.run_volume_backup(backup_id).await
            }
            (BackupKind::Database, BackupOperation::Restore) => {
                let object_key = execution.object_key
                    .ok_or_else(|| ConnectError::invalid_argument("restore execution has no object key"))?;
                self.service.restore_database_backup(backup_id, &object_key).await
            }
            (BackupKind::Volume, BackupOperation::Restore) => {
                let object_key = execution.object_key
                    .ok_or_else(|| ConnectError::invalid_argument("restore execution has no object key"))?;
                self.service.restore_volume_backup(backup_id, &object_key).await
            }
            _ => return Err(ConnectError::invalid_argument("execution kind cannot be retried from this endpoint")),
        };
        result.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    // ── Database backup CRUD ──────────────────────────────────────────

    #[permission(Database, CanRead)]
    async fn list_database_backups<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, BackupListDatabaseBackupsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupListDatabaseBackupsResponse> + Send + use<'a>> {
        let rows = self.queries.backups_get_all().await.map_err(map_error)?;
        let backups = rows.into_iter().map(|row| db_backup_to_proto(Backups::from(row))).collect();
        Response::ok(BackupListDatabaseBackupsResponse { backups, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupGetDatabaseBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupGetDatabaseBackupResponse> + Send + use<'a>> {
        let item = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(BackupGetDatabaseBackupResponse {
            backup: ::buffa::MessageField::some(db_backup_to_proto(item)),
            ..Default::default()
        })
    }

    #[permission(Database, CanCreate)]
    async fn create_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupCreateDatabaseBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupCreateDatabaseBackupResponse> + Send + use<'a>> {
        let org_id = self.resolve_organization_id(Some(request.organization_id)).await;
        let dest_id = self.require_destination_id(Some(request.destination_id)).await?;
        let service_id = self.verify_service_exists(request.service_id).await
            .ok_or_else(|| ConnectError::invalid_argument("valid backup resource is required"))?;
        let created = self.queries.create_backup(&CreateBackupParams {
            app_name: request.app_name.to_owned(),
            schedule: request.schedule.to_owned(),
            enabled: 1,
            database_name: request.database_name.to_owned(),
            prefix: request.prefix.to_owned(),
            service_name: request.service_name.unwrap_or_default().to_owned(),
            keep_latest_count: request.keep_latest_count.unwrap_or(0),
            backup_type: request.backup_type.to_owned(),
            database_type: request.database_type.to_owned(),
            metadata: request.metadata.unwrap_or_default().to_owned(),
            service_id,
            destination_id: dest_id,
            organization_id: org_id,
        }).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        Response::ok(BackupCreateDatabaseBackupResponse {
            backup: ::buffa::MessageField::some(db_backup_to_proto(created)),
            ..Default::default()
        })
    }

    #[permission(Database, CanCreate)]
    async fn update_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupUpdateDatabaseBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupUpdateDatabaseBackupResponse> + Send + use<'a>> {
        let mut item = self.queries.backups_get_by_id(request.id).await.map_err(map_error)?;
        if let Some(v) = request.app_name { item.app_name = v.to_owned(); }
        if let Some(v) = request.schedule { item.schedule = v.to_owned(); }
        if let Some(v) = request.database_name { item.database_name = v.to_owned(); }
        if let Some(v) = request.prefix { item.prefix = v.to_owned(); }
        if let Some(v) = request.service_name { item.service_name = Some(v.to_owned()); }
        if let Some(v) = request.keep_latest_count { item.keep_latest_count = Some(v); }
        if let Some(v) = request.backup_type { item.backup_type = v.to_owned(); }
        if let Some(v) = request.database_type { item.database_type = Some(v.to_owned()); }
        if let Some(v) = request.metadata { item.metadata = Some(v.to_owned()); }
        if let Some(v) = request.destination_id { item.destination_id = v; }
        if let Some(v) = request.enabled { item.enabled = i64::from(v); }
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
        Response::ok(BackupUpdateDatabaseBackupResponse {
            backup: ::buffa::MessageField::some(db_backup_to_proto(updated)),
            ..Default::default()
        })
    }

    #[permission(Database, CanDelete)]
    async fn delete_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupDeleteDatabaseBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.queries.backups_delete(request.id).await.map_err(map_error)?;
        refresh_schedule_runner().await;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn run_database_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRunDatabaseBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.run_database_backup(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    // ── Volume backup CRUD ────────────────────────────────────────────

    #[permission(Database, CanRead)]
    async fn list_volume_backups<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, BackupListVolumeBackupsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupListVolumeBackupsResponse> + Send + use<'a>> {
        if let Some(CacheEnum::VolumeBackups(cached_items)) = self.cache.get(&CacheKey::VolumeBackups).await {
            let backups = cached_items.into_iter().map(volume_backup_to_proto).collect();
            return Response::ok(BackupListVolumeBackupsResponse { backups, ..Default::default() });
        }
        let rows = self.queries.volume_backups_get_all().await.map_err(map_error)?;
        let items: Vec<VolumeBackups> = rows.into_iter().map(VolumeBackups::from).collect();
        self.cache.insert(CacheKey::VolumeBackups, CacheEnum::VolumeBackups(items.clone())).await;
        let backups = items.into_iter().map(volume_backup_to_proto).collect();
        Response::ok(BackupListVolumeBackupsResponse { backups, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupGetVolumeBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupGetVolumeBackupResponse> + Send + use<'a>> {
        let item = self.queries.volume_backups_get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(BackupGetVolumeBackupResponse {
            backup: ::buffa::MessageField::some(volume_backup_to_proto(VolumeBackups::from(item))),
            ..Default::default()
        })
    }

    #[permission(Database, CanCreate)]
    async fn create_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupCreateVolumeBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupCreateVolumeBackupResponse> + Send + use<'a>> {
        let org_id = self.resolve_organization_id(request.organization_id).await;
        let dest_id = self.require_destination_id(request.destination_id).await?;
        crate::services::automation::schedule::schedule::validate_backup_cron_expression(request.cron_expression)
            .map_err(map_error)?;
        let service_id = self
            .verify_service_exists(
                request.service_id
                    .or(request.application_id)
                    .or(request.compose_id)
                    .or(request.database_id)
                    .or(request.postgres_id)
                    .or(request.mysql_id)
                    .or(request.mariadb_id)
                    .or(request.mongo_id)
                    .or(request.redis_id)
                    .or(request.libsql_id),
            )
            .await
            .ok_or_else(|| ConnectError::invalid_argument("valid backup resource is required"))?;
        let created = self.queries.create_volume_backup(&CreateVolumeBackupParams {
            name: request.name.to_owned(),
            volume_name: request.volume_name.to_owned(),
            prefix: request.prefix.to_owned(),
            service_type: request.service_type.to_owned(),
            app_name: request.app_name.to_owned(),
            service_name: request.service_name.unwrap_or_default().to_owned(),
            turn_off: i64::from(request.turn_off),
            cron_expression: request.cron_expression.to_owned(),
            keep_latest_count: request.keep_latest_count.unwrap_or(0),
            enabled: 1,
            destination_id: dest_id,
            organization_id: org_id,
            service_id,
        }).await.map_err(map_error)?;
        self.cache.invalidate(&CacheKey::VolumeBackups).await;
        refresh_schedule_runner().await;
        Response::ok(BackupCreateVolumeBackupResponse {
            backup: ::buffa::MessageField::some(volume_backup_to_proto(created)),
            ..Default::default()
        })
    }

    #[permission(Database, CanCreate)]
    async fn update_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupUpdateVolumeBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<BackupUpdateVolumeBackupResponse> + Send + use<'a>> {
        let mut item = VolumeBackups::from(self.queries.volume_backups_get_by_id(request.id).await.map_err(map_error)?);
        if let Some(v) = request.name { item.name = v.to_owned(); }
        if let Some(v) = request.volume_name { item.volume_name = v.to_owned(); }
        if let Some(v) = request.prefix { item.prefix = v.to_owned(); }
        if let Some(v) = request.service_type { item.service_type = v.to_owned(); }
        if let Some(v) = request.app_name { item.app_name = v.to_owned(); }
        if let Some(v) = request.service_name { item.service_name = Some(v.to_owned()); }
        if let Some(v) = request.turn_off { item.turn_off = i64::from(v); }
        if let Some(v) = request.cron_expression { item.cron_expression = v.to_owned(); }
        if let Some(v) = request.keep_latest_count { item.keep_latest_count = Some(v); }
        if let Some(v) = request.destination_id { item.destination_id = v; }
        if let Some(v) = request.enabled { item.enabled = i64::from(v); }
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
        Response::ok(BackupUpdateVolumeBackupResponse {
            backup: ::buffa::MessageField::some(volume_backup_to_proto(VolumeBackups::from(updated))),
            ..Default::default()
        })
    }

    #[permission(Database, CanDelete)]
    async fn delete_volume_backup<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupDeleteVolumeBackupRequest>,
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
        request: ServiceRequest<'_, BackupRunVolumeBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.run_volume_backup(request.id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn restore_database<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRestoreDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.restore_database_backup(request.id, request.backup_file).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanDeploy)]
    async fn restore_volume<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, BackupRestoreVolumeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.restore_volume_backup(request.id, request.backup_file).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }
}
