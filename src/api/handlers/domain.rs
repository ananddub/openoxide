use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DomainOperationsService, DomainOperationsServiceExt, Empty,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::{AppStateCache, CacheKey};
use crate::services::domain::{CreateDomainDto, DomainService as AppDomainService, PatchDomainDto};

pub struct DomainController {
    service: Arc<AppDomainService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl DomainController {
    pub fn new(service: Arc<AppDomainService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }
}

fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("domain not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::new(ErrorCode::AlreadyExists, msg.clone()),
        other => {
            tracing::error!(error = %other, "domain database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn domain_to_resource(dto: DomainResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "domain".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl DomainOperationsService for DomainController {
    #[permission(Application, CanRead)]
    async fn list_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list_all().await.map_err(map_error)?;
        let items = items.into_iter().map(DomainResponseDto::from).map(domain_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn get_domain<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let item = self.service.get_by_id(request.id).await.map_err(map_error)?;
        Response::ok(domain_to_resource(DomainResponseDto::from(item)))
    }

    #[permission(Application, CanRead)]
    async fn list_application_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let application_id = request.organization_id.unwrap_or(0);
        let items = self.service.list_by_application(application_id).await.map_err(map_error)?;
        let items = items.into_iter().map(DomainResponseDto::from).map(domain_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn list_compose_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let compose_id = request.organization_id.unwrap_or(0);
        let items = self.service.list_by_compose(compose_id).await.map_err(map_error)?;
        let items = items.into_iter().map(DomainResponseDto::from).map(domain_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanCreate)]
    async fn create_domain<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateDomainDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let app_id = dto.application_id;
        let comp_id = dto.compose_id;
        let created = self.service.create(dto).await.map_err(map_error)?;
        if let Some(id) = app_id {
            self.cache.invalidate(&CacheKey::DomainsApp(id)).await;
        }
        if let Some(id) = comp_id {
            self.cache.invalidate(&CacheKey::DomainsCompose(id)).await;
        }
        Response::ok(domain_to_resource(DomainResponseDto::from(created)))
    }

    #[permission(Application, CanCreate)]
    async fn update_domain<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchDomainDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let updated = self.service.patch(request.id, dto).await.map_err(map_error)?;
        self.cache.invalidate_all().await;
        Response::ok(domain_to_resource(DomainResponseDto::from(updated)))
    }

    #[permission(Application, CanDelete)]
    async fn delete_domain<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_error)?;
        self.cache.invalidate_all().await;
        Response::ok(Empty::default())
    }
}

// ── Helper DTOs for serializing ResourceResponse payloads ────────────

use crate::services::domain::DomainRecord;

#[derive(Debug, Clone, serde::Serialize)]
struct DomainResponseDto {
    id: i64,
    host: String,
    https: bool,
    port: Option<i64>,
    path: Option<String>,
    internal_path: Option<String>,
    custom_entrypoint: Option<String>,
    service_name: Option<String>,
    custom_cert_resolver: Option<String>,
    strip_path: bool,
    middlewares: String,
    domain_type: String,
    certificate_type: String,
    service_id: Option<i64>,
    created_at: i64,
    updated_at: i64,
}

impl From<DomainRecord> for DomainResponseDto {
    fn from(value: DomainRecord) -> Self {
        Self {
            id: value.id,
            host: value.host,
            https: value.https != 0,
            port: value.port,
            path: value.path,
            internal_path: value.internal_path,
            custom_entrypoint: value.custom_entrypoint,
            service_name: value.service_name,
            custom_cert_resolver: value.custom_cert_resolver,
            strip_path: value.strip_path != 0,
            middlewares: value.middlewares,
            domain_type: value.domain_type,
            certificate_type: value.certificate_type,
            service_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
