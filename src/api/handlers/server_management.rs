use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::server_management::{
        ServerActionResultDto, ServerBackupDto, ServerCleanupExecutionDto, ServerManagementDto,
        ServerPrivateNetworkDto, UpdatePrivateNetworkDto, UpdateServerManagementDto,
    },
    core::middleware::permission::{
        RequirePermission, ServerCreatePermission, ServerReadPermission,
    },
    services::server_management::{
        ServerCleanupService, ServerLifecycleService, ServerManagementService,
        ServerPrivateNetworkService,
    },
};

type ApiError = (StatusCode, String);

pub struct ServerManagementController {
    management: Arc<ServerManagementService>,
    cleanup: Arc<ServerCleanupService>,
    lifecycle: Arc<ServerLifecycleService>,
    private_network: Arc<ServerPrivateNetworkService>,
}

#[controller("/servers/{server_id}/management")]
impl ServerManagementController {
    fn new(
        management: Arc<ServerManagementService>,
        cleanup: Arc<ServerCleanupService>,
        lifecycle: Arc<ServerLifecycleService>,
        private_network: Arc<ServerPrivateNetworkService>,
    ) -> Self {
        Self {
            management,
            cleanup,
            lifecycle,
            private_network,
        }
    }

    #[get("/private-network")]
    async fn private_network(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<Option<ServerPrivateNetworkDto>>, ApiError> {
        self.private_network
            .get(server_id)
            .await
            .map(Json)
            .map_err(map_error)
    }

    #[put("/private-network")]
    async fn update_private_network(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
        Json(body): Json<UpdatePrivateNetworkDto>,
    ) -> Result<Json<ServerPrivateNetworkDto>, ApiError> {
        self.private_network
            .update(server_id, body)
            .await
            .map(Json)
            .map_err(map_error)
    }

    #[delete("/private-network")]
    async fn disable_private_network(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.private_network
            .disable(server_id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
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
            .map_err(map_error)
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "server not found".into()),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(%other, "server management operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "server management operation failed".into(),
            )
        }
    }
}
