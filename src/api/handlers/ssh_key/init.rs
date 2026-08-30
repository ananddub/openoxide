use std::sync::Arc;

use auto_di::singleton;

use crate::services::ssh_key::SshKeyService as AppSshKeyService;

pub struct SshKeyController {
    pub(super) service: Arc<AppSshKeyService>,
}

#[singleton]
impl SshKeyController {
    fn new(service: Arc<AppSshKeyService>) -> Self {
        Self { service }
    }
}
