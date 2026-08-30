use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, RegistryService, RegistryServiceExt, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::services::registry::{CreateRegistryDto, PatchRegistryDto};

use super::helper::*;
use super::init::RegistryController;

#[auto_grpc::service(connect = true, auth = true)]
impl RegistryService for RegistryController {
    #[permission(Registry, CanRead)]
    async fn list_registries<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        if let Some(CacheEnum::RegistriesList(cached)) =
            self.cache.get(&CacheKey::RegistriesList).await
        {
            let items = cached.into_iter().map(RegistryResponseDto::from).map(registry_to_resource).collect();
            return Response::ok(ResourceListResponse { items, ..Default::default() });
        }

        let items = self.service.list().await.map_err(map_sqlx_error)?;
        self.cache
            .insert(
                CacheKey::RegistriesList,
                CacheEnum::RegistriesList(items.clone()),
            )
            .await;

        let items = items.into_iter().map(RegistryResponseDto::from).map(registry_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Registry, CanRead)]
    async fn get_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let registry = self.service.get_by_id(request.id).await.map_err(map_sqlx_error)?;
        Response::ok(registry_to_resource(RegistryResponseDto::from(registry)))
    }

    #[permission(Registry, CanCreate)]
    async fn create_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateRegistryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let created = self.service.create(dto).await.map_err(map_sqlx_error)?;
        self.cache.invalidate(&CacheKey::RegistriesList).await;
        Response::ok(registry_to_resource(RegistryResponseDto::from(created)))
    }

    #[permission(Registry, CanCreate)]
    async fn update_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchRegistryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let updated = self.service.patch(request.id, dto).await.map_err(map_sqlx_error)?;
        self.cache.invalidate(&CacheKey::RegistriesList).await;
        Response::ok(registry_to_resource(RegistryResponseDto::from(updated)))
    }

    #[permission(Registry, CanDelete)]
    async fn delete_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_sqlx_error)?;
        self.cache.invalidate(&CacheKey::RegistriesList).await;
        Response::ok(Empty::default())
    }

    #[permission(Registry, CanCreate)]
    async fn test_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: TestRegistryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .test_connection_raw(&dto.registry_url, &dto.username, &dto.password)
            .await
            .map_err(ConnectError::invalid_argument)?;
        Response::ok(OperationResult {
            success: true,
            operation: "test_registry".into(),
            ..Default::default()
        })
    }

    #[permission(Registry, CanRead)]
    async fn list_repositories<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let repositories = self.service
            .repositories(request.id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let payload = serde_json::to_string(&RegistryRepositoriesDto { repositories }).unwrap_or_default();
        let item = ResourceResponse {
            id: request.id,
            kind: "registry_repositories".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse { items: vec![item], ..Default::default() })
    }

    #[permission(Registry, CanRead)]
    async fn list_tags<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let dto: RegistryTagsQuery = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let repository = dto.repository.clone();
        let tags = self.service
            .tags(request.id, &repository)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        let payload = serde_json::to_string(&RegistryTagsDto { repository, tags }).unwrap_or_default();
        let item = ResourceResponse {
            id: request.id,
            kind: "registry_tags".into(),
            payload,
            ..Default::default()
        };
        Response::ok(ResourceListResponse { items: vec![item], ..Default::default() })
    }

    #[permission(Registry, CanCreate)]
    async fn rotate_credentials<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: RotateRegistryCredentialsDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let registry = self.service
            .rotate_credentials(request.id, dto.username, dto.password)
            .await
            .map_err(|e| ConnectError::invalid_argument(e))?;
        let payload = serde_json::to_string(&RegistryResponseDto::from(registry)).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "rotate_credentials".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Registry, CanRead)]
    async fn get_usage<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let rows = self.service.usage(request.id).await.map_err(map_sqlx_error)?;
        let usage: Vec<RegistryUsageDto> = rows.into_iter().map(Into::into).collect();
        let payload = serde_json::to_string(&usage).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "registry_usage".into(),
            payload,
            ..Default::default()
        })
    }
}
