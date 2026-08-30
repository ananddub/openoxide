use std::sync::Arc;

use auto_di::singleton;

use crate::services::observability::monitoring::{
    agent_auth::MonitoringAgentAuth, lifecycle::MonitoringLifecycleService,
    monitoring_service::MonitoringService,
};

pub struct MonitoringController {
    pub(super) service: Arc<MonitoringService>,
    pub(super) agent_auth: Arc<MonitoringAgentAuth>,
    pub(super) lifecycle: Arc<MonitoringLifecycleService>,
}

#[singleton]
impl MonitoringController {
    fn new(
        service: Arc<MonitoringService>,
        agent_auth: Arc<MonitoringAgentAuth>,
        lifecycle: Arc<MonitoringLifecycleService>,
    ) -> Self {
        Self {
            service,
            agent_auth,
            lifecycle,
        }
    }
}
