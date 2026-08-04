use crate::{
    db::{
        models::{container_metrics::ContainerMetric, server_metrics::ServerMetric},
        repository::{ContainerMetricRepository, ServerMetricRepository},
    },
    services::remote_server::ServerService,
};
use auto_di::singleton;
use serde_json::Value;
use std::sync::Arc;

pub mod proto {
    tonic::include_proto!("monitoring");
}

pub struct MonitoringService {
    server_service: Arc<ServerService>,
    repo_metrics: Arc<ServerMetricRepository>,
    container_repo: Arc<ContainerMetricRepository>,
}

fn is_local_ip(ip: &str) -> bool {
    ip == "127.0.0.1" || ip == "localhost" || ip == "0.0.0.0" || ip.is_empty()
}

#[singleton]
impl MonitoringService {
    pub fn new(
        server_service: Arc<ServerService>,
        repo_metrics: Arc<ServerMetricRepository>,
        container_repo: Arc<ContainerMetricRepository>,
    ) -> Self {
        Self {
            server_service,
            repo_metrics,
            container_repo,
        }
    }

    pub async fn record_container_metric(&self, metric: ContainerMetric) -> Result<i64, String> {
        self.container_repo
            .create(&metric)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn container_history(
        &self,
        server_id: i64,
        container_id: Option<&str>,
        limit: i64,
    ) -> Result<Vec<ContainerMetric>, String> {
        self.container_repo
            .history_for_server(server_id, container_id, limit.clamp(1, 1000))
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn record_server_metric(&self, metric: ServerMetric) -> Result<i64, String> {
        self.repo_metrics
            .create(&metric)
            .await
            .map_err(|e| format!("Database save metric failed: {}", e))
    }

    pub async fn get_latest_metrics(&self, limit: i64) -> Result<Vec<ServerMetric>, String> {
        self.repo_metrics
            .get_latest(limit)
            .await
            .map_err(|e| format!("Database get metrics failed: {}", e))
    }

    pub async fn get_latest_metrics_per_server(&self) -> Result<Vec<ServerMetric>, String> {
        self.repo_metrics
            .get_latest_per_server()
            .await
            .map_err(|e| format!("Database get per-server metrics failed: {e}"))
    }

    pub fn start_retention(self: &Arc<Self>) {
        let service = Arc::clone(self);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                interval.tick().await;
                let cutoff = chrono::Utc::now().timestamp() - 7 * 24 * 60 * 60;
                let server = service.repo_metrics.delete_older_than(cutoff).await;
                let container = service.container_repo.delete_older_than(cutoff).await;
                match (server, container) {
                    (Ok(server), Ok(container)) if server + container > 0 => tracing::info!(server, container, "pruned old panel metrics"),
                    (Err(error), _) | (_, Err(error)) => tracing::warn!(%error, "panel metric retention failed"),
                    _ => {}
                }
            }
        });
    }

    pub async fn fetch_server_metrics(&self, server_id: i64) -> Result<Value, String> {
        let grpc_url = if let Ok(server) = self.server_service.get_by_id(server_id).await {
            if is_local_ip(&server.ip_address) {
                "http://127.0.0.1:50051".to_string()
            } else {
                format!("http://{}:50051", server.ip_address)
            }
        } else {
            "http://127.0.0.1:50051".to_string()
        };

        let mut client =
            proto::monitoring_service_client::MonitoringServiceClient::connect(grpc_url)
                .await
                .map_err(|e| format!("Failed to connect to gRPC server: {}", e))?;

        let response = client
            .get_server_metrics(proto::GetMetricsRequest {
                server_id,
                limit: 50,
            })
            .await
            .map_err(|e| format!("gRPC GetServerMetrics request failed: {}", e))?;

        let inner = response.into_inner();
        serde_json::to_value(&inner.metrics)
            .map_err(|e| format!("Failed to serialize metrics list to JSON: {}", e))
    }

    pub async fn fetch_container_metrics(
        &self,
        server_id: i64,
        app_name: &str,
    ) -> Result<Value, String> {
        let grpc_url = if let Ok(server) = self.server_service.get_by_id(server_id).await {
            if is_local_ip(&server.ip_address) {
                "http://127.0.0.1:50051".to_string()
            } else {
                format!("http://{}:50051", server.ip_address)
            }
        } else {
            "http://127.0.0.1:50051".to_string()
        };

        let mut client =
            proto::monitoring_service_client::MonitoringServiceClient::connect(grpc_url)
                .await
                .map_err(|e| format!("Failed to connect to gRPC server: {}", e))?;

        let response = client
            .get_container_metrics(proto::GetContainerMetricsRequest {
                server_id,
                app_name: app_name.to_string(),
                limit: 50,
            })
            .await
            .map_err(|e| format!("gRPC GetContainerMetrics request failed: {}", e))?;

        let inner = response.into_inner();
        serde_json::to_value(&inner.metrics)
            .map_err(|e| format!("Failed to serialize container metrics list to JSON: {}", e))
    }
}
