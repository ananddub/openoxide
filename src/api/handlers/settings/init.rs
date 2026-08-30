use std::sync::Arc;

use auto_di::singleton;

use crate::services::platform::settings::SettingsService as AppSettingsService;

pub struct SettingsController {
    pub(super) service: Arc<AppSettingsService>,
}

#[singleton]
impl SettingsController {
    fn new(service: Arc<AppSettingsService>) -> Self {
        Self { service }
    }
}
