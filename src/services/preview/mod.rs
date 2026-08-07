mod cleanup;
mod factory;
mod lifecycle;
mod queries;
mod types;

use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::repository::{ApplicationRepository, DomainRepository, PreviewDeploymentRepository};

pub use types::{PreviewDeploymentOutcome, PreviewDeploymentView, PreviewLifecycleOutcome};

pub struct PreviewDeploymentService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) previews: Arc<PreviewDeploymentRepository>,
    pub(super) applications: Arc<ApplicationRepository>,
    pub(super) domains: Arc<DomainRepository>,
    pub(super) lifecycle_lock: tokio::sync::Mutex<()>,
}

#[singleton]
impl PreviewDeploymentService {
    fn new(
        db: Arc<SqlitePool>,
        previews: Arc<PreviewDeploymentRepository>,
        applications: Arc<ApplicationRepository>,
        domains: Arc<DomainRepository>,
    ) -> Self {
        Self {
            db,
            previews,
            applications,
            domains,
            lifecycle_lock: tokio::sync::Mutex::new(()),
        }
    }
}
