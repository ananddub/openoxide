use std::sync::Arc;

use auto_di::singleton;

use crate::services::identity::auth::AuthService as AppAuthService;
use crate::services::identity::auth::rate_limit::AuthRateLimiter;

pub struct AuthController {
    pub(super) service: Arc<AppAuthService>,
    pub(super) limiter: Arc<AuthRateLimiter>,
}

#[singleton]
impl AuthController {
    fn new(service: Arc<AppAuthService>, limiter: Arc<AuthRateLimiter>) -> Self {
        Self { service, limiter }
    }
}
