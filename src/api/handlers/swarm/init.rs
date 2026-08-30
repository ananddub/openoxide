use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode};
use sqlx::SqlitePool;

use crate::core::cache::{AppStateCache, CacheKey};
use crate::services::server::RemoteServerService;
use crate::utils::docker::DockerCli;

pub struct SwarmController {
    pub(super) db: Arc<SqlitePool>,
    pub(super) cache: Arc<AppStateCache>,
    pub(super) servers: Arc<RemoteServerService>,
}

#[singleton]
impl SwarmController {
    fn new(
        db: Arc<SqlitePool>,
        cache: Arc<AppStateCache>,
        servers: Arc<RemoteServerService>,
    ) -> Self {
        Self { db, cache, servers }
    }
}

// Controller extensions — non-gRPC helper methods live here, not in controller.rs.
impl SwarmController {
    pub(super) async fn remove_stale_hostname_duplicates(
        &self,
        manager_docker: &DockerCli,
        target_docker: &DockerCli,
        target_hostname: &str,
    ) {
        let Ok(new_swarm_info) = target_docker.swarm().inspect().await else {
            return;
        };
        let new_node_id = new_swarm_info.node_id;

        let Ok(nodes) = manager_docker.nodes().list().run_json().await else {
            return;
        };
        for n in nodes {
            if n.hostname == target_hostname
                && n.id != new_node_id
                && n.status.to_lowercase() == "down"
            {
                let _ = manager_docker.nodes().remove(n.id).force().run().await;
            }
        }
    }

    pub(super) async fn invalidate_swarm(&self, server_id: Option<i64>) {
        self.cache.invalidate(&CacheKey::SwarmInfo(server_id)).await;
        self.cache.invalidate(&CacheKey::SwarmNodes(server_id)).await;
    }

    pub(super) async fn executor_for(
        &self,
        server_id: Option<i64>,
    ) -> Result<crate::utils::exec::CommandExecutor, ConnectError> {
        crate::services::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(|error| ConnectError::new(ErrorCode::FailedPrecondition, error))
    }

    pub(super) async fn docker(&self, server_id: Option<i64>) -> Result<DockerCli, ConnectError> {
        Ok(DockerCli::from_executor(self.executor_for(server_id).await?))
    }
}
