use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use tracing::debug;

use crate::docker::api::DockerApi;
use crate::docker::types::ContainerId;
use crate::logs::tail_container_logs;
use crate::store::Store;

pub mod proto {
    tonic::include_proto!("monitoring");
}

use proto::monitoring_service_server::MonitoringService;
pub use proto::monitoring_service_server::MonitoringServiceServer;
use proto::{
    ContainerMetricPoint, ContainerMetricsResponse, GetContainerMetricsRequest, GetMetricsRequest,
    LogChunk, LogStreamRequest, ServerMetricPoint, ServerMetricsResponse,
};

const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 1000;
const DEFAULT_TAIL_LINES: usize = 100;

pub struct MonitoringGrpc {
    store: Arc<Store>,
    server_id: i64,
    docker: DockerApi,
}

impl MonitoringGrpc {
    pub fn new(store: Arc<Store>, server_id: i64, docker: DockerApi) -> Self {
        Self {
            store,
            server_id,
            docker,
        }
    }

    fn check_server_id(&self, requested: i64) -> Result<(), Status> {
        if requested == self.server_id {
            return Ok(());
        }

        Err(Status::not_found(format!(
            "this agent serves server_id {}, not {requested}",
            self.server_id
        )))
    }
}

fn clamp_limit(raw: i64) -> i64 {
    if raw <= 0 {
        DEFAULT_LIMIT
    } else {
        raw.min(MAX_LIMIT)
    }
}

#[tonic::async_trait]
impl MonitoringService for MonitoringGrpc {
    type StreamLogsStream = Pin<Box<dyn Stream<Item = Result<LogChunk, Status>> + Send + 'static>>;

    async fn get_server_metrics(
        &self,
        request: Request<GetMetricsRequest>,
    ) -> Result<Response<ServerMetricsResponse>, Status> {
        let req = request.into_inner();
        self.check_server_id(req.server_id)?;

        let limit = clamp_limit(req.limit as i64);
        let rows = self
            .store
            .get_last_n_server_metrics(limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        debug!(
            requested = limit,
            returned = rows.len(),
            "serving server metrics gRPC"
        );

        let metrics = rows
            .into_iter()
            .map(|r| ServerMetricPoint {
                id: r.id.unwrap_or(0),
                timestamp: r.timestamp,
                cpu: r.cpu,
                cpu_model: r.cpu_model,
                cpu_cores: r.cpu_cores,
                cpu_physical_cores: r.cpu_physical_cores,
                cpu_speed: r.cpu_speed,
                os: r.os,
                distro: r.distro,
                kernel: r.kernel,
                arch: r.arch,
                mem_used: r.mem_used,
                mem_used_gb: r.mem_used_gb,
                mem_total: r.mem_total,
                uptime: r.uptime,
                disk_used: r.disk_used,
                total_disk: r.total_disk,
                network_in: r.network_in,
                network_out: r.network_out,
            })
            .collect();

        Ok(Response::new(ServerMetricsResponse { metrics }))
    }

    async fn get_container_metrics(
        &self,
        request: Request<GetContainerMetricsRequest>,
    ) -> Result<Response<ContainerMetricsResponse>, Status> {
        let req = request.into_inner();
        self.check_server_id(req.server_id)?;

        let limit = clamp_limit(req.limit as i64);
        let rows = self
            .store
            .get_last_n_container_metrics(&req.app_name, limit)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        debug!(
            container = %req.app_name,
            requested = limit,
            returned = rows.len(),
            "serving container metrics gRPC"
        );

        let metrics = rows
            .into_iter()
            .map(|r| ContainerMetricPoint {
                id: r.id.unwrap_or(0),
                timestamp: r.timestamp,
                container_id: r.container_id,
                name: r.name,
                cpu_perc: r.cpu_perc,
                mem_perc: r.mem_perc,
                mem_used_mb: r.mem_used_mb,
                mem_total_mb: r.mem_total_mb,
                net_in_mb: r.net_in_mb,
                net_out_mb: r.net_out_mb,
                block_read_mb: r.block_read_mb,
                block_write_mb: r.block_write_mb,
            })
            .collect();

        Ok(Response::new(ContainerMetricsResponse { metrics }))
    }

    async fn stream_logs(
        &self,
        request: Request<LogStreamRequest>,
    ) -> Result<Response<Self::StreamLogsStream>, Status> {
        let req = request.into_inner();

        let tail_lines = if req.tail_lines > 0 {
            req.tail_lines as usize
        } else {
            DEFAULT_TAIL_LINES
        };

        debug!(
            container = %req.container_id,
            tail_lines,
            "serving log tail"
        );

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let container_id = req.container_id;
        let docker = self.docker.clone();

        tokio::spawn(async move {
            let timestamp = chrono::Utc::now().timestamp();
            let typed_id = ContainerId::new(&container_id);
            for line in tail_container_logs(&docker, &typed_id, tail_lines).await {
                let chunk = LogChunk {
                    container_id: container_id.clone(),
                    log_line: line,
                    timestamp,
                    is_stderr: false,
                };
                if tx.send(Ok(chunk)).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_defaults_when_unset_or_negative() {
        assert_eq!(clamp_limit(0), DEFAULT_LIMIT);
        assert_eq!(clamp_limit(-5), DEFAULT_LIMIT);
    }

    #[test]
    fn limit_is_capped() {
        assert_eq!(clamp_limit(10), 10);
        assert_eq!(clamp_limit(50_000), MAX_LIMIT);
    }

    fn service_for(server_id: i64) -> MonitoringGrpc {
        let pool = sqlx::SqlitePool::connect_lazy("sqlite::memory:").expect("lazy pool");
        MonitoringGrpc::new(
            Arc::new(Store { pool }),
            server_id,
            DockerApi::new("/var/run/docker.sock"),
        )
    }

    #[tokio::test]
    async fn accepts_its_own_server_id() {
        assert!(service_for(7).check_server_id(7).is_ok());
    }

    #[tokio::test]
    async fn rejects_zero_as_wildcard() {
        assert!(service_for(7).check_server_id(0).is_err());
    }

    #[tokio::test]
    async fn rejects_a_different_server_id() {
        let status = service_for(1)
            .check_server_id(999)
            .expect_err("should reject another host's id");
        assert_eq!(status.code(), tonic::Code::NotFound);
    }
}
