use std::sync::Arc;

use crate::config::Config;
use crate::docker::api::DockerApi;
use crate::filter::ContainerFilter;
use auto_di::singleton;

/// Shared application context holding infrastructure components.
#[derive(Clone)]
pub struct MonitorContext {
    pub config: Arc<Config>,
    pub docker: DockerApi,
    pub filter: ContainerFilter,
}

#[singleton]
impl MonitorContext {
    pub fn new(config: Arc<Config>, docker: Arc<DockerApi>) -> Self {
        Self {
            filter: config.container_filter(),
            config,
            docker: (*docker).clone(),
        }
    }
}

