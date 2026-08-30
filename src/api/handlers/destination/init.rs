use std::sync::Arc;

use auto_di::singleton;

use crate::core::cache::AppStateCache;
use crate::services::destination::DestinationService as AppDestinationService;

pub struct DestinationController {
    pub(super) service: Arc<AppDestinationService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl DestinationController {
    fn new(service: Arc<AppDestinationService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}
