use std::sync::Arc;

use auto_di::singleton;

use crate::services::workspace::environment::EnvironmentService as AppEnvironmentService;

pub struct EnvironmentController {
    pub(super) service: Arc<AppEnvironmentService>,
}

#[singleton]
impl EnvironmentController {
    fn new(service: Arc<AppEnvironmentService>) -> Self {
        Self { service }
    }
}
