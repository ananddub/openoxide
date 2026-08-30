use std::sync::Arc;

use auto_di::singleton;

use crate::services::ai::AiService as AppAiService;

pub struct AiController {
    pub(super) service: Arc<AppAiService>,
}

#[singleton]
impl AiController {
    fn new(service: Arc<AppAiService>) -> Self {
        Self { service }
    }
}
