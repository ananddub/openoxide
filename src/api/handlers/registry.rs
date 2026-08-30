use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, RegistryService, RegistryServiceExt, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::{AppStateCache, CacheEnum, CacheKey};
use crate::services::registry::{
    CreateRegistryDto, PatchRegistryDto, RegistryService as AppRegistryService,
};

pub struct RegistryController {
    service: Arc<AppRegistryService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl RegistryController {
    pub fn new(service: Arc<AppRegistryService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("registry not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::new(ErrorCode::AlreadyExists, msg.clone()),
        other => {
            tracing::error!(error = %other, "registry database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn registry_to_resource(dto: RegistryResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id.parse::<i64>().unwrap_or(0),
        kind: "registry".into(),
        payload,
        ..Default::default()
    }
}

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

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

use crate::flash_gen::models::Registries;
use crate::flash_gen::registries::RegistriesUsageReferencesRow;

#[derive(Debug, serde::Deserialize)]
struct RegistryTagsQuery {
    repository: String,
}

#[derive(Debug, serde::Deserialize)]
struct TestRegistryDto {
    registry_url: String,
    username: String,
    password: String,
}

#[derive(Debug, serde::Deserialize)]
struct RotateRegistryCredentialsDto {
    username: String,
    password: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct RegistryRepositoriesDto {
    repositories: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct RegistryTagsDto {
    repository: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct RegistryUsageDto {
    resource_id: String,
    resource_name: String,
    resource_type: String,
    usage_kind: String,
}

impl From<RegistriesUsageReferencesRow> for RegistryUsageDto {
    fn from(value: RegistriesUsageReferencesRow) -> Self {
        Self {
            resource_id: value.resource_id.map(|id| id.to_string()).unwrap_or_default(),
            resource_name: value.resource_name,
            resource_type: value.resource_type,
            usage_kind: value.usage_kind,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct RegistryResponseDto {
    id: String,
    registry_name: String,
    image_prefix: Option<String>,
    username: String,
    password: String,
    registry_url: String,
    registry_type: String,
    created_at: i64,
    updated_at: i64,
}

impl From<Registries> for RegistryResponseDto {
    fn from(val: Registries) -> Self {
        Self {
            id: val.id.to_string(),
            registry_name: val.registry_name,
            image_prefix: val.image_prefix,
            username: val.username,
            password: val.password,
            registry_url: val.registry_url,
            registry_type: val.registry_type,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
