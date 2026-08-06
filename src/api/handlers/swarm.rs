use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};
use sqlx::SqlitePool;

use crate::core::cache::{AppStateCache, CacheEnum, CacheKey};
use crate::utils::docker::core::types::{NodeAvailability, SwarmRole};
use crate::{
    api::dto::swarm::{
        NodeActionDto, NodeAvailabilityDto, NodeDto, SwarmConnectionDto, SwarmInfoDto,
        SwarmJoinDto, SwarmTokensDto,
    },
    services::remote_server::ServerService,
    utils::{
        docker::DockerCli,
        exec::{
            CommandExecutor, ExecError, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey,
            detect_advertise_addr,
        },
        jwt::claim::Claims,
    },
};

type ApiError = (StatusCode, String);

pub struct SwarmController {
    db: Arc<SqlitePool>,
    cache: Arc<AppStateCache>,
    servers: Arc<ServerService>,
}

#[controller("/swarm")]
impl SwarmController {
    fn new(db: Arc<SqlitePool>, cache: Arc<AppStateCache>, servers: Arc<ServerService>) -> Self {
        Self { db, cache, servers }
    }

    #[post("/info")]
    async fn info(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<SwarmInfoDto>, ApiError> {
        let key = CacheKey::SwarmInfo(body.server_id);
        let cached = self
            .cache
            .try_get_with(key, async {
                let docker = self.docker(body.server_id).await?;
                let swarm = docker.swarm().inspect().await.map_err(map_exec)?;
                Ok::<_, ApiError>(CacheEnum::SwarmInfo(swarm))
            })
            .await
            .map_err(|e| (*e).clone())?;

        let CacheEnum::SwarmInfo(swarm) = cached else {
            unreachable!("SwarmInfo key always yields a SwarmInfo cache entry")
        };
        Ok(Json(SwarmInfoDto {
            node_id: swarm.node_id,
            node_addr: swarm.node_addr,
            local_node_state: swarm.local_node_state,
            control_available: swarm.control_available,
            nodes: swarm.nodes as i64,
            managers: swarm.managers as i64,
        }))
    }

    #[post("/tokens")]
    async fn tokens(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<SwarmTokensDto>, ApiError> {
        let docker = self.docker(body.server_id).await?;

        let worker = docker
            .swarm()
            .join_token()
            .get(SwarmRole::Worker)
            .await
            .map_err(map_exec)?;
        let manager = docker
            .swarm()
            .join_token()
            .get(SwarmRole::Manager)
            .await
            .map_err(map_exec)?;

        Ok(Json(SwarmTokensDto { worker, manager }))
    }

    #[post("/nodes")]
    async fn nodes(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<Vec<NodeDto>>, ApiError> {
        let key = CacheKey::SwarmNodes(body.server_id);
        let cached = self
            .cache
            .try_get_with(key, async {
                let docker = self.docker(body.server_id).await?;
                let summaries = docker.nodes().list().run_json().await.map_err(map_exec)?;
                let ids: Vec<String> = summaries.into_iter().map(|s| s.id).collect();
                let nodes = docker.nodes().inspect_all(&ids).await.map_err(map_exec)?;
                Ok::<_, ApiError>(CacheEnum::SwarmNodes(nodes))
            })
            .await
            .map_err(|e| (*e).clone())?;

        let CacheEnum::SwarmNodes(nodes) = cached else {
            unreachable!("SwarmNodes key always yields a SwarmNodes cache entry")
        };
        Ok(Json(nodes.into_iter().map(NodeDto::from).collect()))
    }

    #[post("/nodes/promote")]
    async fn promote(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .nodes()
            .promote(body.node_id)
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(body.server_id).await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/demote")]
    async fn demote(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .nodes()
            .demote(body.node_id)
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(body.server_id).await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/availability")]
    async fn set_availability(
        &self,
        _claims: Claims,
        Json(body): Json<NodeAvailabilityDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let node_aval = NodeAvailability::try_from(body.availability.as_str())
            .map_err(|_| (StatusCode::BAD_REQUEST, "invalid availability value".into()))?;
        docker
            .nodes()
            .update(body.node_id)
            .availability(node_aval)
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(body.server_id).await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/remove")]
    async fn remove_node(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .nodes()
            .remove(body.node_id)
            .force()
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(body.server_id).await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/leave")]
    async fn leave(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let mut leave = docker.swarm().leave();
        if body.force.unwrap_or(false) {
            leave = leave.force();
        }
        leave.run().await.map_err(map_exec)?;
        self.invalidate_swarm(body.server_id).await;
        Ok(StatusCode::NO_CONTENT)
    }

    /// Joins `target_server_id` into `manager_server_id`'s cluster in one
    /// shot: fetches the manager's join token + advertised address, force
    /// leaves any standalone swarm the target is already part of, then runs
    /// `docker swarm join` on the target with a VPN-aware advertise address
    /// of its own (so both sides are reachable over the same mesh).
    #[post("/join")]
    async fn join(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmJoinDto>,
    ) -> Result<StatusCode, ApiError> {
        let role = body.role;

        let manager_docker = self.docker(body.manager_server_id).await?;
        let manager_info = manager_docker.swarm().inspect().await.map_err(map_exec)?;
        if manager_info.local_node_state.to_lowercase() != "active" {
            return Err((
                StatusCode::BAD_REQUEST,
                "target cluster's manager does not have an active swarm".into(),
            ));
        }
        let token = manager_docker
            .swarm()
            .join_token()
            .get(role)
            .await
            .map_err(map_exec)?;
        let remote_addr = format!("{}:2377", manager_info.node_addr);

        let target_executor = self.executor_for(Some(body.target_server_id)).await?;
        let target_docker = DockerCli::from_executor(target_executor.clone());

        // Docker assigns a fresh Node ID on every join, so re-joining a server
        // that was previously part of this cluster (leave/rejoin, re-provision,
        // etc.) leaves a "Down" ghost entry behind under its old ID. Grab the
        // hostname up front so we can sweep those away once the new join lands.
        let target_hostname = target_docker
            .info()
            .await
            .ok()
            .and_then(|info| info.get("Name").and_then(|v| v.as_str()).map(str::to_owned))
            .unwrap_or_default();

        // Best-effort: leave any existing standalone swarm on the target first.
        // If it isn't part of one, this just fails harmlessly.
        let _ = target_docker.swarm().leave().force().run().await;

        let advertise_addr = match self
            .servers
            .setup_advertise_addr(body.target_server_id, None)
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?
        {
            Some(address) => address,
            None => detect_advertise_addr(&target_executor).await,
        };
        target_docker
            .swarm()
            .join()
            .token(token)
            .advertise_addr(&advertise_addr)
            .listen_addr("0.0.0.0:2377")
            .remote(remote_addr)
            .run()
            .await
            .map_err(map_exec)?;

        if !target_hostname.is_empty() {
            self.remove_stale_hostname_duplicates(
                &manager_docker,
                &target_docker,
                &target_hostname,
            )
            .await;
        }

        self.invalidate_swarm(Some(body.target_server_id)).await;
        self.invalidate_swarm(body.manager_server_id).await;

        Ok(StatusCode::NO_CONTENT)
    }

    /// Removes prior "Down" node entries that share the freshly-joined node's
    /// hostname — leftovers from an earlier join of the same server under a
    /// different Node ID. Best-effort: failures here shouldn't fail the join
    /// itself, the cluster is already in the desired state either way.
    async fn remove_stale_hostname_duplicates(
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

    /// Drops the cached info/nodes for a server so the next read pays for a
    /// fresh SSH round trip instead of serving a result that a write just
    /// invalidated (promote/demote/join/leave/etc all change what the next
    /// `/swarm/info` or `/swarm/nodes` call should return).
    async fn invalidate_swarm(&self, server_id: Option<i64>) {
        self.cache.invalidate(&CacheKey::SwarmInfo(server_id)).await;
        self.cache
            .invalidate(&CacheKey::SwarmNodes(server_id))
            .await;
    }

    async fn executor_for(&self, server_id: Option<i64>) -> Result<CommandExecutor, ApiError> {
        match server_id {
            None => Ok(CommandExecutor::Local(LocalExecutor::new())),
            Some(id) => {
                let executor = remote_executor_for(self.db.as_ref(), id).await?;
                Ok(CommandExecutor::Remote(executor))
            }
        }
    }

    async fn docker(&self, server_id: Option<i64>) -> Result<DockerCli, ApiError> {
        Ok(DockerCli::from_executor(
            self.executor_for(server_id).await?,
        ))
    }
}

async fn remote_executor_for(db: &SqlitePool, server_id: i64) -> Result<RemoteExecutor, ApiError> {
    let row = sqlx::query_as::<_, (String, i64, String, String, String)>(
        r#"SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key
           FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id
           WHERE s.id = ?"#,
    )
    .bind(server_id)
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "server or SSH key not found".into()))?;

    let port =
        u16::try_from(row.1).map_err(|_| (StatusCode::BAD_REQUEST, "invalid SSH port".into()))?;

    Ok(RemoteExecutor::new(
        row.0,
        port,
        row.2,
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    )
    .with_sudo())
}

fn map_exec(error: ExecError) -> ApiError {
    tracing::error!(error = %error, "swarm command failed");
    match error {
        ExecError::CommandFailed { .. } => (StatusCode::BAD_GATEWAY, error.to_string()),
        ExecError::Ssh(_) => (StatusCode::BAD_GATEWAY, error.to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}
