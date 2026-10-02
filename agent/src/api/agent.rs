use auto_grpc::service;

pub use crate::core::middleware::AgentMiddleware;
use crate::services::file_transfer::FileTransferService;
use crate::services::monitoring::MonitoringService as AgentMonitoringService;
use std::sync::Arc;

pub mod proto {
    tonic::include_proto!("agent");
}

use proto::agent_service_server::AgentService;
pub use proto::agent_service_server::AgentServiceServer;

#[derive(Clone)]
pub struct AgentGrpc {
    config: Arc<crate::core::config::Config>,
    service: Arc<AgentMonitoringService>,
    file_transfer: Arc<FileTransferService>,
    docker: Arc<crate::docker::api::DockerApi>,
}

#[auto_di::singleton]
impl AgentGrpc {
    pub fn new(
        config: Arc<crate::core::config::Config>,
        service: Arc<AgentMonitoringService>,
        file_transfer: Arc<FileTransferService>,
        docker: Arc<crate::docker::api::DockerApi>,
    ) -> Self {
        Self {
            config,
            service,
            file_transfer,
            docker,
        }
    }

    fn check_server_id(&self, requested: i64) -> Result<(), tonic::Status> {
        if requested > 0 && (self.config.server_id == -1 || requested == self.config.server_id) {
            Ok(())
        } else {
            Err(tonic::Status::not_found(format!(
                "this agent serves server_id {}, not {requested}",
                self.config.server_id
            )))
        }
    }
}

#[service(middleware = [AgentMiddleware])]
impl AgentService for AgentGrpc {
    type WatchContainerStatesStream = crate::services::monitoring::lifecycle::LifecycleStream;
    type ShellStream = crate::services::monitoring::shell::ShellStream;
    type ExecTaskStream = std::pin::Pin<
        Box<
            dyn futures::Stream<Item = Result<proto::ExecTaskOutput, tonic::Status>>
                + Send
                + 'static,
        >,
    >;
    type StreamContainerStatsStream = crate::services::monitoring::stats::StatsStream;

    async fn get_server_metrics(
        &self,
        request: tonic::Request<proto::GetMetricsRequest>,
    ) -> Result<tonic::Response<proto::ServerMetricsResponse>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        self.service.server_metrics(request).await
    }

    async fn get_agent_info(
        &self,
        request: tonic::Request<proto::AgentInfoRequest>,
    ) -> Result<tonic::Response<proto::AgentInfoResponse>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        let mut info = self.file_transfer.agent_info();
        crate::services::update_probe::fill(&self.docker, &mut info, &request.update_image).await;
        Ok(tonic::Response::new(info))
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
        request: tonic::Request<proto::WatchRequest>,
    ) -> Result<tonic::Response<Self::WatchContainerStatesStream>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        self.service.watch_container_states(request).await
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

    async fn stream_container_stats(
        &self,
        request: tonic::Request<proto::StreamStatsRequest>,
    ) -> Result<tonic::Response<Self::StreamContainerStatsStream>, tonic::Status> {
        let request = request.into_inner();
        self.check_server_id(request.server_id)?;
        self.service.stream_container_stats(request).await
    }

    async fn upload_file(
        &self,
        request: tonic::Request<tonic::Streaming<proto::FileUploadMessage>>,
    ) -> Result<tonic::Response<proto::FileUploadResult>, tonic::Status> {
        Ok(tonic::Response::new(
            self.file_transfer.upload(request.into_inner()).await?,
        ))
    }
}
