use std::sync::Arc;

use auto_di::singleton;

use crate::services::identity::permission::PermissionGroupService as AppPermissionGroupService;

pub struct PermissionGroupController {
    pub(super) service: Arc<AppPermissionGroupService>,
}

#[singleton]
impl PermissionGroupController {
    fn new(service: Arc<AppPermissionGroupService>) -> Self {
        Self { service }
    }
}
