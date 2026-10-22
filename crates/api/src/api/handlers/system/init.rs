use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::core::config::Config;

pub struct SystemController {
    pub(super) db: Arc<SqlitePool>,
    pub(super) config: Arc<Config>,
}

#[singleton]
impl SystemController {
    fn new(db: Arc<SqlitePool>, config: Arc<Config>) -> Self {
        Self { db, config }
    }
}
