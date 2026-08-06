pub use types::{
    ComposeOperation, ComposeOperationResult, ComposeRecord, ComposeStatus, ComposeType,
};

use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::core::cache::AppStateCache;
use crate::repository::{
    ComposeProjectRepository, DeploymentRepository, ResourceDependencyRepository,
};

pub struct ComposeService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) repo_compose: Arc<ComposeProjectRepository>,
    pub(super) repo_deploy: Arc<DeploymentRepository>,
    pub(super) repo_dependencies: Arc<ResourceDependencyRepository>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeService {
    fn new(
        db: Arc<SqlitePool>,
        repo_compose: Arc<ComposeProjectRepository>,
        repo_deploy: Arc<DeploymentRepository>,
        repo_dependencies: Arc<ResourceDependencyRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            db,
            repo_compose,
            repo_deploy,
            repo_dependencies,
            cache,
        }
    }
}

pub mod auto_excuter;
pub mod crud;
pub mod management;
pub mod operations;
pub mod queries;
pub mod remote;
pub mod source;
pub mod types;
