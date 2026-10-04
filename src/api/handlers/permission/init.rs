use std::sync::Arc;

use auto_di::singleton;

use crate::services::identity::permission::{
    PermissionGroupService as AppPermissionGroupService, PermissionService,
};

pub struct PermissionGroupController {
    pub(super) service: Arc<AppPermissionGroupService>,
    pub(super) permissions: Arc<PermissionService>,
}

#[singleton]
impl PermissionGroupController {
    fn new(service: Arc<AppPermissionGroupService>, permissions: Arc<PermissionService>) -> Self {
        Self {
            service,
            permissions,
        }
    }
}
