use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path};

use super::error::{ApiError, map_sqlx_error};
use crate::{
    api::dto::server::{
        ServerActionResultDto, ServerBackupDto, ServerCleanupExecutionDto, ServerManagementDto,
        UpdateServerManagementDto,
    },
    core::middleware::permission::{
        RequirePermission, ServerCreatePermission, ServerReadPermission,
    },
    services::server::{ServerCleanupService, ServerLifecycleService, ServerManagementService},
};

pub struct ServerManagementController {
    management: Arc<ServerManagementService>,
    cleanup: Arc<ServerCleanupService>,
    lifecycle: Arc<ServerLifecycleService>,
}

#[controller("/servers/{server_id}/management")]
impl ServerManagementController {
    fn new(
        management: Arc<ServerManagementService>,
        cleanup: Arc<ServerCleanupService>,
        lifecycle: Arc<ServerLifecycleService>,
    ) -> Self {
        Self {
            management,
            cleanup,
            lifecycle,
        }
    }

    #[get]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerManagementDto>, ApiError> {
        self.management
            .get(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[put]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
        Json(body): Json<UpdateServerManagementDto>,
    ) -> Result<Json<ServerManagementDto>, ApiError> {
        self.management
            .update(server_id, body)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/audit/repair")]
    async fn repair(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerActionResultDto>, ApiError> {
        self.lifecycle
            .auto_repair(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/gpu/configure")]
    async fn configure_gpu(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerActionResultDto>, ApiError> {
        self.lifecycle
            .configure_gpu(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/cleanup/run")]
    async fn cleanup(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerActionResultDto>, ApiError> {
        self.cleanup
            .run(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/cleanup/history")]
    async fn cleanup_history(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<Vec<ServerCleanupExecutionDto>>, ApiError> {
        self.cleanup
            .history(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/upgrade")]
    async fn upgrade(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerActionResultDto>, ApiError> {
        self.lifecycle
            .upgrade(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/backup")]
    async fn backup(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerBackupDto>, ApiError> {
        self.lifecycle
            .backup(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/diagnostics")]
    async fn diagnostics(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerActionResultDto>, ApiError> {
        self.lifecycle
            .diagnostics(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }
}
