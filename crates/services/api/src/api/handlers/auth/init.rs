use std::sync::Arc;

use auto_di::singleton;

use crate::services::identity::auth::AuthService as AppAuthService;

pub struct AuthController {
    pub(super) service: Arc<AppAuthService>,
}

#[singleton]
impl AuthController {
    fn new(service: Arc<AppAuthService>) -> Self {
        Self { service }
    }
}
