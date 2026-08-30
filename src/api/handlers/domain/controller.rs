use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DomainOperationsService, DomainOperationsServiceExt, Empty,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::CacheKey;
use crate::services::domain::{CreateDomainDto, PatchDomainDto};

use super::helper::*;
use super::init::DomainController;

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
