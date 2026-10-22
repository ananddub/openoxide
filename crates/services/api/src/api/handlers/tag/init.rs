use std::sync::Arc;

use auto_di::singleton;

use crate::services::workspace::tag::TagService as AppTagService;

pub struct TagController {
    pub(super) service: Arc<AppTagService>,
}

#[singleton]
impl TagController {
    fn new(service: Arc<AppTagService>) -> Self {
        Self { service }
    }
}
