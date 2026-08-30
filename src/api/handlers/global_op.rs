use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    GlobalOperationsApiService, GlobalOperationsApiServiceExt,
};
use crate::services::global_op::{
    ContainerActionDto, DockerManagementService, DockerPruneRequestDto,
};

pub struct DockerManagementController {
    service: Arc<DockerManagementService>,
}

#[singleton]
impl DockerManagementController {
    pub fn new(service: Arc<DockerManagementService>) -> Self {
        Self { service }
    }
}

fn api_error(error: String) -> ConnectError {
    if error.starts_with("invalid") {
        ConnectError::invalid_argument(error)
    } else {
        ConnectError::new(ErrorCode::FailedPrecondition, error)
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl GlobalOperationsApiService for DockerManagementController {
    #[permission(Server, CanCreate)]
    async fn upload_file<'a>(
        &'a self,
        ctx: RequestContext,
        _requests: connectrpc::InboundStream<ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        // Client-streaming RPC: each chunk arrives as a ResourceRequest whose
        // payload carries a base64 file segment. Chunked upload is not yet
        // wired to the docker service — reject until a proper streaming
        // protocol is defined.
        let error: ServiceResult<OperationResult> = Err(ConnectError::unimplemented(
            "streaming file upload is not yet supported via ConnectRPC",
        ));
        error
    }

    #[permission(Server, CanRead)]
    async fn download_file<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        // Download returns binary content; encode as base64 in the payload.
        let dto: DownloadFileDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let bytes = self
            .service
            .download_container_bytes(dto.server_id, &dto.container_id, &dto.path)
            .await
            .map_err(api_error)?;
        let payload = serde_json::to_string(&DownloadFileResponseDto {
            content: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes),
            path: dto.path,
        })
        .unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "container_file".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_containers<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let items = self.service.containers(server_id).await.map_err(api_error)?;
        let items = items
            .into_iter()
            .map(|s| {
                let payload = serde_json::to_string(&s).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "container".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn list_images<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let items = self.service.images(server_id).await.map_err(api_error)?;
        let items = items
            .into_iter()
            .map(|s| {
                let payload = serde_json::to_string(&s).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "image".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn list_networks<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let items = self.service.networks(server_id).await.map_err(api_error)?;
        let items = items
            .into_iter()
            .map(|s| {
                let payload = serde_json::to_string(&s).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "network".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn list_volumes<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let items = self.service.volumes(server_id).await.map_err(api_error)?;
        let items = items
            .into_iter()
            .map(|s| {
                let payload = serde_json::to_string(&s).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "volume".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn inspect_container<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: InspectDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let value = self
            .service
            .container_inspect(dto.server_id, &dto.container_id)
            .await
            .map_err(api_error)?;
        let payload = serde_json::to_string(&value).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "container_inspect".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_image<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: InspectDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let value = self
            .service
            .image_inspect(dto.server_id, &dto.container_id)
            .await
            .map_err(api_error)?;
        let payload = serde_json::to_string(&value).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "image_inspect".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: InspectDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let value = self
            .service
            .network_inspect(dto.server_id, &dto.container_id)
            .await
            .map_err(api_error)?;
        let payload = serde_json::to_string(&value).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "network_inspect".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_volume<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: InspectDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let value = self
            .service
            .volume_inspect(dto.server_id, &dto.container_id)
            .await
            .map_err(api_error)?;
        let payload = serde_json::to_string(&value).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "volume_inspect".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn container_action<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: ContainerActionRequestDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .container_action(None, &request.id.to_string(), dto.action)
            .await
            .map_err(api_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "container_action".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn remove_container<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let dto: ContainerRemoveRequestDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .remove_container(None, &request.id.to_string(), dto.force, dto.volumes)
            .await
            .map_err(api_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanRead)]
    async fn disk_usage<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let value = self.service.disk_usage(server_id).await.map_err(api_error)?;
        let payload = serde_json::to_string(&value).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "disk_usage".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn prune<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: DockerPruneRequestDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.service.prune(None, dto).await.map_err(api_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "prune".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct InspectDto {
    server_id: Option<i64>,
    container_id: String,
}

#[derive(Debug, Deserialize)]
struct DownloadFileDto {
    server_id: Option<i64>,
    container_id: String,
    path: String,
}

#[derive(Debug, serde::Serialize)]
struct DownloadFileResponseDto {
    content: String,
    path: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ContainerActionRequestDto {
    action: ContainerActionDto,
}

#[derive(Debug, Clone, Deserialize)]
struct ContainerRemoveRequestDto {
    force: bool,
    volumes: bool,
}
