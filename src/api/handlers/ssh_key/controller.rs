use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    SshKeyService, SshKeyServiceExt,
};
use crate::services::ssh_key::{CreateSshKeyDto, PatchSshKeyDto};

use super::helper::*;
use super::init::SshKeyController;

#[auto_grpc::service(connect = true, auth = true)]
impl SshKeyService for SshKeyController {
    #[permission(Server, CanRead)]
    async fn list_keys<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list().await.map_err(map_db_error)?;
        let items = items.into_iter().map(SshKeyResponseDto::from).map(key_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn get_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let key = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    #[permission(Server, CanCreate)]
    async fn create_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateSshKeyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = self.service.create(dto).await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    #[permission(Server, CanCreate)]
    async fn generate_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: GenerateSshKeyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = self.service.generate(dto.name, dto.description, &dto.key_type)
            .await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    async fn generate_pair<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: GeneratePairRequestDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (private_key, public_key) = crate::utils::ssh::generate_keypair(&dto.key_type)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let resp = GeneratePairResponseDto { private_key, public_key };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "ssh_key_pair".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn update_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchSshKeyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = self.service.patch(request.id, dto).await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    #[permission(Server, CanCreate)]
    async fn mark_used<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service.mark_used(request.id).await.map_err(map_db_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "mark_used".into(),
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
