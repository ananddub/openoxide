use std::sync::Arc;

use auto_di::singleton;

use crate::services::overview::OverviewService;

pub struct OverviewController {
    pub(super) service: Arc<OverviewService>,
}

#[singleton]
impl OverviewController {
    fn new(service: Arc<OverviewService>) -> Self {
        Self { service }
    }
}
