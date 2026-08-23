use std::sync::Arc;

use crate::config::Config;
use crate::docker::api::DockerApi;
use crate::filter::ContainerFilter;
use crate::store::Store;
use auto_di::singleton;

/// Shared application context holding infrastructure components.
#[derive(Clone)]
pub struct MonitorContext {
    pub config: Arc<Config>,
    pub store: Arc<Store>,
    pub docker: DockerApi,
    pub filter: ContainerFilter,
}

impl MonitorContext {
    pub fn from_parts(
        config: Arc<Config>,
        store: Arc<Store>,
        docker: DockerApi,
        filter: ContainerFilter,
    ) -> Self {
        Self {
            config,
            store,
            docker,
            filter,
        }
    }
}

#[singleton]
impl MonitorContext {
    pub fn new(config: Arc<Config>, store: Arc<Store>, docker: Arc<DockerApi>) -> Self {
        Self {
            filter: config.container_filter(),
            config,
            store,
            docker: (*docker).clone(),
        }
    }
}
