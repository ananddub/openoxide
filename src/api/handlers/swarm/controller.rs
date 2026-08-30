use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    OperationResult, ResourceListResponse, ResourceRequest, ResourceResponse,
    SwarmOperationsService, SwarmOperationsServiceExt,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::utils::docker::DockerCli;
use crate::utils::exec::detect_advertise_addr;

use super::helper::*;
use super::init::SwarmController;

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
