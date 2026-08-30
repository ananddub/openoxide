use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    TraefikOperationsService, TraefikOperationsServiceExt,
};
use crate::services::traefik::{
    StructuredMiddlewareDto, TraefikService, TraefikStatsLogsQueryDto, TraefikToggleRequestsDto,
    TraefikWriteFileDto, UpdateTraefikVersionDto,
};

pub struct TraefikController {
    service: Arc<TraefikService>,
}

#[singleton]
impl TraefikController {
    pub fn new(service: Arc<TraefikService>) -> Self {
        Self { service }
    }
}

fn map_error(error: String) -> ConnectError {
    ConnectError::new(ErrorCode::Internal, error)
}

fn map_bad_request(error: String) -> ConnectError {
    ConnectError::invalid_argument(error)
}

#[auto_grpc::service(connect = true, auth = true)]
impl TraefikOperationsService for TraefikController {
    #[permission(Traefik, CanRead)]
    async fn list_files<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let files = self
            .service
            .list_files(server_id)
            .await
            .map_err(map_error)?;
        let items = files
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "traefik_file".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Traefik, CanRead)]
    async fn tree<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let files = self
            .service
            .list_file_tree(server_id)
            .await
            .map_err(map_error)?;
        let items = files
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "traefik_file_tree".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Traefik, CanRead)]
    async fn get_content<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let server_id = request.organization_id;
        let payload_str = request.payload;
        let path: Option<String> = serde_json::from_str(payload_str)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let path = path.ok_or_else(|| {
            ConnectError::invalid_argument("File 'path' parameter is required")
        })?;
        let file_content = self
            .service
            .read_file(server_id, &path)
            .await
            .map_err(map_bad_request)?;
        let payload = serde_json::to_string(&file_content).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "traefik_file_content".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn update_content<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: TraefikWriteFileDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.write_file(dto).await.map_err(map_bad_request)?;
        Response::ok(OperationResult {
            success: true,
            operation: "update_content".into(),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn health<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let health = self
            .service
            .check_health(None)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&health).unwrap_or_default();
        Response::ok(OperationResult {
            success: health.is_healthy,
            operation: "health".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn request_status<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let status = self
            .service
            .have_activated_requests(None)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&status).unwrap_or_default();
        Response::ok(OperationResult {
            success: status.is_active,
            operation: "request_status".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn toggle_requests<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: TraefikToggleRequestsDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .toggle_requests(dto)
            .await
            .map_err(map_error)?;
        Response::ok(OperationResult {
            success: result,
            operation: "toggle_requests".into(),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let dto: TraefikStatsLogsQueryDto = serde_json::from_str(
            request.query.as_deref().unwrap_or("{}"),
        )
        .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let logs = self
            .service
            .read_stats_logs(dto)
            .await
            .map_err(map_error)?;
        let items = logs
            .items
            .into_iter()
            .map(|entry| {
                let payload = serde_json::to_string(&entry).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "traefik_log".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Traefik, CanRead)]
    async fn version<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let version = self
            .service
            .version(None)
            .await
            .map_err(|error| ConnectError::new(ErrorCode::FailedPrecondition, error))?;
        let payload = serde_json::to_string(&version).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "traefik_version".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn update_version<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: UpdateTraefikVersionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .update_version(dto)
            .await
            .map_err(|error| ConnectError::new(ErrorCode::FailedPrecondition, error))?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "update_version".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn render_middleware<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: StructuredMiddlewareDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = TraefikService::structured_middleware(dto)
            .map_err(|error| ConnectError::invalid_argument(error))?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "traefik_middleware".into(),
            payload,
            ..Default::default()
        })
    }
}
