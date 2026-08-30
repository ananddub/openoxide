use std::sync::Arc;

use auto_di::singleton;

use crate::core::cache::AppStateCache;
use crate::services::domain::DomainService as AppDomainService;

pub struct DomainController {
    pub(super) service: Arc<AppDomainService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl DomainController {
    fn new(service: Arc<AppDomainService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}
