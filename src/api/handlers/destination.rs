use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DestinationService, DestinationServiceExt, Empty, OperationResult, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::{AppStateCache, CacheEnum, CacheKey};
use crate::services::destination::{
    CreateDestinationDto, DestinationService as AppDestinationService, PatchDestinationDto,
};

/// Helper struct to extract the string ID from payload JSON for destination entities.
#[derive(serde::Deserialize)]
struct DestinationId {
    id: String,
}

pub struct DestinationController {
    service: Arc<AppDestinationService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl DestinationController {
    pub fn new(service: Arc<AppDestinationService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("destination not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "destination database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn destination_to_resource(dto: DestinationResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id.parse::<i64>().unwrap_or(0),
        kind: "destination".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl DestinationService for DestinationController {
    #[permission(Database, CanRead)]
    async fn list_destinations<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        if let Some(CacheEnum::DestinationsList(cached)) =
            self.cache.get(&CacheKey::DestinationsList).await
        {
            let items = cached
                .into_iter()
                .map(DestinationResponseDto::from)
                .map(destination_to_resource)
                .collect();
            return Response::ok(ResourceListResponse { items, ..Default::default() });
        }

        let items = self.service.list().await.map_err(map_sqlx_error)?;
        self.cache
            .insert(
                CacheKey::DestinationsList,
                CacheEnum::DestinationsList(items.clone()),
            )
            .await;

        let items = items
            .into_iter()
            .map(DestinationResponseDto::from)
            .map(destination_to_resource)
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn get_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        // Destination IDs are UUID strings — extract from payload JSON
        let id: DestinationId = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let dest = self.service.get_by_id(&id.id).await.map_err(map_sqlx_error)?;
        Response::ok(destination_to_resource(DestinationResponseDto::from(dest)))
    }

    #[permission(Database, CanCreate)]
    async fn create_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateDestinationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let created = self.service.create(dto).await.map_err(map_sqlx_error)?;
        self.cache.invalidate(&CacheKey::DestinationsList).await;
        Response::ok(destination_to_resource(DestinationResponseDto::from(created)))
    }

    #[permission(Database, CanCreate)]
    async fn update_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchDestinationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        // Destination IDs are UUID strings — extract the "id" field from payload
        let id: DestinationId = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let updated = self.service.patch(&id.id, dto).await.map_err(map_sqlx_error)?;
        self.cache.invalidate(&CacheKey::DestinationsList).await;
        Response::ok(destination_to_resource(DestinationResponseDto::from(updated)))
    }

    #[permission(Database, CanDelete)]
    async fn delete_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        // Destination IDs are UUID strings — extract from payload JSON
        let id: DestinationId = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.delete(&id.id).await.map_err(map_sqlx_error)?;
        self.cache.invalidate(&CacheKey::DestinationsList).await;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanCreate)]
    async fn test_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: TestDestinationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .test_connection_raw(
                &dto.provider,
                &dto.access_key,
                &dto.secret_access_key,
                &dto.bucket,
                &dto.region,
                &dto.endpoint,
            )
            .await
            .map_err(ConnectError::invalid_argument)?;
        Response::ok(OperationResult {
            success: true,
            operation: "test_destination".into(),
            ..Default::default()
        })
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

use crate::db::models::destinations::Destination;

#[derive(Debug, serde::Deserialize)]
struct TestDestinationDto {
    provider: String,
    access_key: String,
    secret_access_key: String,
    bucket: String,
    region: String,
    endpoint: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct DestinationResponseDto {
    id: String,
    name: String,
    provider: String,
    access_key: String,
    secret_access_key: String,
    bucket: String,
    region: String,
    endpoint: String,
    organization_id: i64,
    created_at: i64,
    updated_at: i64,
}

impl From<Destination> for DestinationResponseDto {
    fn from(val: Destination) -> Self {
        Self {
            id: val.id.to_string(),
            name: val.name,
            provider: val.provider,
            access_key: val.access_key,
            secret_access_key: val.secret_access_key,
            bucket: val.bucket,
            region: val.region,
            endpoint: val.endpoint,
            organization_id: val.organization_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
