use std::sync::Arc;

use auto_di::singleton;

use crate::services::compute::preview::PreviewDeploymentService as AppPreviewService;

pub struct PreviewDeploymentController {
    pub(super) service: Arc<AppPreviewService>,
}

#[singleton]
impl PreviewDeploymentController {
    fn new(service: Arc<AppPreviewService>) -> Self {
        Self { service }
    }
}
