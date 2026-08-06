use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;
use tokio_util::sync::CancellationToken;

use crate::{
    api::dto::networking::RootNetworkStatusDto,
    utils::{
        builder::swarm::{RUSTPLOY_NETWORK, ensure_overlay_network, ensure_swarm_manager},
        docker::DockerCli,
        exec::{CommandExecutor, LocalExecutor},
    },
};

pub struct RootNetworkService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl RootNetworkService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn diagnose(&self, server_id: Option<i64>) -> Result<RootNetworkStatusDto, String> {
        let docker = DockerCli::from_executor(self.executor(server_id).await?);
        match docker.networks().inspect(RUSTPLOY_NETWORK).await {
            Ok(network) => Ok(status_from_network(network, false)),
            Err(error) => Ok(RootNetworkStatusDto {
                name: RUSTPLOY_NETWORK.into(),
                exists: false,
                healthy: false,
                repaired: false,
                driver: None,
                scope: None,
                attachable: None,
                connected_resources: 0,
                issue: Some(format!("network is missing or unreadable: {error}")),
            }),
        }
    }

    pub async fn repair(&self, server_id: Option<i64>) -> Result<RootNetworkStatusDto, String> {
        let executor = self.executor(server_id).await?;
        let docker = DockerCli::from_executor(executor.clone());
        if let Ok(network) = docker.networks().inspect(RUSTPLOY_NETWORK).await {
            let healthy = is_healthy(&network);
            if healthy {
                return Ok(status_from_network(network, false));
            }
            if !network.containers.is_empty() {
                return Err(format!(
                    "cannot repair root network while {} resources are attached",
                    network.containers.len()
                ));
            }
            docker
                .networks()
                .rm(RUSTPLOY_NETWORK)
                .run()
                .await
                .map_err(|error| format!("could not remove invalid root network: {error}"))?;
        }

        let cancel = CancellationToken::new();
        ensure_swarm_manager(&executor, &docker, &cancel)
            .await
            .map_err(|error| error.to_string())?;
        ensure_overlay_network(&docker, RUSTPLOY_NETWORK, &cancel)
            .await
            .map_err(|error| error.to_string())?;
        let network = docker
            .networks()
            .inspect(RUSTPLOY_NETWORK)
            .await
            .map_err(|error| format!("could not verify repaired root network: {error}"))?;
        Ok(status_from_network(network, true))
    }

    async fn executor(&self, server_id: Option<i64>) -> Result<CommandExecutor, String> {
        match server_id {
            Some(id) => crate::services::compose::remote::remote_executor(self.db.as_ref(), id)
                .await
                .map(CommandExecutor::Remote),
            None => Ok(CommandExecutor::Local(LocalExecutor::new())),
        }
    }
}

fn status_from_network(
    network: crate::utils::docker::NetworkInspect,
    repaired: bool,
) -> RootNetworkStatusDto {
    let healthy = is_healthy(&network);
    RootNetworkStatusDto {
        name: network.name,
        exists: true,
        healthy,
        repaired,
        driver: Some(network.driver.clone()),
        scope: Some(network.scope.clone()),
        attachable: Some(network.attachable),
        connected_resources: network.containers.len() as i64,
        issue: (!healthy).then(|| {
            format!(
                "expected encrypted attachable overlay/swarm network, found driver={} scope={} attachable={} encrypted={}",
                network.driver,
                network.scope,
                network.attachable,
                network.options.contains_key("encrypted")
            )
        }),
    }
}

fn is_healthy(network: &crate::utils::docker::NetworkInspect) -> bool {
    network.driver == "overlay"
        && network.scope == "swarm"
        && network.attachable
        && network.options.contains_key("encrypted")
}
