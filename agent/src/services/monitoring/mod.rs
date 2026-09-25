use std::sync::Arc;

use auto_di::singleton;
use tokio::sync::Mutex;

use crate::{docker::api::DockerApi, telemetry::collector::system::SystemCollector};

pub mod hub;
pub mod lifecycle;
pub mod metrics;
pub mod shell;
pub mod state;
pub mod stats;

/// Agent-side service facade. Transport handlers delegate here so Docker,
/// metrics, lifecycle and shell dependencies stay in one owned service.
/// Host metrics are sampled live through the retained `SystemCollector`
/// (sysinfo needs two CPU refreshes over time, so it is stateful); container
/// data comes from the shared hubs: one stats feed and one events feed serve
/// every listener, so the agent holds one Docker connection per feed no
/// matter how many panel consumers watch.
#[derive(Clone)]
pub struct MonitoringService {
    pub docker: Arc<DockerApi>,
    pub system: Arc<Mutex<SystemCollector>>,
    pub stats_hub: Arc<hub::StatsHub>,
    pub events_hub: Arc<hub::EventsHub>,
}

#[singleton]
impl MonitoringService {
    pub fn new(
        docker: Arc<DockerApi>,
        stats_hub: Arc<hub::StatsHub>,
        events_hub: Arc<hub::EventsHub>,
    ) -> Self {
        Self {
            docker,
            system: Arc::new(Mutex::new(SystemCollector::new())),
            stats_hub,
            events_hub,
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
        lifecycle::LifecycleService::watch(self.docker.clone(), (*self.events_hub).clone(), request)
            .await
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
        stats::StatsService::stream((*self.stats_hub).clone(), request).await
    }
}
