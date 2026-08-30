use std::sync::Arc;

use auto_di::singleton;

use crate::services::identity::organization::OrganizationService as AppOrganizationService;

pub struct OrganizationController {
    pub(super) service: Arc<AppOrganizationService>,
}

#[singleton]
impl OrganizationController {
    fn new(service: Arc<AppOrganizationService>) -> Self {
        Self { service }
    }
}
