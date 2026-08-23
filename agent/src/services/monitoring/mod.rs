use std::sync::Arc;

use crate::{docker::api::DockerApi, store::Store};
use auto_di::singleton;

pub mod lifecycle;
pub mod logs;
pub mod metrics;
pub mod shell;
pub mod state;

/// Agent-side service facade. Transport handlers delegate here so Docker,
/// metrics, lifecycle and shell dependencies stay in one owned service.
#[derive(Clone)]
pub struct MonitoringService {
    pub store: Arc<Store>,
    pub docker: Arc<DockerApi>,
}

#[singleton]
impl MonitoringService {
    pub fn new(store: Arc<Store>, docker: Arc<DockerApi>) -> Self {
        Self { store, docker }
    }

    pub async fn server_metrics(
        &self,
        request: crate::api::proto::GetMetricsRequest,
    ) -> Result<tonic::Response<crate::api::proto::ServerMetricsResponse>, tonic::Status> {
        metrics::MetricsService::server(&self.store, request).await
    }

    pub async fn container_metrics(
        &self,
        request: crate::api::proto::GetContainerMetricsRequest,
    ) -> Result<tonic::Response<crate::api::proto::ContainerMetricsResponse>, tonic::Status> {
        metrics::MetricsService::containers(&self.store, request).await
    }

    pub async fn container_states(
        &self,
    ) -> Result<crate::api::proto::ContainerStatesResponse, tonic::Status> {
        state::StateService::current(&self.docker).await
    }

    pub async fn watch_container_states(
        &self,
        controls: tonic::Streaming<crate::api::proto::LifecycleControl>,
    ) -> Result<tonic::Response<lifecycle::LifecycleStream>, tonic::Status> {
        lifecycle::LifecycleService::watch((*self.docker).clone(), controls).await
    }

    pub fn stream_logs(
        &self,
        request: crate::api::proto::LogStreamRequest,
    ) -> tonic::Response<logs::LogStream> {
        logs::LogsService::stream((*self.docker).clone(), request)
    }

    pub async fn shell(
        &self,
        server_id: i64,
        input: tonic::Streaming<crate::api::proto::ShellMessage>,
    ) -> Result<tonic::Response<shell::ShellStream>, tonic::Status> {
        shell::ShellService::open(server_id, input, server_id, (*self.docker).clone()).await
    }
}
