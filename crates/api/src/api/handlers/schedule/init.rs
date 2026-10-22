use std::sync::Arc;

use auto_di::singleton;

use crate::core::cache::AppStateCache;
use crate::services::automation::schedule::ScheduleService;

pub struct ScheduleController {
    pub(super) service: Arc<ScheduleService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl ScheduleController {
    fn new(service: Arc<ScheduleService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}
