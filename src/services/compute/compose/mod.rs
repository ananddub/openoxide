pub use types::{
    ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeStatus, ComposeType,
};

// The former `remote` file module was merged into `queries`; this alias keeps
// the historical `compose::remote::…` import paths working for the call sites
// across the crate.
pub use queries as remote;

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

pub mod crud;
pub mod management;
pub mod operations;
pub mod queries;
pub mod source;
pub mod types;
