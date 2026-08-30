pub use types::{
    ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeStatus, ComposeType,
};

use std::sync::Arc;

use auto_di::singleton;

use crate::core::cache::AppStateCache;
use crate::db::queries::FlashQueries;

pub struct ComposeService {
    pub(super) queries: Arc<FlashQueries>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeService {
    fn new(queries: Arc<FlashQueries>, cache: Arc<AppStateCache>) -> Self {
        Self { queries, cache }
    }
}

pub mod auto_executer;
pub mod crud;
pub mod management;
pub mod operations;
pub mod queries;
pub mod remote;
pub mod source;
pub mod types;
