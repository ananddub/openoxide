use crate::flash_gen::db::Queries;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::{ops::Deref, sync::Arc};

/// Application-owned DI wrapper around the generated FlashORM query client.
pub struct FlashQueries(Queries);

#[singleton]
impl FlashQueries {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(Queries::new(pool.as_ref().clone()))
    }
}

impl Deref for FlashQueries {
    type Target = Queries;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
