pub use management::ApplicationCleanupResult;
pub use types::{ApplicationOperation, ApplicationOperationResult, ApplicationRecord};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::core::cache::AppStateCache;
use crate::{
    db::queries::FlashQueries,
    services::compute::engine::DeploymentService,
};

pub struct ApplicationService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) queries: Arc<FlashQueries>,
    pub(super) deployment_service: Arc<DeploymentService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl ApplicationService {
    fn new(
        db: Arc<SqlitePool>,
        queries: Arc<FlashQueries>,
        deployment_service: Arc<DeploymentService>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            db,
            queries,
            deployment_service,
            cache,
        }
    }
}

pub mod auto_executer;
pub mod config;
pub mod crud;
pub mod import_export;
pub mod management;
pub mod middleware;
pub mod mount;
pub mod network;
pub mod operations;
pub mod patch;
pub mod port;
pub mod queries;
pub mod redirect;
pub mod remote;
pub mod rollback;
pub mod security;
pub mod source;
pub mod types;
