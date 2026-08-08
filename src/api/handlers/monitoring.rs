use crate::core::middleware::permission::{Alert, CanMonitor, CanWrite, Server};
use crate::{
    api::dto::monitoring::{
        CreateMaintenanceWindowDto, MaintenanceWindowDto, MonitoringActionDto,
        MonitoringAgentStatusDto, MonitoringPolicyDto, MonitoringStatusResponseDto,
        MonitoringTokenResponseDto, RotateMonitoringTokenDto, UpdateMonitoringPolicyDto,
    },
    core::middleware::permission::RequirePermission,
    services::monitoring::{
        agent_auth::MonitoringAgentAuth, lifecycle::MonitoringLifecycleService,
        monitoring_service::MonitoringService,
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
    lifecycle: Arc<MonitoringLifecycleService>,
}

#[controller("/api/monitoring")]
impl MonitoringController {
    fn new(
        service: Arc<MonitoringService>,
        agent_auth: Arc<MonitoringAgentAuth>,
        lifecycle: Arc<MonitoringLifecycleService>,
    ) -> Self {
        Self {
            service,
            agent_auth,
            lifecycle,
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
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(id): Path<i64>,
    ) -> Result<Json<Value>, ApiError> {
        verify_server_organization(&self.agent_auth, id, permission.organization_id()).await?;
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
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(id): Path<i64>,
        Query(query): Query<ContainerQueryParam>,
    ) -> Result<Json<Value>, ApiError> {
        verify_claim_organization(permission.organization_id(), query.organization_id)?;
        verify_server_organization(&self.agent_auth, id, permission.organization_id()).await?;
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
        RequirePermission(_claims, permission): RequirePermission<Alert, CanWrite>,
        Path(server_id): Path<i64>,
        Json(body): Json<RotateMonitoringTokenDto>,
    ) -> Result<Json<MonitoringTokenResponseDto>, ApiError> {
        if body.organization_id != permission.organization_id() {
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
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<MonitoringAgentStatusDto>, ApiError> {
        verify_server_organization(&self.agent_auth, server_id, permission.organization_id())
            .await?;
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

    #[get("/policy/{organization_id}")]
    async fn policy(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<MonitoringPolicyDto>, ApiError> {
        verify_claim_organization(permission.organization_id(), organization_id)?;
        self.lifecycle
            .policy(organization_id)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[put("/policy/{organization_id}")]
    async fn update_policy(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Alert, CanWrite>,
        Path(organization_id): Path<i64>,
        Json(body): Json<UpdateMonitoringPolicyDto>,
    ) -> Result<Json<MonitoringPolicyDto>, ApiError> {
        verify_claim_organization(permission.organization_id(), organization_id)?;
        self.lifecycle
            .update_policy(organization_id, body)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[post("/agents/{server_id}/restart")]
    async fn restart_agent(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Alert, CanWrite>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<MonitoringActionDto>, ApiError> {
        verify_server_organization(&self.agent_auth, server_id, permission.organization_id())
            .await?;
        self.lifecycle
            .restart_agent(server_id)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[post("/agents/{server_id}/reinstall")]
    async fn reinstall_agent(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Alert, CanWrite>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<MonitoringActionDto>, ApiError> {
        verify_server_organization(&self.agent_auth, server_id, permission.organization_id())
            .await?;
        self.lifecycle
            .reinstall_agent(server_id)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[get("/server/{id}/export")]
    async fn export_server_metrics(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(id): Path<i64>,
    ) -> Result<String, ApiError> {
        verify_server_organization(&self.agent_auth, id, permission.organization_id()).await?;
        let metrics = self
            .service
            .server_history(id, 1000)
            .await
            .map_err(|error| (StatusCode::BAD_GATEWAY, error))?;
        let mut csv = String::from(
            "timestamp,server_id,cpu,mem_used,mem_total,disk_used,total_disk,network_in,network_out\n",
        );
        for metric in metrics {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                metric.timestamp.unwrap_or_default(),
                metric.server_id,
                metric.cpu,
                metric.mem_used,
                metric.mem_total,
                metric.disk_used,
                metric.total_disk,
                metric.network_in,
                metric.network_out
            ));
        }
        Ok(csv)
    }

    #[get("/server/{id}/prometheus")]
    async fn prometheus(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(id): Path<i64>,
    ) -> Result<String, ApiError> {
        verify_server_organization(&self.agent_auth, id, permission.organization_id()).await?;
        let metric = self
            .service
            .server_history(id, 1)
            .await
            .map_err(|error| (StatusCode::BAD_GATEWAY, error))?
            .into_iter()
            .next()
            .ok_or((StatusCode::NOT_FOUND, "no metrics available".into()))?;
        Ok(format!(
            "# TYPE rustploy_server_cpu_percent gauge\nrustploy_server_cpu_percent{{server_id=\"{}\"}} {}\n# TYPE rustploy_server_memory_used_bytes gauge\nrustploy_server_memory_used_bytes{{server_id=\"{}\"}} {}\n# TYPE rustploy_server_disk_used_bytes gauge\nrustploy_server_disk_used_bytes{{server_id=\"{}\"}} {}\n",
            id, metric.cpu, id, metric.mem_used, id, metric.disk_used
        ))
    }

    #[post("/maintenance/{organization_id}")]
    async fn create_maintenance_window(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Alert, CanWrite>,
        Path(organization_id): Path<i64>,
        Json(body): Json<CreateMaintenanceWindowDto>,
    ) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
        verify_claim_organization(permission.organization_id(), organization_id)?;
        if let Some(server_id) = body.server_id {
            verify_server_organization(&self.agent_auth, server_id, organization_id).await?;
        }
        let id = self
            .lifecycle
            .create_window(
                organization_id,
                body.server_id,
                body.starts_at,
                body.ends_at,
                &body.reason,
            )
            .await
            .map_err(internal)?;
        Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
    }

    #[get("/maintenance/{organization_id}")]
    async fn maintenance_windows(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<MaintenanceWindowDto>>, ApiError> {
        verify_claim_organization(permission.organization_id(), organization_id)?;
        self.lifecycle
            .windows(organization_id)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[delete("/maintenance/{organization_id}/{id}")]
    async fn delete_maintenance_window(
        &self,
        RequirePermission(_claims, permission): RequirePermission<Alert, CanWrite>,
        Path((organization_id, id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        verify_claim_organization(permission.organization_id(), organization_id)?;
        if self
            .lifecycle
            .delete_window(id, organization_id)
            .await
            .map_err(internal)?
        {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((StatusCode::NOT_FOUND, "maintenance window not found".into()))
        }
    }
}

fn verify_claim_organization(
    claim_organization_id: i64,
    organization_id: i64,
) -> Result<(), ApiError> {
    if claim_organization_id == organization_id {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            "organization does not match authenticated scope".into(),
        ))
    }
}

fn internal(error: sqlx::Error) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
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
