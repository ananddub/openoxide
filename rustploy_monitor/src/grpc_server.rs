use crate::containers::tail_container_logs;
use crate::db::Db;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use tracing::info;

pub mod proto {
    tonic::include_proto!("monitoring");
}

use proto::monitoring_service_server::MonitoringService;
pub use proto::monitoring_service_server::MonitoringServiceServer;
use proto::{
    ContainerMetricPoint, ContainerMetricsRequest, ContainerMetricsResponse,
    GetContainerMetricsRequest, GetMetricsRequest, HealthAck, HealthReportRequest, LogChunk,
    LogStreamRequest, MetricsAck, ServerMetricPoint, ServerMetricsResponse, SystemMetricsRequest,
};

pub struct MonitoringGrpcServer {
    db: Arc<Db>,
}

impl MonitoringGrpcServer {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl MonitoringService for MonitoringGrpcServer {
    type StreamSystemMetricsStream =
        Pin<Box<dyn Stream<Item = Result<MetricsAck, Status>> + Send + 'static>>;

    async fn stream_system_metrics(
        &self,
        request: Request<tonic::Streaming<SystemMetricsRequest>>,
    ) -> Result<Response<Self::StreamSystemMetricsStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Ok(Some(msg)) = in_stream.message().await {
                info!(
                    "gRPC System Stream Received - Server ID: {}, CPU: {:.2}%, RAM: {:.2}%",
                    msg.server_id, msg.cpu_usage, msg.memory_percent
                );
                let ack = MetricsAck {
                    success: true,
                    received_timestamp: msg.timestamp,
                    message: "System metric point acknowledged".to_string(),
                };
                if tx.send(Ok(ack)).await.is_err() {
                    break;
                }
            }
        });

        let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream)))
    }

    type StreamContainerMetricsStream =
        Pin<Box<dyn Stream<Item = Result<MetricsAck, Status>> + Send + 'static>>;

    async fn stream_container_metrics(
        &self,
        request: Request<tonic::Streaming<ContainerMetricsRequest>>,
    ) -> Result<Response<Self::StreamContainerMetricsStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Ok(Some(msg)) = in_stream.message().await {
                info!(
                    "gRPC Container Stream Received - Container: {}, CPU: {:.2}%",
                    msg.container_name, msg.cpu_percent
                );
                let ack = MetricsAck {
                    success: true,
                    received_timestamp: msg.timestamp,
                    message: "Container metric point acknowledged".to_string(),
                };
                if tx.send(Ok(ack)).await.is_err() {
                    break;
                }
            }
        });

        let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream)))
    }

    type StreamLogsStream = Pin<Box<dyn Stream<Item = Result<LogChunk, Status>> + Send + 'static>>;

    async fn stream_logs(
        &self,
        request: Request<LogStreamRequest>,
    ) -> Result<Response<Self::StreamLogsStream>, Status> {
        let msg = request.into_inner();
        let tail_count = if msg.tail_lines > 0 {
            msg.tail_lines as usize
        } else {
            100
        };

        info!(
            "gRPC Log Stream Requested for Container ID: {}, App ID: {}, Tail: {}",
            msg.container_id, msg.application_id, tail_count
        );

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let container_id = msg.container_id.clone();

        tokio::spawn(async move {
            let log_lines = tail_container_logs(&container_id, tail_count);
            for line in log_lines {
                let chunk = LogChunk {
                    container_id: container_id.clone(),
                    log_line: line,
                    timestamp: chrono::Utc::now().timestamp(),
                    is_stderr: false,
                };
                if tx.send(Ok(chunk)).await.is_err() {
                    break;
                }
            }
        });

        let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream)))
    }

    type StreamHealthStream =
        Pin<Box<dyn Stream<Item = Result<HealthAck, Status>> + Send + 'static>>;

    async fn stream_health(
        &self,
        request: Request<tonic::Streaming<HealthReportRequest>>,
    ) -> Result<Response<Self::StreamHealthStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Ok(Some(msg)) = in_stream.message().await {
                info!(
                    "gRPC Health Stream Received - Target: {} ({}), Status: {}",
                    msg.target_id, msg.target_type, msg.status
                );
                let ack = HealthAck {
                    acknowledged: true,
                    timestamp: msg.timestamp,
                };
                if tx.send(Ok(ack)).await.is_err() {
                    break;
                }
            }
        });

        let output_stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(output_stream)))
    }

    async fn get_server_metrics(
        &self,
        request: Request<GetMetricsRequest>,
    ) -> Result<Response<ServerMetricsResponse>, Status> {
        let msg = request.into_inner();
        let limit = if msg.limit <= 0 { 50 } else { msg.limit as i64 };

        let db_metrics = self
            .db
            .get_last_n_server_metrics(limit)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let metrics = db_metrics
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
        let msg = request.into_inner();
        let limit = if msg.limit <= 0 { 50 } else { msg.limit as i64 };

        let db_metrics = self
            .db
            .get_last_n_container_metrics(&msg.app_name, limit)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        let metrics = db_metrics
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
}
