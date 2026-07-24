use crate::grpc_server::proto::monitoring_service_client::MonitoringServiceClient;
pub use crate::grpc_server::proto::{
    ContainerMetricsRequest, HealthReportRequest, LogChunk, LogStreamRequest, SystemMetricsRequest,
};
use tonic::transport::Channel;
use tracing::{error, info};

pub struct GrpcTelemetryPusher {
    client: Option<MonitoringServiceClient<Channel>>,
}

impl GrpcTelemetryPusher {
    pub async fn connect(grpc_url: &str) -> Self {
        match MonitoringServiceClient::connect(grpc_url.to_string()).await {
            Ok(client) => {
                info!("Connected real-time gRPC stream client to {}", grpc_url);
                Self {
                    client: Some(client),
                }
            }
            Err(err) => {
                error!(
                    "gRPC stream channel connection error to {}: {:?}",
                    grpc_url, err
                );
                Self { client: None }
            }
        }
    }

    pub async fn stream_system_metrics(
        &mut self,
        req: SystemMetricsRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref mut client) = self.client {
            let stream = tokio_stream::iter(vec![req]);
            let _res = client.stream_system_metrics(stream).await?;
            info!("pushed system metrics to real-time gRPC stream channel");
        }
        Ok(())
    }

    pub async fn stream_container_metrics(
        &mut self,
        req: ContainerMetricsRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref mut client) = self.client {
            let stream = tokio_stream::iter(vec![req]);
            let _res = client.stream_container_metrics(stream).await?;
            info!("pushed container metrics to real-time gRPC stream channel");
        }
        Ok(())
    }

    pub async fn stream_logs(
        &mut self,
        req: LogStreamRequest,
    ) -> Result<tonic::Streaming<LogChunk>, Box<dyn std::error::Error + Send + Sync>> {
        let client = self.client.as_mut().ok_or("gRPC client not connected")?;
        let res = client.stream_logs(req).await?;
        Ok(res.into_inner())
    }

    pub async fn stream_health(
        &mut self,
        req: HealthReportRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref mut client) = self.client {
            let stream = tokio_stream::iter(vec![req]);
            let _res = client.stream_health(stream).await?;
            info!("pushed health report to real-time gRPC stream channel");
        }
        Ok(())
    }
}
