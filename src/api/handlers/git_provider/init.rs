use std::sync::Arc;

use auto_di::singleton;

use crate::services::git_provider::GitProviderService as AppGitProviderService;

pub struct GitProviderController {
    pub(super) service: Arc<AppGitProviderService>,
}

#[singleton]
impl GitProviderController {
    fn new(service: Arc<AppGitProviderService>) -> Self {
        Self { service }
    }
}
