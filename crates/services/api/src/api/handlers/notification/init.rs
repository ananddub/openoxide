use std::sync::Arc;

use auto_di::singleton;

use crate::db::queries::FlashQueries;
use crate::services::integrations::notification::NotificationService;

use super::builder::NotificationProviderBuilder;

pub struct NotificationController {
    pub(super) queries: Arc<FlashQueries>,
    pub(super) service: Arc<NotificationService>,
    pub(super) builder: NotificationProviderBuilder,
}

#[singleton]
impl NotificationController {
    fn new(queries: Arc<FlashQueries>, service: Arc<NotificationService>) -> Self {
        let builder = NotificationProviderBuilder;
        Self {
            queries,
            service,
            builder,
        }
    }
}
