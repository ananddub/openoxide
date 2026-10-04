use std::sync::Arc;

use auto_di::singleton;
use tokio::sync::Mutex;

use crate::{docker::api::DockerApi, telemetry::collector::system::SystemCollector};

pub mod cgroup;
pub mod lifecycle;
pub mod metrics;
pub mod shell;
pub mod state;
pub mod stats;

/// Agent-side service facade. Transport handlers delegate here so Docker,
/// metrics, lifecycle and shell dependencies stay in one owned service.
/// Nothing is stored or buffered between calls: every request reads its
/// values live from Docker, procfs and sysinfo at the moment it arrives, so
/// an answer may lag by the measurement time itself but is always the
/// current one. The `Mutex` around the host collector only serialises
/// measurements (two overlapping CPU windows would corrupt each other); the
/// collector itself carries no data from one sample to the next.
#[derive(Clone)]
pub struct MonitoringService {
    pub docker: Arc<DockerApi>,
    pub system: Arc<Mutex<SystemCollector>>,
}

#[singleton]
impl MonitoringService {
    pub fn new(docker: Arc<DockerApi>) -> Self {
        Self {
            docker,
            system: Arc::new(Mutex::new(SystemCollector::new())),
        }
    }

    pub async fn server_metrics(
        &self,
        request: crate::api::proto::GetMetricsRequest,
    ) -> Result<tonic::Response<crate::api::proto::ServerMetricsResponse>, tonic::Status> {
        metrics::MetricsService::server(&self.system, request).await
    }

    pub async fn container_metrics(
        &self,
        request: crate::api::proto::GetContainerMetricsRequest,
    ) -> Result<tonic::Response<crate::api::proto::ContainerMetricsResponse>, tonic::Status> {
        metrics::MetricsService::containers(&self.docker, request).await
    }

    pub async fn container_states(
        &self,
    ) -> Result<crate::api::proto::ContainerStatesResponse, tonic::Status> {
        state::StateService::current(&self.docker).await
    }

    pub async fn watch_container_states(
        &self,
        request: crate::api::proto::WatchRequest,
    ) -> Result<tonic::Response<lifecycle::LifecycleStream>, tonic::Status> {
        lifecycle::LifecycleService::watch(self.docker.clone(), request).await
    }

    pub async fn shell(
        &self,
        server_id: i64,
        input: tonic::Streaming<crate::api::proto::ShellMessage>,
    ) -> Result<tonic::Response<shell::ShellStream>, tonic::Status> {
        shell::ShellService::open(server_id, input, server_id, (*self.docker).clone()).await
    }

    pub async fn stream_container_stats(
        &self,
        request: crate::api::proto::StreamStatsRequest,
    ) -> Result<tonic::Response<stats::StatsStream>, tonic::Status> {
        stats::StatsService::stream(self.docker.clone(), request).await
    }
}
