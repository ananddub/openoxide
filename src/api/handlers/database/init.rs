use std::sync::Arc;

use auto_di::singleton;

use crate::services::compute::database::DatabaseService as AppDatabaseService;

pub struct DatabaseController {
    pub(super) service: Arc<AppDatabaseService>,
    pub(super) mounts: Arc<crate::services::compute::database::mount::DatabaseMountService>,
    pub(super) swarm:
        Arc<crate::services::compute::database::swarm_settings::DatabaseSwarmSettingsService>,
}

#[singleton]
impl DatabaseController {
    fn new(
        service: Arc<AppDatabaseService>,
        mounts: Arc<crate::services::compute::database::mount::DatabaseMountService>,
        swarm: Arc<
            crate::services::compute::database::swarm_settings::DatabaseSwarmSettingsService,
        >,
    ) -> Self {
        Self {
            service,
            mounts,
            swarm,
        }
    }
}
