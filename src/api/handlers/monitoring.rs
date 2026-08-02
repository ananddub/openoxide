use crate::{
    api::dto::monitoring::{
        ContainerLogSseEventDto, ContainerMetricSseEventDto, IngestContainerMetricBatchDto,
        IngestContainerMetricDto, IngestSystemMetricDto, MetricIngestResponseDto,
        MonitoringStatusResponseDto,
    },
    db::models::server_metrics::ServerMetric,
    services::monitoring::{
        monitoring_service::MonitoringService,
        sse::{ContainerMetricSseEvent, MonitoringSseBus},
    },
    utils::jwt::claim::Claims,
};
use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
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
}

pub struct MonitoringController {
    service: Arc<MonitoringService>,
    sse_bus: Arc<MonitoringSseBus>,
}

#[controller("/api/monitoring")]
impl MonitoringController {
    fn new(service: Arc<MonitoringService>, sse_bus: Arc<MonitoringSseBus>) -> Self {
        Self { service, sse_bus }
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
        Json(body): Json<IngestSystemMetricDto>,
    ) -> Result<Json<MetricIngestResponseDto>, ApiError> {
        let metric = ServerMetric {
            timestamp: None,
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
        _claims: Claims,
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
        _claims: Claims,
        Path(id): Path<i64>,
        Query(query): Query<ContainerQueryParam>,
    ) -> Result<Json<Value>, ApiError> {
        let app_name = query.app_name.as_deref().unwrap_or("");
        let val = self
            .service
            .fetch_container_metrics(id, app_name)
            .await
            .map_err(|e| (StatusCode::BAD_GATEWAY, e))?;

        Ok(Json(val))
    }

    #[post("/containers")]
    async fn ingest_container_metrics(
        &self,
        Json(body): Json<IngestContainerMetricDto>,
    ) -> Result<Json<MetricIngestResponseDto>, ApiError> {
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
        Json(body): Json<IngestContainerMetricBatchDto>,
    ) -> Result<Json<MetricIngestResponseDto>, ApiError> {
        let count = body.metrics.len();

        for metric in body.metrics {
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
        _claims: Claims,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>> + 'static> {
        let rx = self.sse_bus.subscribe_container_metrics();
        let stream = tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(|res| match res {
            Ok(item) => {
                let json = serde_json::to_string(&item).unwrap_or_default();
                Some(Ok(Event::default().event("container-metric").data(json)))
            }
            Err(_) => None,
        });

        Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
    }

    #[get("/stream/logs", sse = ContainerLogSseEventDto)]
    async fn stream_logs(
        &self,
        _claims: Claims,
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
}
