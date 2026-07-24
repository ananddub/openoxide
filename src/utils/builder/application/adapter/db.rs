use crate::repository::{ApplicationRepository, DomainRepository, MountRepository};
use crate::utils::builder::spec::ApplicationSpec;
use auto_di::singleton;
use std::sync::Arc;

use super::mapper::AppRowWithRelations;

#[derive(Clone)]
pub struct ApplicationSpecAdapter {
    app_repo: Arc<ApplicationRepository>,
    domain_repo: Arc<DomainRepository>,
    mount_repo: Arc<MountRepository>,
}

#[singleton]
impl ApplicationSpecAdapter {
    pub fn new(
        app_repo: Arc<ApplicationRepository>,
        domain_repo: Arc<DomainRepository>,
        mount_repo: Arc<MountRepository>,
    ) -> Self {
        Self {
            app_repo,
            domain_repo,
            mount_repo,
        }
    }

    pub async fn load(&self, application_id: i64) -> sqlx::Result<ApplicationSpec> {
        let app = self.app_repo.get_spec_row(application_id).await?;
        let domains = self
            .domain_repo
            .list_by_application_raw(application_id)
            .await?;
        let mounts = self
            .mount_repo
            .fetch_for_application(application_id)
            .await?;

        let data = AppRowWithRelations {
            app,
            domains,
            mounts,
        };
        ApplicationSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))
    }
}
