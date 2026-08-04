use crate::{
    api::dto::monitoring::{
        ContainerLogSseEventDto, ContainerMetricSseEventDto, IngestContainerMetricBatchDto,
        IngestContainerMetricDto, IngestSystemMetricDto, MetricIngestResponseDto,
        MonitoringStatusResponseDto, MonitoringTokenResponseDto, RotateMonitoringTokenDto,
        MonitoringAgentStatusDto,
    },
    db::models::container_metrics::ContainerMetric,
    db::models::server_metrics::ServerMetric,
    services::monitoring::{
        agent_auth::MonitoringAgentAuth,
        monitoring_service::MonitoringService,
        sse::{ContainerMetricSseEvent, MonitoringSseBus},
    },
    core::middleware::permission::{RequirePermission, ServerMonitorPermission, AlertWritePermission},
};
use auto_di::resolve;
use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::{HeaderMap, StatusCode},
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use poem_openapi::Object;
use serde::Deserialize;
use serde_json::Value;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio_stream::StreamExt;

type ApiError = (StatusCode, String);

#[derive(Clone, Debug, Deserialize, Object)]
pub struct ContainerQueryParam {
    #[serde(rename = "appName")]
    pub app_name: Option<String>,
    pub container_id: Option<String>,
    pub limit: Option<i64>,
    pub organization_id: i64,
}

pub struct MonitoringController {
    service: Arc<MonitoringService>,
    sse_bus: Arc<MonitoringSseBus>,
    agent_auth: Arc<MonitoringAgentAuth>,
}

#[controller("/api/monitoring")]
impl MonitoringController {
    fn new(
        service: Arc<MonitoringService>,
        sse_bus: Arc<MonitoringSseBus>,
        agent_auth: Arc<MonitoringAgentAuth>,
    ) -> Self {
        Self {
            service,
            sse_bus,
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
                "/monitoring/stream/containers".into(),
                "/monitoring/stream/logs".into(),
            ],
        }))
    }

    #[post("/server")]
    async fn ingest_server_metrics(
        &self,
        headers: HeaderMap,
        Json(body): Json<IngestSystemMetricDto>,
    ) -> Result<Json<MetricIngestResponseDto>, ApiError> {
        verify_agent(&self.agent_auth, &headers, Some(body.server_id)).await?;
        let metric = ServerMetric {
            timestamp: None,
            server_id: body.server_id,
            cpu: body.cpu,
            cpu_model: body.cpu_model.unwrap_or_else(|| "Generic CPU".into()),
            cpu_cores: body.cpu_cores.unwrap_or(4),
            cpu_physical_cores: body.cpu_physical_cores.unwrap_or(2),
            cpu_speed: body.cpu_speed.unwrap_or(2.4),
            os: body.os.unwrap_or_else(|| "Linux".into()),
            distro: body.distro.unwrap_or_else(|| "Linux".into()),
            kernel: body.kernel.unwrap_or_else(|| "Linux".into()),
            arch: body.arch.unwrap_or_else(|| "x86_64".into()),
            mem_used: body.mem_used,
            mem_used_gb: body.mem_used_gb,
            mem_total: body.mem_total,
            uptime: body.uptime,
            disk_used: body.disk_used,
            total_disk: body.total_disk,
            network_in: body.network_in,
            network_out: body.network_out,
        };

        self.service
            .record_server_metric(metric)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

        Ok(Json(MetricIngestResponseDto {
            success: true,
            message: "System metric recorded successfully into SQLite database".to_string(),
        }))
    }

    #[get("/server/{id}")]
    async fn get_server_metrics(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerMonitorPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<Value>, ApiError> {
        let val = self
            .service
            .fetch_server_metrics(id)
            .await
            .map_err(|e| (StatusCode::BAD_GATEWAY, e))?;

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
                    query.container_id.as_deref(),
                    query.limit.unwrap_or(100),
                )
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?,
        )
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Json(val))
    }

    #[post("/containers")]
    async fn ingest_container_metrics(
        &self,
        headers: HeaderMap,
        Json(body): Json<IngestContainerMetricDto>,
    ) -> Result<Json<MetricIngestResponseDto>, ApiError> {
        verify_agent(&self.agent_auth, &headers, Some(body.server_id)).await?;
        persist_container_metric(&self.service, &body).await?;
        self.sse_bus
            .publish_container_metric(ContainerMetricSseEvent {
                server_id: body.server_id,
                application_id: body.application_id,
                compose_id: body.compose_id,
                container_id: body.container_id.clone(),
                container_name: body.container_name,
                cpu_percent: body.cpu_percent,
                memory_used_mb: body.memory_used_mb,
                memory_limit_mb: body.memory_limit_mb,
                net_rx_kbps: body.net_rx_kbps,
                net_tx_kbps: body.net_tx_kbps,
                timestamp: body.timestamp,
            });

        Ok(Json(MetricIngestResponseDto {
            success: true,
            message: "Container metric published to real-time SSE stream".to_string(),
        }))
    }

    /// Accepts a batch of container metrics and publishes each to the SSE bus.
    ///
    /// Agents stream stats continuously and flush them in batches, so this is
    /// the endpoint they use; the single-metric variant above is kept for
    /// anything posting one reading at a time.
    #[post("/containers/batch")]
    async fn ingest_container_metrics_batch(
        &self,
        headers: HeaderMap,
        Json(body): Json<IngestContainerMetricBatchDto>,
    ) -> Result<Json<MetricIngestResponseDto>, ApiError> {
        let server_id = body.metrics.first().map(|metric| metric.server_id);
        if body
            .metrics
            .iter()
            .any(|metric| Some(metric.server_id) != server_id)
        {
            return Err((
                StatusCode::BAD_REQUEST,
                "one batch may contain metrics for only one server".into(),
            ));
        }
        verify_agent(&self.agent_auth, &headers, server_id).await?;
        let count = body.metrics.len();

        for metric in body.metrics {
            persist_container_metric(&self.service, &metric).await?;
            self.sse_bus
                .publish_container_metric(ContainerMetricSseEvent {
                    server_id: metric.server_id,
                    application_id: metric.application_id,
                    compose_id: metric.compose_id,
                    container_id: metric.container_id,
                    container_name: metric.container_name,
                    cpu_percent: metric.cpu_percent,
                    memory_used_mb: metric.memory_used_mb,
                    memory_limit_mb: metric.memory_limit_mb,
                    net_rx_kbps: metric.net_rx_kbps,
                    net_tx_kbps: metric.net_tx_kbps,
                    timestamp: metric.timestamp,
                });
        }

        Ok(Json(MetricIngestResponseDto {
            success: true,
            message: format!("{count} container metrics published to real-time SSE stream"),
        }))
    }

    #[get("/stream/containers", sse = ContainerMetricSseEventDto)]
    async fn stream_container_metrics(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerMonitorPermission>,
        Query(query): Query<ContainerMetricStreamQuery>,
    ) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>> + 'static>, ApiError> {
        verify_server_organization(&self.agent_auth, query.server_id, query.organization_id)
            .await?;
        let rx = self.sse_bus.subscribe_container_metrics();
        let stream =
            tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(move |res| match res {
                Ok(item) if query.matches(&item) => {
                    let json = serde_json::to_string(&item).unwrap_or_default();
                    Some(Ok(Event::default().event("container-metric").data(json)))
                }
                _ => None,
            });

        Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15))))
    }

    #[get("/stream/logs", sse = ContainerLogSseEventDto)]
    async fn stream_logs(
        &self,
        RequirePermission(_claims, _): RequirePermission<ServerMonitorPermission>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>> + 'static> {
        let rx = self.sse_bus.subscribe_logs();
        let stream = tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(|res| match res {
            Ok(item) => {
                let json = serde_json::to_string(&item).unwrap_or_default();
                Some(Ok(Event::default().event("container-log").data(json)))
            }
            Err(_) => None,
        });

        Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
    }

    #[post("/agents/{server_id}/token")]
    async fn rotate_agent_token(
        &self,
        RequirePermission(claims, _): RequirePermission<AlertWritePermission>,
        Path(server_id): Path<i64>,
        Json(body): Json<RotateMonitoringTokenDto>,
    ) -> Result<Json<MonitoringTokenResponseDto>, ApiError> {
        if body.organization_id != claims.user.group_id {
            return Err((StatusCode::FORBIDDEN, "organization does not match authenticated scope".into()));
        }
        if !self.agent_auth.server_belongs_to_organization(server_id, body.organization_id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))? {
            return Err((StatusCode::FORBIDDEN, "server is not linked to this organization".into()));
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
        let status = self.agent_auth.status(server_id).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::NOT_FOUND, "monitoring agent is not registered".into()))?;
        let state = match status.last_seen_at {
            Some(ts) if chrono::Utc::now().timestamp() - ts <= 180 => "ONLINE",
            Some(_) => "STALE",
            None => "NEVER_SEEN",
        };
        Ok(Json(MonitoringAgentStatusDto { server_id: status.server_id, organization_id: status.organization_id, last_seen_at: status.last_seen_at, agent_version: status.agent_version, state: state.into() }))
    }
}

async fn persist_container_metric(
    service: &MonitoringService,
    metric: &IngestContainerMetricDto,
) -> Result<(), ApiError> {
    let metrics_json = serde_json::json!({
        "cpu_percent": metric.cpu_percent, "memory_used_mb": metric.memory_used_mb,
        "memory_limit_mb": metric.memory_limit_mb, "net_rx_kbps": metric.net_rx_kbps,
        "net_tx_kbps": metric.net_tx_kbps
    })
    .to_string();
    service
        .record_container_metric(ContainerMetric {
            id: None,
            timestamp: metric.timestamp,
            container_id: metric.container_id.clone(),
            container_name: metric.container_name.clone(),
            metrics_json,
            server_id: metric.server_id,
            application_id: (metric.application_id > 0).then_some(metric.application_id),
            compose_id: (metric.compose_id > 0).then_some(metric.compose_id),
        })
        .await
        .map(|_| ())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

#[derive(Clone, Debug, Deserialize, Object)]
pub struct ContainerMetricStreamQuery {
    pub server_id: i64,
    pub organization_id: i64,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
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

impl ContainerMetricStreamQuery {
    fn matches(&self, event: &ContainerMetricSseEvent) -> bool {
        event.server_id == self.server_id
            && self
                .application_id
                .is_none_or(|id| event.application_id == id)
            && self.compose_id.is_none_or(|id| event.compose_id == id)
    }
}

async fn verify_agent(
    auth: &MonitoringAgentAuth,
    headers: &HeaderMap,
    body_server_id: Option<i64>,
) -> Result<(), ApiError> {
    let expected = resolve::<crate::core::config::Config>()
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "configuration unavailable".into(),
            )
        })?
        .metrics_token
        .clone();
    let supplied = headers
        .get("x-metrics-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let per_server_valid = match body_server_id {
        Some(server_id) => auth
            .authenticate(server_id, supplied)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        None => false,
    };
    let global_valid = !expected.trim().is_empty() && supplied.as_bytes() == expected.as_bytes();
    if !per_server_valid && !global_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "invalid monitoring agent token".into(),
        ));
    }
    if let Some(body_server_id) = body_server_id {
        let header_server_id = headers
            .get("x-server-id")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<i64>().ok())
            .ok_or((
                StatusCode::BAD_REQUEST,
                "missing or invalid X-Server-Id".into(),
            ))?;
        if header_server_id != body_server_id {
            return Err((
                StatusCode::FORBIDDEN,
                "agent server identity does not match payload".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(server_id: i64, application_id: i64, compose_id: i64) -> ContainerMetricSseEvent {
        ContainerMetricSseEvent {
            server_id,
            application_id,
            compose_id,
            container_id: "abc".into(),
            container_name: "web".into(),
            cpu_percent: 1.0,
            memory_used_mb: 1.0,
            memory_limit_mb: 2.0,
            net_rx_kbps: 0.0,
            net_tx_kbps: 0.0,
            timestamp: 0,
        }
    }

    #[test]
    fn metric_stream_never_crosses_servers() {
        let query = ContainerMetricStreamQuery {
            server_id: 7,
            organization_id: 1,
            application_id: None,
            compose_id: None,
        };
        assert!(query.matches(&event(7, 1, 0)));
        assert!(!query.matches(&event(8, 1, 0)));
    }

    #[test]
    fn metric_stream_can_scope_to_an_application() {
        let query = ContainerMetricStreamQuery {
            server_id: 7,
            organization_id: 1,
            application_id: Some(42),
            compose_id: None,
        };
        assert!(query.matches(&event(7, 42, 0)));
        assert!(!query.matches(&event(7, 43, 0)));
    }
}
