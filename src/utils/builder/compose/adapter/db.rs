use crate::repository::{ComposeProjectRepository, DomainRepository, MountRepository};
use crate::utils::builder::compose::spec::ComposeSpec;
use auto_di::singleton;
use std::sync::Arc;

use super::mapper::ComposeRowWithRelations;

#[derive(Clone)]
pub struct ComposeSpecAdapter {
    compose_repo: Arc<ComposeProjectRepository>,
    domain_repo: Arc<DomainRepository>,
    mount_repo: Arc<MountRepository>,
}

#[singleton]
impl ComposeSpecAdapter {
    pub fn new(
        compose_repo: Arc<ComposeProjectRepository>,
        domain_repo: Arc<DomainRepository>,
        mount_repo: Arc<MountRepository>,
    ) -> Self {
        Self {
            compose_repo,
            domain_repo,
            mount_repo,
        }
    }

    pub async fn load(&self, compose_id: i64) -> sqlx::Result<ComposeSpec> {
        let compose = self.compose_repo.get_spec_row(compose_id).await?;
        let domains = self.domain_repo.list_by_compose_raw(compose_id).await?;
        let mounts = self.mount_repo.fetch_for_compose(compose_id).await?;

        let data = ComposeRowWithRelations {
            compose,
            domains,
            mounts,
        };
        ComposeSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))
    }
}
