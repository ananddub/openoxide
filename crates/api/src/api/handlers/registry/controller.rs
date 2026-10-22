use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateRegistryRequest, CreateRegistryResponse, DeleteRegistryRequest, Empty,
    GetRegistryRequest, GetRegistryResponse, GetRegistryUsageRequest, GetRegistryUsageResponse,
    ListRegistriesRequest, ListRegistriesResponse, ListRegistryRepositoriesRequest,
    ListRegistryRepositoriesResponse, ListRegistryTagsRequest, ListRegistryTagsResponse,
    RegistryService, RegistryServiceExt, RotateRegistryCredentialsRequest,
    RotateRegistryCredentialsResponse, TestRegistryRequest, TestRegistryResponse,
    UpdateRegistryRequest, UpdateRegistryResponse,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::services::platform::registry::{CreateRegistryDto, PatchRegistryDto};

use super::helper::*;
use super::init::RegistryController;

#[auto_grpc::service(connect = true, auth = true)]
impl RegistryService for RegistryController {
    #[permission(Registry, CanRead)]
    async fn list_registries<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ListRegistriesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListRegistriesResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let cache_key = CacheKey::RegistriesList(organization_id);
        if let Some(CacheEnum::RegistriesList(cached)) = self.cache.get(&cache_key).await {
            let registries = cached.into_iter().map(registry_to_proto).collect();
            return Response::ok(ListRegistriesResponse { registries });
        }

        let items = self
            .service
            .list(organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .insert(cache_key, CacheEnum::RegistriesList(items.clone()))
            .await;

        let registries = items.into_iter().map(registry_to_proto).collect();
        Response::ok(ListRegistriesResponse { registries })
    }

    #[permission(Registry, CanRead)]
    async fn get_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetRegistryRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetRegistryResponse> + Send + use<'a>> {
        let registry = self
            .service
            .get_by_id(request.id, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(GetRegistryResponse {
            registry: ::buffa::MessageField::some(registry_to_proto(registry)),
        })
    }

    #[permission(Registry, CanCreate)]
    async fn create_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateRegistryRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateRegistryResponse> + Send + use<'a>> {
        let dto = CreateRegistryDto {
            registry_name: request.registry_name.to_owned(),
            image_prefix: request.image_prefix.map(str::to_owned),
            username: request.username.to_owned(),
            password: request.password.to_owned(),
            registry_url: request.registry_url.to_owned(),
            registry_type: request.registry_type.to_owned(),
        };
        let created = self
            .service
            .create(dto, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .invalidate(&CacheKey::RegistriesList(auth.organization_id))
            .await;
        Response::ok(CreateRegistryResponse {
            registry: ::buffa::MessageField::some(registry_to_proto(created)),
        })
    }

    #[permission(Registry, CanCreate)]
    async fn update_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateRegistryRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateRegistryResponse> + Send + use<'a>> {
        // An absent field keeps its stored value; `image_prefix`
        // present-but-empty clears it.
        let dto = PatchRegistryDto {
            registry_name: request.registry_name.map(str::to_owned),
            image_prefix: request.image_prefix.map(|v| {
                if v.is_empty() {
                    None
                } else {
                    Some(v.to_owned())
                }
            }),
            username: request.username.map(str::to_owned),
            password: request.password.map(str::to_owned),
            registry_url: request.registry_url.map(str::to_owned),
            registry_type: request.registry_type.map(str::to_owned),
        };
        let updated = self
            .service
            .patch(request.id, dto, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .invalidate(&CacheKey::RegistriesList(auth.organization_id))
            .await;
        Response::ok(UpdateRegistryResponse {
            registry: ::buffa::MessageField::some(registry_to_proto(updated)),
        })
    }

    #[permission(Registry, CanDelete)]
    async fn delete_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteRegistryRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .invalidate(&CacheKey::RegistriesList(auth.organization_id))
            .await;
        Response::ok(Empty::default())
    }

    #[permission(Registry, CanCreate)]
    async fn test_registry<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestRegistryRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestRegistryResponse> + Send + use<'a>> {
        // With `id` set, the saved registry's stored credentials are tested;
        // otherwise the raw url/username/password are used (create dialog).
        let result = match request.id {
            Some(id) => self.service.test_connection(id, auth.organization_id).await,
            None => {
                self.service
                    .test_connection_raw(request.registry_url, request.username, request.password)
                    .await
            }
        };
        result.map_err(ConnectError::invalid_argument)?;
        Response::ok(TestRegistryResponse { success: true })
    }

    #[permission(Registry, CanRead)]
    async fn list_repositories<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListRegistryRepositoriesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListRegistryRepositoriesResponse> + Send + use<'a>>
    {
        let repositories = self
            .service
            .repositories(request.registry_id, auth.organization_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        Response::ok(ListRegistryRepositoriesResponse { repositories })
    }

    #[permission(Registry, CanRead)]
    async fn list_tags<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListRegistryTagsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListRegistryTagsResponse> + Send + use<'a>> {
        let repository = request.repository.to_owned();
        let tags = self
            .service
            .tags(request.registry_id, &repository, auth.organization_id)
            .await
            .map_err(|e| ConnectError::new(ErrorCode::Unavailable, e))?;
        Response::ok(ListRegistryTagsResponse { repository, tags })
    }

    #[permission(Registry, CanCreate)]
    async fn rotate_credentials<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RotateRegistryCredentialsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RotateRegistryCredentialsResponse> + Send + use<'a>>
    {
        let registry = self
            .service
            .rotate_credentials(
                request.registry_id,
                request.username.to_owned(),
                request.password.to_owned(),
                auth.organization_id,
            )
            .await
            .map_err(ConnectError::invalid_argument)?;
        Response::ok(RotateRegistryCredentialsResponse {
            registry: ::buffa::MessageField::some(registry_to_proto(registry)),
        })
    }

    #[permission(Registry, CanRead)]
    async fn get_usage<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetRegistryUsageRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetRegistryUsageResponse> + Send + use<'a>> {
        let rows = self
            .service
            .usage(request.registry_id, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        let usage = rows.into_iter().map(usage_to_proto).collect();
        Response::ok(GetRegistryUsageResponse { usage })
    }
}
