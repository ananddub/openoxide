use std::sync::Arc;

use auto_di::singleton;

use crate::core::cache::AppStateCache;
use crate::services::registry::RegistryService as AppRegistryService;

pub struct RegistryController {
    pub(super) service: Arc<AppRegistryService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl RegistryController {
    fn new(service: Arc<AppRegistryService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}
