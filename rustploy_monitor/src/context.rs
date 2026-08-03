use std::sync::Arc;
use std::time::Duration;

use crate::config::Config;
use crate::docker::api::DockerApi;
use crate::filter::ContainerFilter;
use crate::store::Store;

/// Shared application context holding infrastructure components.
#[derive(Clone)]
pub struct MonitorContext {
    pub config: Arc<Config>,
    pub store: Arc<Store>,
    pub docker: DockerApi,
    pub filter: ContainerFilter,
    pub http_client: reqwest::Client,
}

impl MonitorContext {
    pub fn new(
        config: Arc<Config>,
        store: Arc<Store>,
        docker: DockerApi,
        filter: ContainerFilter,
    ) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(10)
            .build()
            .unwrap_or_default();

        Self {
            config,
            store,
            docker,
            filter,
            http_client,
        }
    }
}
