mod cleanup;
mod factory;
mod lifecycle;
mod queries;
mod types;

use std::sync::Arc;

use auto_di::singleton;

use crate::db::queries::FlashQueries;

pub use types::{PreviewDeploymentOutcome, PreviewDeploymentView, PreviewLifecycleOutcome};

pub struct PreviewDeploymentService {
    pub(super) queries: Arc<FlashQueries>,
    pub(super) lifecycle_lock: tokio::sync::Mutex<()>,
}

#[singleton]
impl PreviewDeploymentService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self {
            queries,
            lifecycle_lock: tokio::sync::Mutex::new(()),
        }
    }
}
