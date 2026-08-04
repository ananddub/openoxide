use crate::{
    api::dto::monitoring::{
        MonitoringAgentStatusDto, MonitoringStatusResponseDto, MonitoringTokenResponseDto,
        RotateMonitoringTokenDto,
    },
    core::middleware::permission::{
        AlertWritePermission, RequirePermission, ServerMonitorPermission,
    },
    services::monitoring::{
        agent_auth::MonitoringAgentAuth, monitoring_service::MonitoringService,
    },
};
use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use poem_openapi::Object;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

type ApiError = (StatusCode, String);

#[derive(Clone, Debug, Deserialize, Object)]
pub struct ContainerQueryParam {
    #[serde(rename = "appName")]
    pub app_name: Option<String>,
    pub limit: Option<i64>,
    pub organization_id: i64,
}

pub struct MonitoringController {
    service: Arc<MonitoringService>,
    agent_auth: Arc<MonitoringAgentAuth>,
}

#[controller("/api/monitoring")]
impl MonitoringController {
    fn new(service: Arc<MonitoringService>, agent_auth: Arc<MonitoringAgentAuth>) -> Self {
        Self {
            service,
            agent_auth,
        }
    }

    #[get("")]
    async fn status_index(&self) -> Result<Json<MonitoringStatusResponseDto>, ApiError> {
        Ok(Json(MonitoringStatusResponseDto {
            status: "ok".into(),
            service: "rustploy monitoring service".into(),
            endpoints: vec![
                "/monitoring/server/{id}".into(),
                "/monitoring/containers/{id}".into(),
            ],
        }))
    }

    #[get("/server/{id}")]
    async fn get_server_metrics(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerMonitorPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<Value>, ApiError> {
        let val = serde_json::to_value(
            self.service
                .server_history(id, 50)
                .await
                .map_err(|e| (StatusCode::BAD_GATEWAY, e))?,
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(val))
    }

    #[get("/containers/{id}")]
    async fn get_container_metrics(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerMonitorPermission>,
        Path(id): Path<i64>,
        Query(query): Query<ContainerQueryParam>,
    ) -> Result<Json<Value>, ApiError> {
        verify_server_organization(&self.agent_auth, id, query.organization_id).await?;
        let val = serde_json::to_value(
            self.service
                .container_history(
                    id,
                    query.app_name.as_deref().unwrap_or_default(),
                    query.limit.unwrap_or(100),
                )
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?,
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(val))
    }

    #[post("/agents/{server_id}/token")]
    async fn rotate_agent_token(
        &self,
        RequirePermission(claims, _): RequirePermission<AlertWritePermission>,
        Path(server_id): Path<i64>,
        Json(body): Json<RotateMonitoringTokenDto>,
    ) -> Result<Json<MonitoringTokenResponseDto>, ApiError> {
        if body.organization_id != claims.user.group_id {
            return Err((
                StatusCode::FORBIDDEN,
                "organization does not match authenticated scope".into(),
            ));
        }
        if !self
            .agent_auth
            .server_belongs_to_organization(server_id, body.organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        {
            return Err((
                StatusCode::FORBIDDEN,
                "server is not linked to this organization".into(),
            ));
        }
        let token = self
            .agent_auth
            .rotate(server_id, body.organization_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(Json(MonitoringTokenResponseDto {
            server_id,
            organization_id: body.organization_id,
            token,
        }))
    }

    #[get("/agents/{server_id}/status")]
    async fn agent_status(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerMonitorPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<MonitoringAgentStatusDto>, ApiError> {
        let status = self
            .agent_auth
            .status(server_id)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((
                StatusCode::NOT_FOUND,
                "monitoring agent is not registered".into(),
            ))?;
        let state = match status.last_seen_at {
            Some(ts) if chrono::Utc::now().timestamp() - ts <= 180 => "ONLINE",
            Some(_) => "STALE",
            None => "NEVER_SEEN",
        };
        Ok(Json(MonitoringAgentStatusDto {
            server_id: status.server_id,
            organization_id: status.organization_id,
            last_seen_at: status.last_seen_at,
            agent_version: status.agent_version,
            state: state.into(),
        }))
    }
}

async fn verify_server_organization(
    auth: &MonitoringAgentAuth,
    server_id: i64,
    organization_id: i64,
) -> Result<(), ApiError> {
    let bound = auth
        .organization_id(server_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if bound == Some(organization_id) {
        Ok(())
    } else {
        Err((StatusCode::NOT_FOUND, "monitoring server not found".into()))
    }
}
