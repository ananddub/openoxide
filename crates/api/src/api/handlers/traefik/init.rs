use std::sync::Arc;

use auto_di::singleton;

use crate::services::networking::traefik::TraefikService;

pub struct TraefikController {
    pub(super) service: Arc<TraefikService>,
}

#[singleton]
impl TraefikController {
    fn new(service: Arc<TraefikService>) -> Self {
        Self { service }
    }
}
