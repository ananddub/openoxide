use std::sync::Arc;

use auto_di::singleton;

use crate::services::vault::VaultService as AppVaultService;

pub struct VaultController {
    pub(super) service: Arc<AppVaultService>,
}

#[singleton]
impl VaultController {
    fn new(service: Arc<AppVaultService>) -> Self {
        Self { service }
    }
}
