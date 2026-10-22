use std::sync::Arc;

use auto_di::singleton;

use crate::services::platform::global_ops::DockerManagementService;

pub struct DockerManagementController {
    pub(super) service: Arc<DockerManagementService>,
}

#[singleton]
impl DockerManagementController {
    fn new(service: Arc<DockerManagementService>) -> Self {
        Self { service }
    }
}
