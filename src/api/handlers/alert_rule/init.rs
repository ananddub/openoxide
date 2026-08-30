use std::sync::Arc;

use auto_di::singleton;

use crate::services::{
    integrations::notification::NotificationService, observability::alert::AlertService,
    observability::monitoring::lifecycle::MonitoringLifecycleService,
};

pub struct AlertRuleController {
    pub(super) service: Arc<AlertService>,
    pub(super) notifications: Arc<NotificationService>,
    pub(super) lifecycle: Arc<MonitoringLifecycleService>,
}

#[singleton]
impl AlertRuleController {
    fn new(
        service: Arc<AlertService>,
        notifications: Arc<NotificationService>,
        lifecycle: Arc<MonitoringLifecycleService>,
    ) -> Self {
        Self {
            service,
            notifications,
            lifecycle,
        }
    }
}
