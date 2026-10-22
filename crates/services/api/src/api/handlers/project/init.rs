use std::sync::Arc;

use auto_di::singleton;

use crate::core::cache::AppStateCache;
use crate::services::workspace::project::ProjectService as AppProjectService;

pub struct ProjectController {
    pub(super) service: Arc<AppProjectService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl ProjectController {
    fn new(service: Arc<AppProjectService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}
