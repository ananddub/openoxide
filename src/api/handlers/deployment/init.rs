use std::sync::Arc;

use auto_di::singleton;

use crate::services::compute::engine::DeploymentService as AppDeploymentService;

pub struct DeploymentController {
    pub(super) service: Arc<AppDeploymentService>,
}

#[singleton]
impl DeploymentController {
    fn new(service: Arc<AppDeploymentService>) -> Self {
        Self { service }
    }
}
