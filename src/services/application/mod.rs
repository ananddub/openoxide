pub use management::ApplicationCleanupResult;
pub use types::{ApplicationOperation, ApplicationOperationResult, ApplicationRecord};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::core::cache::AppStateCache;
use crate::{
    repository::{
        ApplicationRepository, DeploymentRepository, ResourceDependencyRepository,
        RollbackRepository,
    },
    services::deployment::DeploymentService,
};

pub struct ApplicationService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) repo_app: Arc<ApplicationRepository>,
    pub(super) repo_deploy: Arc<DeploymentRepository>,
    pub(super) repo_rollback: Arc<RollbackRepository>,
    pub(super) repo_dependencies: Arc<ResourceDependencyRepository>,
    pub(super) deployment_service: Arc<DeploymentService>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl ApplicationService {
    fn new(
        db: Arc<SqlitePool>,
        repo_app: Arc<ApplicationRepository>,
        repo_deploy: Arc<DeploymentRepository>,
        repo_rollback: Arc<RollbackRepository>,
        repo_dependencies: Arc<ResourceDependencyRepository>,
        deployment_service: Arc<DeploymentService>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            db,
            repo_app,
            repo_deploy,
            repo_rollback,
            repo_dependencies,
            deployment_service,
            cache,
        }
    }
}

pub mod auto_excuter;
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
