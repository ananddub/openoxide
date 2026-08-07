use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use super::error::{ApiError, map_sqlx_error};
use crate::{
    api::dto::server::{PrivateNetworkHealthDto, ServerPrivateNetworkDto, UpdatePrivateNetworkDto},
    core::middleware::permission::{
        RequirePermission, ServerCreatePermission, ServerReadPermission,
    },
    services::server::ServerPrivateNetworkService,
};

pub struct ServerPrivateNetworkController {
    service: Arc<ServerPrivateNetworkService>,
}

#[controller("/servers/{server_id}/private-network")]
impl ServerPrivateNetworkController {
    fn new(service: Arc<ServerPrivateNetworkService>) -> Self {
        Self { service }
    }

    #[get]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<Option<ServerPrivateNetworkDto>>, ApiError> {
        self.service
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
        Json(body): Json<UpdatePrivateNetworkDto>,
    ) -> Result<Json<ServerPrivateNetworkDto>, ApiError> {
        self.service
            .update(server_id, body)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete]
    async fn disable(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .disable(server_id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[post("/setup")]
    async fn setup(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerPrivateNetworkDto>, ApiError> {
        self.service
            .setup_transport(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/health")]
    async fn health(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<PrivateNetworkHealthDto>, ApiError> {
        self.service
            .health(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/repair")]
    async fn repair(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerPrivateNetworkDto>, ApiError> {
        self.service
            .repair_transport(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/re-setup")]
    async fn re_setup(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerPrivateNetworkDto>, ApiError> {
        self.service
            .re_setup_transport(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/rotate-keys")]
    async fn rotate_keys(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<ServerPrivateNetworkDto>, ApiError> {
        self.service
            .rotate_wireguard(server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/teardown")]
    async fn teardown(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerCreatePermission>,
        Path(server_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .teardown_transport(server_id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}
