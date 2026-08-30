use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use sqlx::SqlitePool;

use crate::api::connect::generated::openoxide::api::v1::{
    OperationResult, ResourceListResponse, ResourceRequest, ResourceResponse,
    SwarmOperationsService, SwarmOperationsServiceExt,
};
use crate::core::cache::{AppStateCache, CacheEnum, CacheKey};
use crate::services::server::RemoteServerService;
use crate::utils::docker::DockerCli;
use crate::utils::exec::{detect_advertise_addr, ExecError};

pub struct SwarmController {
    db: Arc<SqlitePool>,
    cache: Arc<AppStateCache>,
    servers: Arc<RemoteServerService>,
}

#[singleton]
impl SwarmController {
    pub fn new(
        db: Arc<SqlitePool>,
        cache: Arc<AppStateCache>,
        servers: Arc<RemoteServerService>,
    ) -> Self {
        Self { db, cache, servers }
    }
}

fn map_exec(error: ExecError) -> ConnectError {
    tracing::error!(error = %error, "swarm command failed");
    match error {
        ExecError::CommandFailed { .. } => ConnectError::new(ErrorCode::FailedPrecondition, error.to_string()),
        ExecError::Ssh(_) => ConnectError::new(ErrorCode::FailedPrecondition, error.to_string()),
        _ => ConnectError::new(ErrorCode::Internal, error.to_string()),
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl SwarmOperationsService for SwarmController {
    async fn info<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: SwarmConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = CacheKey::SwarmInfo(dto.server_id);
        let cached = self
            .cache
            .try_get_with(key, async {
                let docker = self.docker(dto.server_id).await?;
                let swarm = docker.swarm().inspect().await.map_err(map_exec)?;
                Ok::<_, ConnectError>(CacheEnum::SwarmInfo(swarm))
            })
            .await
            .map_err(|e| (*e).clone())?;

        let CacheEnum::SwarmInfo(swarm) = cached else {
            unreachable!("SwarmInfo key always yields a SwarmInfo cache entry")
        };
        let info = SwarmInfoDto {
            node_id: swarm.node_id,
            node_addr: swarm.node_addr,
            local_node_state: swarm.local_node_state,
            control_available: swarm.control_available,
            nodes: swarm.nodes as i64,
            managers: swarm.managers as i64,
        };
        let payload = serde_json::to_string(&info).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "swarm_info".into(),
            payload,
            ..Default::default()
        })
    }

    async fn get_join_tokens<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: SwarmConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let docker = self.docker(dto.server_id).await?;
        let worker = docker
            .swarm()
            .join_token()
            .get(crate::utils::docker::core::types::SwarmRole::Worker)
            .await
            .map_err(map_exec)?;
        let manager = docker
            .swarm()
            .join_token()
            .get(crate::utils::docker::core::types::SwarmRole::Manager)
            .await
            .map_err(map_exec)?;
        let tokens = SwarmTokensDto { worker, manager };
        let payload = serde_json::to_string(&tokens).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "swarm_tokens".into(),
            payload,
            ..Default::default()
        })
    }

    async fn list_nodes<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let dto: SwarmConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = CacheKey::SwarmNodes(dto.server_id);
        let cached = self
            .cache
            .try_get_with(key, async {
                let docker = self.docker(dto.server_id).await?;
                let summaries = docker.nodes().list().run_json().await.map_err(map_exec)?;
                let ids: Vec<String> = summaries.into_iter().map(|s| s.id).collect();
                let nodes = docker.nodes().inspect_all(&ids).await.map_err(map_exec)?;
                Ok::<_, ConnectError>(CacheEnum::SwarmNodes(nodes))
            })
            .await
            .map_err(|e| (*e).clone())?;

        let CacheEnum::SwarmNodes(nodes) = cached else {
            unreachable!("SwarmNodes key always yields a SwarmNodes cache entry")
        };
        let items = nodes
            .into_iter()
            .map(|n| {
                let dto = NodeDto::from(n);
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "swarm_node".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    async fn promote_node<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: NodeActionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let docker = self.docker(dto.server_id).await?;
        docker.nodes().promote(dto.node_id).run().await.map_err(map_exec)?;
        self.invalidate_swarm(dto.server_id).await;
        Response::ok(OperationResult {
            success: true,
            operation: "promote_node".into(),
            ..Default::default()
        })
    }

    async fn demote_node<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: NodeActionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let docker = self.docker(dto.server_id).await?;
        docker.nodes().demote(dto.node_id).run().await.map_err(map_exec)?;
        self.invalidate_swarm(dto.server_id).await;
        Response::ok(OperationResult {
            success: true,
            operation: "demote_node".into(),
            ..Default::default()
        })
    }

    async fn update_node_availability<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: NodeAvailabilityDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let node_aval = crate::utils::docker::core::types::NodeAvailability::try_from(dto.availability.as_str())
            .map_err(|_| ConnectError::invalid_argument("invalid availability value"))?;
        let docker = self.docker(dto.server_id).await?;
        docker.nodes().update(dto.node_id).availability(node_aval).run().await.map_err(map_exec)?;
        self.invalidate_swarm(dto.server_id).await;
        Response::ok(OperationResult {
            success: true,
            operation: "update_node_availability".into(),
            ..Default::default()
        })
    }

    async fn remove_node<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: NodeActionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let docker = self.docker(dto.server_id).await?;
        docker.nodes().remove(dto.node_id).force().run().await.map_err(map_exec)?;
        self.invalidate_swarm(dto.server_id).await;
        Response::ok(OperationResult {
            success: true,
            operation: "remove_node".into(),
            ..Default::default()
        })
    }

    async fn leave<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: SwarmConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let docker = self.docker(dto.server_id).await?;
        let mut leave = docker.swarm().leave();
        if dto.force.unwrap_or(false) {
            leave = leave.force();
        }
        leave.run().await.map_err(map_exec)?;
        self.invalidate_swarm(dto.server_id).await;
        Response::ok(OperationResult {
            success: true,
            operation: "leave".into(),
            ..Default::default()
        })
    }

    async fn join<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: SwarmJoinDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;

        let manager_docker = self.docker(dto.manager_server_id).await?;
        let manager_info = manager_docker.swarm().inspect().await.map_err(map_exec)?;
        if manager_info.local_node_state.to_lowercase() != "active" {
            return Err(ConnectError::invalid_argument(
                "target cluster's manager does not have an active swarm",
            ));
        }
        let token = manager_docker
            .swarm()
            .join_token()
            .get(dto.role)
            .await
            .map_err(map_exec)?;
        let remote_addr = format!("{}:2377", manager_info.node_addr);

        let target_executor = self.executor_for(Some(dto.target_server_id)).await?;
        let target_docker = DockerCli::from_executor(target_executor.clone());

        let target_hostname = target_docker
            .system()
            .info()
            .run()
            .await
            .ok()
            .and_then(|info| info.get("Name").and_then(|v| v.as_str()).map(str::to_owned))
            .unwrap_or_default();

        let _ = target_docker.swarm().leave().force().run().await;

        let advertise_addr = match self
            .servers
            .setup_advertise_addr(dto.target_server_id, None)
            .await
            .map_err(|error| ConnectError::invalid_argument(error.to_string()))?
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
            self.remove_stale_hostname_duplicates(&manager_docker, &target_docker, &target_hostname)
                .await;
        }

        self.invalidate_swarm(Some(dto.target_server_id)).await;
        self.invalidate_swarm(dto.manager_server_id).await;

        Response::ok(OperationResult {
            success: true,
            operation: "join".into(),
            ..Default::default()
        })
    }
}

// ── Private helpers ──────────────────────────────────────────────────

impl SwarmController {
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

    async fn invalidate_swarm(&self, server_id: Option<i64>) {
        self.cache.invalidate(&CacheKey::SwarmInfo(server_id)).await;
        self.cache.invalidate(&CacheKey::SwarmNodes(server_id)).await;
    }

    async fn executor_for(
        &self,
        server_id: Option<i64>,
    ) -> Result<crate::utils::exec::CommandExecutor, ConnectError> {
        crate::services::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(|error| ConnectError::new(ErrorCode::FailedPrecondition, error))
    }

    async fn docker(&self, server_id: Option<i64>) -> Result<DockerCli, ConnectError> {
        Ok(DockerCli::from_executor(self.executor_for(server_id).await?))
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

use serde::{Deserialize, Serialize};

use crate::utils::docker::NodeInspect;
use crate::utils::docker::core::types::{NodeRole, SwarmRole};

// ------------------------------------------------------------------ //
//  Request DTOs                                                        //
// ------------------------------------------------------------------ //

/// Connection details needed to talk to a remote server's Docker daemon.
#[derive(Debug, Deserialize)]
struct SwarmConnectionDto {
    /// Server DB id — used to resolve SSH credentials.
    server_id: Option<i64>,
    /// Force the operation when Docker requires explicit confirmation.
    ///
    /// Used by swarm leave for manager nodes. Keep this false for normal
    /// leave actions; set true only when intentionally resetting a node so it
    /// can join another cluster.
    force: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct NodeActionDto {
    /// Server id of the node to act on.
    server_id: Option<i64>,
    /// Node id (from docker node ls).
    node_id: String,
}

#[derive(Debug, Deserialize)]
struct NodeAvailabilityDto {
    server_id: Option<i64>,
    node_id: String,
    /// active | pause | drain
    availability: String,
}

/// Joins one server into another server's existing Swarm cluster, in one
/// shot: force-leaves any standalone swarm the target is already part of,
/// then joins it to the manager's cluster with a VPN-aware advertise
/// address.
#[derive(Debug, Deserialize)]
struct SwarmJoinDto {
    /// Server id of the node that should join the cluster.
    target_server_id: i64,
    /// Server id whose cluster to join. None = local engine's cluster.
    manager_server_id: Option<i64>,
    role: SwarmRole,
}

// ------------------------------------------------------------------ //
//  Response DTOs                                                       //
// ------------------------------------------------------------------ //

#[derive(Debug, Serialize)]
struct SwarmTokensDto {
    worker: String,
    manager: String,
}

#[derive(Debug, Serialize)]
struct SwarmInfoDto {
    node_id: String,
    node_addr: String,
    local_node_state: String,
    control_available: bool,
    nodes: i64,
    managers: i64,
}

#[derive(Debug, Serialize)]
struct NodeDto {
    id: String,
    hostname: String,
    status: String,
    availability: String,
    /// From `Spec.Role`, the authoritative config value. Doesn't flip based
    /// on reachability, unlike `docker node ls`'s ManagerStatus column.
    role: NodeRole,
    is_leader: bool,
    /// "reachable" | "unreachable" | "" (empty for workers).
    reachability: String,
    ip_address: String,
    engine_version: String,
}

impl From<NodeInspect> for NodeDto {
    fn from(n: NodeInspect) -> Self {
        let (is_leader, reachability) = n
            .manager_status
            .map(|m| (m.leader, m.reachability))
            .unwrap_or_default();
        Self {
            id: n.id,
            hostname: n.description.hostname,
            status: n.status.state,
            availability: n.spec.availability,
            role: n.spec.role,
            is_leader,
            reachability,
            ip_address: n.status.addr,
            engine_version: n.description.engine.engine_version,
        }
    }
}
