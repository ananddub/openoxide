use std::sync::Arc;

use auto_di::singleton;

use crate::services::infrastructure::terminal::TerminalService as AppTerminalService;

pub struct TerminalController {
    pub(super) service: Arc<AppTerminalService>,
}

#[singleton]
impl TerminalController {
    fn new(service: Arc<AppTerminalService>) -> Self {
        Self { service }
    }
}
