use crate::{
    core::config::Config,
    db::models::{container_metrics::ContainerMetric, server_metrics::ServerMetric},
    services::{monitoring::agent_auth::MonitoringAgentAuth, server::RemoteServerService},
};
use auto_di::singleton;
use std::sync::Arc;
use tonic::{Request, metadata::MetadataValue};

pub mod proto {
    tonic::include_proto!("monitoring");
}

pub struct MonitoringService {
    server_service: Arc<RemoteServerService>,
    config: Arc<Config>,
    agent_auth: Arc<MonitoringAgentAuth>,
}

#[singleton]
impl MonitoringService {
    pub fn new(
        server_service: Arc<RemoteServerService>,
        config: Arc<Config>,
        agent_auth: Arc<MonitoringAgentAuth>,
    ) -> Self {
        Self {
            server_service,
            config,
            agent_auth,
        }
    }

    async fn client(
        &self,
        server_id: i64,
    ) -> Result<
        proto::monitoring_service_client::MonitoringServiceClient<tonic::transport::Channel>,
        String,
    > {
        let server = self
            .server_service
            .get_by_id(server_id)
            .await
            .map_err(|_| format!("monitoring server {server_id} not found"))?;
        let host = match server.ip_address.as_str() {
            "" | "0.0.0.0" | "localhost" => "127.0.0.1",
            other => other,
        };
        proto::monitoring_service_client::MonitoringServiceClient::connect(format!(
            "http://{host}:50051"
        ))
        .await
        .map_err(|error| format!("monitoring agent {server_id} is unreachable: {error}"))
    }

    async fn authenticated<T>(&self, server_id: i64, message: T) -> Result<Request<T>, String> {
        let raw_token = self
            .agent_auth
            .query_token(server_id)
            .await
            .map_err(|error| error.to_string())?
            .unwrap_or_else(|| self.config.metrics_token.clone());
        let token: MetadataValue<_> = raw_token
            .parse()
            .map_err(|_| "METRICS_TOKEN is not valid gRPC metadata".to_string())?;
        let mut request = Request::new(message);
        request.metadata_mut().insert("x-metrics-token", token);
        Ok(request)
    }

    pub async fn server_history(
        &self,
        server_id: i64,
        limit: i64,
    ) -> Result<Vec<ServerMetric>, String> {
        let mut client = self.client(server_id).await?;
        let request = self
            .authenticated(
                server_id,
                proto::GetMetricsRequest {
                    server_id,
                    limit: limit.clamp(1, 1000) as i32,
                },
            )
            .await?;
        let metrics = client
            .get_server_metrics(request)
            .await
            .map_err(|error| format!("agent metric query failed: {error}"))?
            .into_inner()
            .metrics;
        let _ = self.agent_auth.touch_seen(server_id).await;
        Ok(metrics
            .into_iter()
            .map(|metric| ServerMetric {
                timestamp: chrono::DateTime::parse_from_rfc3339(&metric.timestamp)
                    .ok()
                    .map(|value| value.timestamp()),
                server_id,
                cpu: metric.cpu,
                cpu_model: metric.cpu_model,
                cpu_cores: i64::from(metric.cpu_cores),
                cpu_physical_cores: i64::from(metric.cpu_physical_cores),
                cpu_speed: metric.cpu_speed,
                os: metric.os,
                distro: metric.distro,
                kernel: metric.kernel,
                arch: metric.arch,
                mem_used: metric.mem_used,
                mem_used_gb: metric.mem_used_gb,
                mem_total: metric.mem_total,
                uptime: metric.uptime as i64,
                disk_used: metric.disk_used,
                total_disk: metric.total_disk,
                network_in: metric.network_in,
                network_out: metric.network_out,
            })
            .collect())
    }

    pub async fn container_history(
        &self,
        server_id: i64,
        app_name: &str,
        limit: i64,
    ) -> Result<Vec<ContainerMetric>, String> {
        let mut client = self.client(server_id).await?;
        let request = self
            .authenticated(
                server_id,
                proto::GetContainerMetricsRequest {
                    server_id,
                    app_name: app_name.to_owned(),
                    limit: limit.clamp(1, 1000) as i32,
                },
            )
            .await?;
        let metrics = client
            .get_container_metrics(request)
            .await
            .map_err(|error| format!("agent container query failed: {error}"))?
            .into_inner()
            .metrics;
        let _ = self.agent_auth.touch_seen(server_id).await;
        Ok(metrics
            .into_iter()
            .map(|metric| ContainerMetric {
                id: (metric.id > 0).then_some(metric.id),
                timestamp: chrono::DateTime::parse_from_rfc3339(&metric.timestamp)
                    .map(|value| value.timestamp())
                    .unwrap_or_default(),
                container_id: metric.container_id,
                container_name: metric.name,
                metrics_json: serde_json::json!({
                    "cpu_percent": metric.cpu_perc,
                    "memory_percent": metric.mem_perc,
                    "memory_used_mb": metric.mem_used_mb,
                    "memory_limit_mb": metric.mem_total_mb,
                    "net_rx_kbps": metric.net_in_mb * 1024.0,
                    "net_tx_kbps": metric.net_out_mb * 1024.0,
                    "block_read_mb": metric.block_read_mb,
                    "block_write_mb": metric.block_write_mb
                })
                .to_string(),
                server_id,
                application_id: (metric.application_id > 0).then_some(metric.application_id),
                compose_id: (metric.compose_id > 0).then_some(metric.compose_id),
            })
            .collect())
    }

    pub async fn get_latest_metrics_per_server(&self) -> Result<Vec<ServerMetric>, String> {
        let servers = self
            .server_service
            .list()
            .await
            .map_err(|error| error.to_string())?;
        let mut latest = Vec::new();
        for server in servers {
            let Some(server_id) = server.id else { continue };
            match self.server_history(server_id, 1).await {
                Ok(mut metrics) => latest.append(&mut metrics),
                Err(error) => tracing::debug!(server_id, %error, "monitoring agent unavailable"),
            }
        }
        Ok(latest)
    }

    pub async fn get_latest_container_metrics(&self) -> Result<Vec<ContainerMetric>, String> {
        let servers = self
            .server_service
            .list()
            .await
            .map_err(|error| error.to_string())?;
        let mut latest = Vec::new();
        for server in servers {
            let Some(server_id) = server.id else { continue };
            match self.container_history(server_id, "", 1000).await {
                Ok(metrics) => {
                    let mut newest = std::collections::HashMap::new();
                    for metric in metrics {
                        newest.entry(metric.container_id.clone()).or_insert(metric);
                    }
                    latest.extend(newest.into_values());
                }
                Err(error) => tracing::debug!(server_id, %error, "monitoring agent unavailable"),
            }
        }
        Ok(latest)
    }
}
