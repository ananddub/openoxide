use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use tracing::debug;

use crate::docker::api::DockerApi;
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

/// Default number of metric points returned when a request omits `limit`.
const DEFAULT_LIMIT: i64 = 50;
/// Upper bound on `limit` so one query can't pull the whole table into memory.
const MAX_LIMIT: i64 = 1000;
/// Default log lines returned when a request omits `tail_lines`.
const DEFAULT_TAIL_LINES: usize = 100;

/// Serves the panel's read queries against this agent's local metric store.
pub struct MonitoringGrpc {
    store: Arc<Store>,
    /// This agent's own id. Requests naming a different host are rejected —
    /// the store holds only this host's metrics, so answering anyway would
    /// hand back the wrong host's data under the caller's label.
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
        // 0 means "whichever host you are" — the panel doesn't always know.
        if requested == 0 || requested == self.server_id {
            return Ok(());
        }

        Err(Status::not_found(format!(
            "this agent serves server_id {}, not {requested}",
            self.server_id
        )))
    }
}

/// Clamps a caller-supplied limit into `1..=MAX_LIMIT`.
fn clamp_limit(requested: i32) -> i64 {
    if requested <= 0 {
        DEFAULT_LIMIT
    } else {
        i64::from(requested).min(MAX_LIMIT)
    }
}

#[tonic::async_trait]
impl MonitoringService for MonitoringGrpc {
    async fn get_server_metrics(
        &self,
        request: Request<GetMetricsRequest>,
    ) -> Result<Response<ServerMetricsResponse>, Status> {
        let req = request.into_inner();
        self.check_server_id(req.server_id)?;
        let limit = clamp_limit(req.limit);

        let rows = self
            .store
            .get_last_n_server_metrics(limit)
            .await
            .map_err(|e| Status::internal(format!("could not read server metrics: {e}")))?;

        debug!(count = rows.len(), limit, "served server metrics query");

        let metrics = rows
            .into_iter()
            .map(|m| ServerMetricPoint {
                id: m.id.unwrap_or(0),
                timestamp: m.timestamp,
                cpu: m.cpu,
                cpu_model: m.cpu_model,
                cpu_cores: m.cpu_cores,
                cpu_physical_cores: m.cpu_physical_cores,
                cpu_speed: m.cpu_speed,
                os: m.os,
                distro: m.distro,
                kernel: m.kernel,
                arch: m.arch,
                mem_used: m.mem_used,
                mem_used_gb: m.mem_used_gb,
                mem_total: m.mem_total,
                uptime: m.uptime,
                disk_used: m.disk_used,
                total_disk: m.total_disk,
                network_in: m.network_in,
                network_out: m.network_out,
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
        let limit = clamp_limit(req.limit);

        let rows = self
            .store
            .get_last_n_container_metrics(&req.app_name, limit)
            .await
            .map_err(|e| Status::internal(format!("could not read container metrics: {e}")))?;

        debug!(
            count = rows.len(),
            app_name = %req.app_name,
            "served container metrics query"
        );

        let metrics = rows
            .into_iter()
            .map(|m| ContainerMetricPoint {
                id: m.id.unwrap_or(0),
                timestamp: m.timestamp,
                container_id: m.container_id,
                name: m.name,
                cpu_perc: m.cpu_perc,
                mem_perc: m.mem_perc,
                mem_used_mb: m.mem_used_mb,
                mem_total_mb: m.mem_total_mb,
                net_in_mb: m.net_in_mb,
                net_out_mb: m.net_out_mb,
                block_read_mb: m.block_read_mb,
                block_write_mb: m.block_write_mb,
            })
            .collect();

        Ok(Response::new(ContainerMetricsResponse { metrics }))
    }

    type StreamLogsStream = Pin<Box<dyn Stream<Item = Result<LogChunk, Status>> + Send + 'static>>;

    /// Tails a container's recent logs. This reads a bounded snapshot and then
    /// completes — it is not a live follow, despite the streaming response type.
    async fn stream_logs(
        &self,
        request: Request<LogStreamRequest>,
    ) -> Result<Response<Self::StreamLogsStream>, Status> {
        let req = request.into_inner();

        if req.container_id.trim().is_empty() {
            return Err(Status::invalid_argument("container_id is required"));
        }

        let tail_lines = if req.tail_lines > 0 {
            req.tail_lines as usize
        } else {
            DEFAULT_TAIL_LINES
        };

        debug!(
            container_id = %req.container_id,
            tail_lines,
            "serving log tail"
        );

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let container_id = req.container_id;
        let docker = self.docker.clone();

        tokio::spawn(async move {
            let timestamp = chrono::Utc::now().timestamp();
            for line in tail_container_logs(&docker, &container_id, tail_lines).await {
                let chunk = LogChunk {
                    container_id: container_id.clone(),
                    log_line: line,
                    timestamp,
                    is_stderr: false,
                };
                // Receiver hung up — stop reading rather than filling the channel.
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

    /// Builds a service whose store is never touched — these tests only
    /// exercise the server_id gate, which runs before any query.
    /// Async because sqlx needs a Tokio context even to build a lazy pool.
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
    async fn accepts_zero_as_wildcard() {
        assert!(service_for(7).check_server_id(0).is_ok());
    }

    /// Regression: the store has no server_id column, so a query for another
    /// host used to silently return this host's rows. The panel would then
    /// file them under the wrong server.
    #[tokio::test]
    async fn rejects_a_different_server_id() {
        let status = service_for(1)
            .check_server_id(999)
            .expect_err("should reject another host's id");
        assert_eq!(status.code(), tonic::Code::NotFound);
    }
}
