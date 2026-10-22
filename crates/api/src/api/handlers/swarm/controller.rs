use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, SwarmCreateNetworkRequest, SwarmCreateNetworkResponse, SwarmDeleteNetworkRequest,
    SwarmDemoteNodeRequest, SwarmGetJoinTokensRequest, SwarmGetJoinTokensResponse,
    SwarmGetServiceRequest, SwarmGetServiceResponse, SwarmGetSettingsRequest,
    SwarmGetSettingsResponse, SwarmInfo, SwarmInfoRequest, SwarmInfoResponse, SwarmJoinRequest,
    SwarmJoinTokens, SwarmLeaveRequest, SwarmListNetworksRequest, SwarmListNetworksResponse,
    SwarmListNodesRequest, SwarmListNodesResponse, SwarmListServicesRequest,
    SwarmListServicesResponse, SwarmPromoteNodeRequest, SwarmRemoveNodeRequest, SwarmService,
    SwarmServiceExt, SwarmUpdateNetworkRequest, SwarmUpdateNetworkResponse,
    SwarmUpdateNodeAvailabilityRequest, SwarmUpdateServiceRequest, SwarmUpdateServiceResponse,
    SwarmUpdateSettingsRequest, SwarmUpdateSettingsResponse,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::utils::docker::DockerCli;
use crate::utils::exec::detect_advertise_addr;

use super::helper::*;
use super::init::SwarmController;

#[auto_grpc::service(connect = true, auth = true)]
impl SwarmService for SwarmController {
    async fn get_settings<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmGetSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmGetSettingsResponse> + Send + use<'a>> {
        Err::<Response<SwarmGetSettingsResponse>, _>(ConnectError::unimplemented(
            "swarm settings are not served yet",
        ))
    }

    async fn update_settings<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmUpdateSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmUpdateSettingsResponse> + Send + use<'a>>
    {
        Err::<Response<SwarmUpdateSettingsResponse>, _>(ConnectError::unimplemented(
            "swarm settings are not served yet",
        ))
    }

    async fn list_networks<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmListNetworksRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmListNetworksResponse> + Send + use<'a>> {
        Err::<Response<SwarmListNetworksResponse>, _>(ConnectError::unimplemented(
            "swarm networks are not served yet",
        ))
    }

    async fn create_network<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmCreateNetworkRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmCreateNetworkResponse> + Send + use<'a>>
    {
        Err::<Response<SwarmCreateNetworkResponse>, _>(ConnectError::unimplemented(
            "swarm networks are not served yet",
        ))
    }

    async fn update_network<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmUpdateNetworkRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmUpdateNetworkResponse> + Send + use<'a>>
    {
        Err::<Response<SwarmUpdateNetworkResponse>, _>(ConnectError::unimplemented(
            "swarm networks are not served yet",
        ))
    }

    async fn delete_network<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmDeleteNetworkRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        Err::<Response<Empty>, _>(ConnectError::unimplemented(
            "swarm networks are not served yet",
        ))
    }

    async fn list_services<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmListServicesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmListServicesResponse> + Send + use<'a>> {
        Err::<Response<SwarmListServicesResponse>, _>(ConnectError::unimplemented(
            "swarm services are not served yet",
        ))
    }

    async fn get_service<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmGetServiceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmGetServiceResponse> + Send + use<'a>> {
        Err::<Response<SwarmGetServiceResponse>, _>(ConnectError::unimplemented(
            "swarm services are not served yet",
        ))
    }

    async fn update_service<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, SwarmUpdateServiceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmUpdateServiceResponse> + Send + use<'a>>
    {
        Err::<Response<SwarmUpdateServiceResponse>, _>(ConnectError::unimplemented(
            "swarm services are not served yet",
        ))
    }

    async fn info<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmInfoRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmInfoResponse> + Send + use<'a>> {
        let server_id = request.server_id;
        let key = CacheKey::SwarmInfo(server_id);
        let cached = self
            .cache
            .try_get_with(key, async {
                let docker = self.docker(server_id).await?;
                let swarm = docker.swarm().inspect().await.map_err(map_exec)?;
                Ok::<_, ConnectError>(CacheEnum::SwarmInfo(swarm))
            })
            .await
            .map_err(|e| (*e).clone())?;

        let CacheEnum::SwarmInfo(swarm) = cached else {
            unreachable!("SwarmInfo key always yields a SwarmInfo cache entry")
        };
        let info = SwarmInfo {
            node_id: swarm.node_id,
            node_addr: swarm.node_addr,
            local_node_state: swarm.local_node_state,
            control_available: swarm.control_available,
            nodes: swarm.nodes as i64,
            managers: swarm.managers as i64,
        };
        Response::ok(SwarmInfoResponse {
            info: ::buffa::MessageField::some(info),
        })
    }

    async fn get_join_tokens<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmGetJoinTokensRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmGetJoinTokensResponse> + Send + use<'a>>
    {
        let server_id = request.server_id;
        let docker = self.docker(server_id).await?;
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
        let tokens = SwarmJoinTokens { worker, manager };
        Response::ok(SwarmGetJoinTokensResponse {
            tokens: ::buffa::MessageField::some(tokens),
        })
    }

    async fn list_nodes<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmListNodesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmListNodesResponse> + Send + use<'a>> {
        let server_id = request.server_id;
        let key = CacheKey::SwarmNodes(server_id);
        let cached = self
            .cache
            .try_get_with(key, async {
                let docker = self.docker(server_id).await?;
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
        let nodes = nodes.into_iter().map(node_to_proto).collect();
        Response::ok(SwarmListNodesResponse { nodes })
    }

    async fn promote_node<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmPromoteNodeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let server_id = request.server_id;
        let node_id = request.node_id.to_owned();
        let docker = self.docker(server_id).await?;
        docker
            .nodes()
            .promote(node_id)
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(server_id).await;
        Response::ok(Empty::default())
    }

    async fn demote_node<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmDemoteNodeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let server_id = request.server_id;
        let node_id = request.node_id.to_owned();
        let docker = self.docker(server_id).await?;
        docker
            .nodes()
            .demote(node_id)
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(server_id).await;
        Response::ok(Empty::default())
    }

    async fn update_node_availability<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmUpdateNodeAvailabilityRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let server_id = request.server_id;
        let node_id = request.node_id.to_owned();
        let availability = availability_to_docker_str(request.availability)?;
        let node_aval = crate::utils::docker::core::types::NodeAvailability::try_from(availability)
            .map_err(|_| ConnectError::invalid_argument("invalid availability value"))?;
        let docker = self.docker(server_id).await?;
        docker
            .nodes()
            .update(node_id)
            .availability(node_aval)
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(server_id).await;
        Response::ok(Empty::default())
    }

    async fn remove_node<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmRemoveNodeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let server_id = request.server_id;
        let node_id = request.node_id.to_owned();
        let docker = self.docker(server_id).await?;
        docker
            .nodes()
            .remove(node_id)
            .force()
            .run()
            .await
            .map_err(map_exec)?;
        self.invalidate_swarm(server_id).await;
        Response::ok(Empty::default())
    }

    async fn leave<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmLeaveRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let server_id = request.server_id;
        let docker = self.docker(server_id).await?;
        let mut leave = docker.swarm().leave();
        if request.force {
            leave = leave.force();
        }
        leave.run().await.map_err(map_exec)?;
        self.invalidate_swarm(server_id).await;
        Response::ok(Empty::default())
    }

    async fn join<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SwarmJoinRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let target_server_id = request.target_server_id;
        let manager_server_id = request.manager_server_id;
        let role = proto_role_to_docker(request.role)?;

        let manager_docker = self.docker(manager_server_id).await?;
        let manager_info = manager_docker.swarm().inspect().await.map_err(map_exec)?;
        if manager_info.local_node_state.to_lowercase() != "active" {
            return Err(ConnectError::invalid_argument(
                "target cluster's manager does not have an active swarm",
            ));
        }
        let token = manager_docker
            .swarm()
            .join_token()
            .get(role)
            .await
            .map_err(map_exec)?;
        let remote_addr = format!("{}:2377", manager_info.node_addr);

        let target_executor = self.executor_for(Some(target_server_id)).await?;
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
            .setup_advertise_addr(target_server_id, None)
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
            self.remove_stale_hostname_duplicates(
                &manager_docker,
                &target_docker,
                &target_hostname,
            )
            .await;
        }

        self.invalidate_swarm(Some(target_server_id)).await;
        self.invalidate_swarm(manager_server_id).await;

        Response::ok(Empty::default())
    }
}
