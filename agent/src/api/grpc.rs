use auto_grpc::service;

pub use crate::core::middleware::MonitoringMiddleware;
use crate::services::monitoring::MonitoringService as AgentMonitoringService;
use std::sync::Arc;

pub mod proto {
    tonic::include_proto!("monitoring");
}

use proto::monitoring_service_server::MonitoringService;
pub use proto::monitoring_service_server::MonitoringServiceServer;

#[derive(Clone)]
pub struct MonitoringGrpc {
    config: Arc<crate::core::config::Config>,
    service: Arc<AgentMonitoringService>,
}

#[auto_di::singleton]
impl MonitoringGrpc {
    pub fn new(
        config: Arc<crate::core::config::Config>,
        service: Arc<AgentMonitoringService>,
    ) -> Self {
        Self { config, service }
    }

    fn check_server_id(&self, requested: i64) -> Result<(), tonic::Status> {
        if requested == self.config.server_id {
            Ok(())
        } else {
            Err(tonic::Status::not_found(format!(
                "this agent serves server_id {}, not {requested}",
                self.config.server_id
            )))
        }
    }
}

#[service(middleware = [MonitoringMiddleware])]
impl MonitoringService for MonitoringGrpc {
    type StreamLogsStream = crate::services::monitoring::logs::LogStream;
    type WatchContainerStatesStream = crate::services::monitoring::lifecycle::LifecycleStream;
    type ShellStream = crate::services::monitoring::shell::ShellStream;
    type ExecTaskStream = std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<proto::ExecTaskOutput, tonic::Status>>
                + Send
                + 'static,
        >,
    >;

    async fn get_server_metrics(
        &self,
        request: tonic::Request<proto::GetMetricsRequest>,
    ) -> Result<tonic::Response<proto::ServerMetricsResponse>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        self.service.server_metrics(request).await
    }

    async fn get_container_metrics(
        &self,
        request: tonic::Request<proto::GetContainerMetricsRequest>,
    ) -> Result<tonic::Response<proto::ContainerMetricsResponse>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        self.service.container_metrics(request).await
    }

    async fn get_container_states(
        &self,
        request: tonic::Request<proto::GetMetricsRequest>,
    ) -> Result<tonic::Response<proto::ContainerStatesResponse>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        Ok(tonic::Response::new(self.service.container_states().await?))
    }

    async fn watch_container_states(
        &self,
        request: tonic::Request<tonic::Streaming<proto::LifecycleControl>>,
    ) -> Result<tonic::Response<Self::WatchContainerStatesStream>, tonic::Status> {
        self.service
            .watch_container_states(request.into_inner())
            .await
    }

    async fn stream_logs(
        &self,
        request: tonic::Request<proto::LogStreamRequest>,
    ) -> Result<tonic::Response<Self::StreamLogsStream>, tonic::Status> {
        Ok(self.service.stream_logs(request.into_inner()))
    }

    async fn shell(
        &self,
        request: tonic::Request<tonic::Streaming<proto::ShellMessage>>,
    ) -> Result<tonic::Response<Self::ShellStream>, tonic::Status> {
        self.service
            .shell(self.config.server_id, request.into_inner())
            .await
    }

    async fn exec_task(
        &self,
        request: tonic::Request<proto::ExecTaskRequest>,
    ) -> Result<tonic::Response<Self::ExecTaskStream>, tonic::Status> {
        let stream = crate::services::monitoring::shell::ShellService::execute(
            self.config.server_id,
            request.into_inner(),
        )
        .await?;
        Ok(tonic::Response::new(Box::pin(stream)))
    }
}
