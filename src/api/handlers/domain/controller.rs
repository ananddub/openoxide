use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    DomainOperationsService, DomainOperationsServiceExt, Empty,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::core::cache::CacheKey;
use crate::db::reactive::realtime;
use crate::flash_gen::models::Domains;
use crate::services::networking::domain::{CreateDomainDto, PatchDomainDto};

use super::helper::*;
use super::init::DomainController;

#[auto_grpc::service(connect = true, auth = true)]
impl DomainOperationsService for DomainController {
    #[permission(Application, CanRead)]
    async fn list_domains<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list_all(auth.organization_id).await.map_err(map_error)?;
        let items = items.into_iter().map(DomainResponseDto::from).map(domain_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn stream_list_domains(
        &self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let stream = {
            let sql = format!("SELECT * FROM domains WHERE organization_id = {organization_id}");
            realtime()
                .watch_query::<Domains>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(crate::services::networking::domain::DomainRecord::from)
                    .map(DomainResponseDto::from)
                    .map(domain_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Application, CanRead)]
    async fn get_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let item = self.service.get_by_id(request.id, auth.organization_id).await.map_err(map_error)?;
        Response::ok(domain_to_resource(DomainResponseDto::from(item)))
    }

    #[permission(Application, CanRead)]
    async fn stream_domain(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let id = request.id;
        let organization_id = auth.organization_id;
        let stream = {
            let sql = format!(
                "SELECT * FROM domains WHERE id = {id} AND organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<Domains>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(domain_to_resource(DomainResponseDto::from(
                crate::services::networking::domain::DomainRecord::from(item),
            ))),
            None => Err(ConnectError::not_found("domain not found")),
        }))
    }

    #[permission(Application, CanRead)]
    async fn list_application_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        // ResourceQuery has no dedicated parent-id slot; parent-scoped lists
        // carry the parent in `environment_id` (same convention as
        // stream_list_project_tags). organization_id is the caller's org and
        // must never be interpreted as a service id.
        let application_id = request.environment_id.unwrap_or(0);
        let items = self.service.list_by_application(application_id).await.map_err(map_error)?;
        let items = items.into_iter().map(DomainResponseDto::from).map(domain_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn stream_list_application_domains(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        // Parent id rides in `environment_id` (see list_application_domains).
        let application_id = request.environment_id.unwrap_or(0);
        let stream = {
            let sql = format!("SELECT * FROM domains WHERE service_id = {application_id}");
            realtime()
                .watch_query::<Domains>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(crate::services::networking::domain::DomainRecord::from)
                    .map(DomainResponseDto::from)
                    .map(domain_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Application, CanRead)]
    async fn list_compose_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        // Parent id rides in `environment_id` (see list_application_domains).
        let compose_id = request.environment_id.unwrap_or(0);
        let items = self.service.list_by_compose(compose_id).await.map_err(map_error)?;
        let items = items.into_iter().map(DomainResponseDto::from).map(domain_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn stream_list_compose_domains(
        &self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        // Parent id rides in `environment_id` (see list_application_domains).
        let compose_id = request.environment_id.unwrap_or(0);
        let stream = {
            let sql = format!("SELECT * FROM domains WHERE service_id = {compose_id}");
            realtime()
                .watch_query::<Domains>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(crate::services::networking::domain::DomainRecord::from)
                    .map(DomainResponseDto::from)
                    .map(domain_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Application, CanCreate)]
    async fn create_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateDomainDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let app_id = dto.application_id;
        let comp_id = dto.compose_id;
        let created = self.service.create(dto, auth.organization_id).await.map_err(map_error)?;
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
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchDomainDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let updated = self.service.patch(request.id, dto, auth.organization_id).await.map_err(map_error)?;
        self.cache.invalidate_all().await;
        Response::ok(domain_to_resource(DomainResponseDto::from(updated)))
    }

    #[permission(Application, CanDelete)]
    async fn delete_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id, auth.organization_id).await.map_err(map_error)?;
        self.cache.invalidate_all().await;
        Response::ok(Empty::default())
    }
}
