use std::sync::Arc;

use auto_di::singleton;

use crate::db::queries::FlashQueries;

pub struct AuditController {
    pub(super) queries: Arc<FlashQueries>,
}

#[singleton]
impl AuditController {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }
}
