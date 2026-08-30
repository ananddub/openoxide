use std::sync::Arc;

use auto_di::singleton;

use crate::services::database_network::DatabaseNetworkService as AppDatabaseNetworkService;

pub struct DatabaseNetworkController {
    pub(super) service: Arc<AppDatabaseNetworkService>,
}

#[singleton]
impl DatabaseNetworkController {
    fn new(service: Arc<AppDatabaseNetworkService>) -> Self {
        Self { service }
    }
}
