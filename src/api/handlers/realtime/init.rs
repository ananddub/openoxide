use std::sync::Arc;

use auto_di::singleton;

use crate::services::platform::realtime::RealtimeService;

pub struct RealtimeController {
    pub(super) service: Arc<RealtimeService>,
}

#[singleton]
impl RealtimeController {
    fn new(service: Arc<RealtimeService>) -> Self {
        Self { service }
    }
}
