use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, GlobalContainerActionRequest, GlobalContainerActionResponse,
    GlobalDeleteContainerRequest, GlobalDiskUsageRequest, GlobalDiskUsageResponse,
    GlobalDownloadContainerFileRequest, GlobalDownloadContainerFileResponse,
    GlobalDownloadFileRequest, GlobalDownloadFileResponse, GlobalInspectContainerRequest,
    GlobalInspectContainerResponse, GlobalInspectImageRequest, GlobalInspectImageResponse,
    GlobalInspectNetworkRequest, GlobalInspectNetworkResponse, GlobalInspectVolumeRequest,
    GlobalInspectVolumeResponse, GlobalListContainersRequest, GlobalListContainersResponse,
    GlobalListImagesRequest, GlobalListImagesResponse, GlobalListNetworksRequest,
    GlobalListNetworksResponse, GlobalListVolumesRequest, GlobalListVolumesResponse,
    GlobalPruneRequest, GlobalPruneResponse, GlobalRemoveContainerRequest, GlobalService,
    GlobalServiceExt, GlobalUploadContainerFileRequest, GlobalUploadContainerFileResponse,
    GlobalUploadFileRequest, GlobalUploadFileResponse,
};
use crate::services::platform::global_ops::DockerPruneRequestDto;

use super::helper::*;
use super::init::DockerManagementController;

#[auto_grpc::service(connect = true, auth = true)]
impl GlobalService for DockerManagementController {
    #[permission(Server, CanCreate)]
    async fn upload_file<'a>(
        &'a self,
        ctx: RequestContext,
        // Client-streaming RPC: exactly one header chunk first, then byte
        // chunks. Chunked upload is not yet wired to the docker service —
        // reject until a proper streaming protocol is defined.
        _requests: connectrpc::InboundStream<GlobalUploadFileRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalUploadFileResponse> + Send + use<'a>> {
        let error: ServiceResult<GlobalUploadFileResponse> = Err(ConnectError::unimplemented(
            "streaming file upload is not yet supported via ConnectRPC",
        ));
        error
    }

    #[permission(Server, CanRead)]
    async fn download_file<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalDownloadFileRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalDownloadFileResponse> + Send + use<'a>>
    {
        // Download returns raw bytes on the wire; the old handler
        // base64-wrapped them in a JSON payload.
        let path = request.path.to_owned();
        let bytes = self
            .service
            .download_container_bytes(request.server_id, request.container_id, request.path)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalDownloadFileResponse {
            content: ::buffa::bytes::Bytes::from(bytes),
            path,
        })
    }

    #[permission(Server, CanCreate)]
    async fn upload_container_file<'a>(
        &'a self,
        ctx: RequestContext,
        _requests: connectrpc::InboundStream<GlobalUploadContainerFileRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalUploadContainerFileResponse> + Send + use<'a>>
    {
        let error: ServiceResult<GlobalUploadContainerFileResponse> =
            Err(ConnectError::unimplemented(
                "streaming file upload is not yet supported via ConnectRPC",
            ));
        error
    }

    #[permission(Server, CanRead)]
    async fn download_container_file<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalDownloadContainerFileRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<GlobalDownloadContainerFileResponse> + Send + use<'a>,
    > {
        let path = request.path.to_owned();
        let bytes = self
            .service
            .download_container_bytes(request.server_id, request.container_id, request.path)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalDownloadContainerFileResponse {
            content: ::buffa::bytes::Bytes::from(bytes),
            path,
        })
    }

    #[permission(Server, CanRead)]
    async fn list_containers<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalListContainersRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalListContainersResponse> + Send + use<'a>>
    {
        let items = self
            .service
            .containers(request.server_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalListContainersResponse {
            containers: items.into_iter().map(container_to_proto).collect(),
        })
    }

    #[permission(Server, CanRead)]
    async fn list_images<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalListImagesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalListImagesResponse> + Send + use<'a>> {
        let items = self
            .service
            .images(request.server_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalListImagesResponse {
            images: items.into_iter().map(image_to_proto).collect(),
        })
    }

    #[permission(Server, CanRead)]
    async fn list_networks<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalListNetworksRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalListNetworksResponse> + Send + use<'a>>
    {
        let items = self
            .service
            .networks(request.server_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalListNetworksResponse {
            networks: items.into_iter().map(network_to_proto).collect(),
        })
    }

    #[permission(Server, CanRead)]
    async fn list_volumes<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalListVolumesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalListVolumesResponse> + Send + use<'a>> {
        let items = self
            .service
            .volumes(request.server_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalListVolumesResponse {
            volumes: items.into_iter().map(volume_to_proto).collect(),
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_container<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalInspectContainerRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalInspectContainerResponse> + Send + use<'a>>
    {
        let value = self
            .service
            .container_inspect(request.server_id, request.container_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalInspectContainerResponse {
            data_json: json_data(&value),
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_image<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalInspectImageRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalInspectImageResponse> + Send + use<'a>>
    {
        let value = self
            .service
            .image_inspect(request.server_id, request.image_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalInspectImageResponse {
            data_json: json_data(&value),
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalInspectNetworkRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalInspectNetworkResponse> + Send + use<'a>>
    {
        let value = self
            .service
            .network_inspect(request.server_id, request.network_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalInspectNetworkResponse {
            data_json: json_data(&value),
        })
    }

    #[permission(Server, CanRead)]
    async fn inspect_volume<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalInspectVolumeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalInspectVolumeResponse> + Send + use<'a>>
    {
        let value = self
            .service
            .volume_inspect(request.server_id, request.volume_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalInspectVolumeResponse {
            data_json: json_data(&value),
        })
    }

    #[permission(Server, CanCreate)]
    async fn container_action<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalContainerActionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalContainerActionResponse> + Send + use<'a>>
    {
        // The typed request carries no server_id, matching the old handler
        // which acted on the local docker host.
        let action = to_container_action(request.action)?;
        let result = self
            .service
            .container_action(None, request.container_id, action)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalContainerActionResponse {
            success: true,
            output: ::buffa::MessageField::some(output_to_proto(result)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn remove_container<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalRemoveContainerRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .remove_container(None, request.container_id, request.force, request.volumes)
            .await
            .map_err(api_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanCreate)]
    async fn delete_container<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalDeleteContainerRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .remove_container(None, request.container_id, request.force, request.volumes)
            .await
            .map_err(api_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanRead)]
    async fn disk_usage<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalDiskUsageRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalDiskUsageResponse> + Send + use<'a>> {
        let value = self
            .service
            .disk_usage(request.server_id)
            .await
            .map_err(api_error)?;
        Response::ok(GlobalDiskUsageResponse {
            data_json: json_data(&value),
        })
    }

    #[permission(Server, CanCreate)]
    async fn prune<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GlobalPruneRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GlobalPruneResponse> + Send + use<'a>> {
        let dto = DockerPruneRequestDto {
            containers: request.containers,
            images: request.images,
            networks: request.networks,
            volumes: request.volumes,
        };
        let result = self.service.prune(None, dto).await.map_err(api_error)?;
        Response::ok(GlobalPruneResponse {
            operations: result.operations.into_iter().map(output_to_proto).collect(),
        })
    }
}
