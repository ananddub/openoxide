use crate::repository::{ComposeProjectRepository, DomainRepository, MountRepository};
use crate::utils::builder::compose::spec::{ComposeServiceNetworkSpec, ComposeSpec};
use auto_di::singleton;
use serde::Deserialize;
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
        let service_networks = resolve_service_networks(&compose.service_networks).await?;
        let domains = self.domain_repo.list_by_compose_raw(compose_id).await?;
        let mounts = self.mount_repo.fetch_for_compose(compose_id).await?;

        let data = ComposeRowWithRelations {
            compose,
            domains,
            mounts,
            service_networks,
        };
        ComposeSpec::try_from(data).map_err(|e| sqlx::Error::Protocol(e.to_string()))
    }
}

#[derive(Deserialize)]
struct RawComposeServiceNetwork {
    service_name: String,
    network_ids: Vec<String>,
    detach_rustploy_network: i64,
}

async fn resolve_service_networks(raw: &str) -> sqlx::Result<Vec<ComposeServiceNetworkSpec>> {
    let parsed = serde_json::from_str::<Vec<RawComposeServiceNetwork>>(raw).unwrap_or_default();
    let mut out = Vec::with_capacity(parsed.len());
    for item in parsed {
        let network_ids = serde_json::to_string(&item.network_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let (networks, _) = crate::utils::builder::database::builder::resolve_database_networks(
            Some(&network_ids),
            item.detach_rustploy_network,
        )
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        out.push(ComposeServiceNetworkSpec {
            service_name: item.service_name,
            networks,
        });
    }
    Ok(out)
}
