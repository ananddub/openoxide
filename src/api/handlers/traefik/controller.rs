use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, TraefikGetFileContentRequest, TraefikGetFileContentResponse,
    TraefikGetFileTreeRequest, TraefikGetFileTreeResponse, TraefikGetRequestLogsRequest,
    TraefikGetRequestLogsResponse, TraefikGetRequestStatusRequest, TraefikGetRequestStatusResponse,
    TraefikGetVersionRequest, TraefikGetVersionResponse, TraefikHealthRequest,
    TraefikHealthResponse, TraefikListFilesRequest, TraefikListFilesResponse,
    TraefikRenderMiddlewareRequest, TraefikRenderMiddlewareResponse, TraefikService,
    TraefikServiceExt, TraefikToggleRequestsRequest, TraefikToggleRequestsResponse,
    TraefikUpdateFileContentRequest, TraefikUpdateVersionRequest, TraefikUpdateVersionResponse,
};
use crate::services::networking::traefik::{
    TraefikStatsLogsQueryDto, TraefikToggleRequestsDto, TraefikWriteFileDto,
    UpdateTraefikVersionDto,
};
use crate::services::networking::traefik::TraefikService as TraefikLifecycleService;

use super::helper::*;
use super::init::TraefikController;

#[auto_grpc::service(connect = true, auth = true)]
impl TraefikService for TraefikController {
    #[permission(Traefik, CanRead)]
    async fn list_files<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikListFilesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikListFilesResponse> + Send + use<'a>> {
        let server_id = request.server_id;
        let files = self.service.list_files(server_id).await.map_err(map_error)?;
        let files = files.into_iter().map(file_node_to_proto).collect();
        Response::ok(TraefikListFilesResponse {
            files,
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn get_file_tree<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikGetFileTreeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikGetFileTreeResponse> + Send + use<'a>> {
        let server_id = request.server_id;
        let nodes = self
            .service
            .list_file_tree(server_id)
            .await
            .map_err(map_error)?;
        let nodes = nodes.into_iter().map(file_tree_node_to_proto).collect();
        Response::ok(TraefikGetFileTreeResponse {
            nodes,
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn get_file_content<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikGetFileContentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikGetFileContentResponse> + Send + use<'a>>
    {
        let server_id = request.server_id;
        let path = request.path.to_owned();
        let file_content = self
            .service
            .read_file(server_id, &path)
            .await
            .map_err(map_bad_request)?;
        Response::ok(TraefikGetFileContentResponse {
            file: ::buffa::MessageField::some(file_content_to_proto(path, file_content)),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn update_file_content<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikUpdateFileContentRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let dto = TraefikWriteFileDto {
            server_id: request.server_id,
            path: request.path.to_owned(),
            content: request.content.to_owned(),
        };
        self.service.write_file(dto).await.map_err(map_bad_request)?;
        Response::ok(Empty::default())
    }

    #[permission(Traefik, CanRead)]
    async fn health<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, TraefikHealthRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikHealthResponse> + Send + use<'a>> {
        let health = self.service.check_health(None).await.map_err(map_error)?;
        Response::ok(TraefikHealthResponse {
            health: ::buffa::MessageField::some(health_to_proto(health)),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn get_request_status<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, TraefikGetRequestStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikGetRequestStatusResponse> + Send + use<'a>>
    {
        let status = self
            .service
            .have_activated_requests(None)
            .await
            .map_err(map_error)?;
        Response::ok(TraefikGetRequestStatusResponse {
            status: ::buffa::MessageField::some(requests_status_to_proto(status)),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn toggle_requests<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikToggleRequestsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikToggleRequestsResponse> + Send + use<'a>>
    {
        let dto = TraefikToggleRequestsDto {
            server_id: request.server_id,
            enable: request.enable,
        };
        let result = self.service.toggle_requests(dto).await.map_err(map_error)?;
        Response::ok(TraefikToggleRequestsResponse {
            active: result,
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn get_request_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikGetRequestLogsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikGetRequestLogsResponse> + Send + use<'a>>
    {
        let page = request
            .page
            .map(|page| usize::try_from(page).map_err(|_| connectrpc::ConnectError::invalid_argument("page out of range")))
            .transpose()?;
        let page_size = request
            .page_size
            .map(|page_size| {
                usize::try_from(page_size)
                    .map_err(|_| connectrpc::ConnectError::invalid_argument("page_size out of range"))
            })
            .transpose()?;
        let status = request
            .status
            .map(|status| {
                u16::try_from(status)
                    .map_err(|_| connectrpc::ConnectError::invalid_argument("status out of range"))
            })
            .transpose()?;
        let dto = TraefikStatsLogsQueryDto {
            server_id: request.server_id,
            page,
            page_size,
            search: request.search.map(str::to_owned),
            status,
        };
        let logs = self.service.read_stats_logs(dto).await.map_err(map_error)?;
        let entries = logs.items.into_iter().map(log_entry_to_proto).collect();
        Response::ok(TraefikGetRequestLogsResponse {
            entries,
            total_count: logs.total_count as i64,
            page: logs.page as i64,
            page_size: logs.page_size as i64,
            ..Default::default()
        })
    }

    #[permission(Traefik, CanRead)]
    async fn get_version<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, TraefikGetVersionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikGetVersionResponse> + Send + use<'a>> {
        let version = self
            .service
            .version(None)
            .await
            .map_err(|error| {
                connectrpc::ConnectError::new(connectrpc::ErrorCode::FailedPrecondition, error)
            })?;
        Response::ok(TraefikGetVersionResponse {
            version: ::buffa::MessageField::some(version_to_proto(version)),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn update_version<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikUpdateVersionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikUpdateVersionResponse> + Send + use<'a>>
    {
        let dto = UpdateTraefikVersionDto {
            server_id: request.server_id,
            version: request.version.to_owned(),
        };
        let result = self.service.update_version(dto).await.map_err(|error| {
            connectrpc::ConnectError::new(connectrpc::ErrorCode::FailedPrecondition, error)
        })?;
        Response::ok(TraefikUpdateVersionResponse {
            version: ::buffa::MessageField::some(version_to_proto(result)),
            ..Default::default()
        })
    }

    #[permission(Traefik, CanWrite)]
    async fn render_middleware<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TraefikRenderMiddlewareRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TraefikRenderMiddlewareResponse> + Send + use<'a>>
    {
        let kind = middleware_kind_name(request.kind)?;
        let dto = typed_dto(serde_json::json!({
            "kind": kind,
            "name": request.name,
            "values": request.values.iter().map(|(key, value)| (*key, *value)).collect::<std::collections::BTreeMap<_, _>>(),
            "list": request.list.iter().collect::<Vec<_>>(),
        }))?;
        let result = TraefikLifecycleService::structured_middleware(dto)
            .map_err(|error| connectrpc::ConnectError::invalid_argument(error))?;
        Response::ok(TraefikRenderMiddlewareResponse {
            middleware: ::buffa::MessageField::some(middleware_to_proto(result)),
            ..Default::default()
        })
    }
}
