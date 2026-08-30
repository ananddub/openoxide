use auto_di::singleton;
use std::sync::Arc;

use crate::core::cache::AppStateCache;
use crate::db::queries::FlashQueries;

pub struct DatabaseService {
    pub(super) queries: Arc<FlashQueries>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl DatabaseService {
    fn new(queries: Arc<FlashQueries>, cache: Arc<AppStateCache>) -> Self {
        Self { queries, cache }
    }
}

pub use queries::ResourceDependencyCounts;
pub use types::*;

pub mod crud;
pub mod management;
pub mod mount;
pub mod operations;
pub mod queries;
pub mod swarm_settings;
pub mod types;
