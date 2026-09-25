use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateDomainRequest, CreateDomainResponse, DeleteDomainRequest, DomainService,
    DomainServiceExt, Empty, GetDomainRequest, GetDomainResponse, ListApplicationDomainsRequest,
    ListApplicationDomainsResponse, ListComposeDomainsRequest, ListComposeDomainsResponse,
    ListDomainsRequest, ListDomainsResponse, UpdateDomainRequest, UpdateDomainResponse,
    ValidateDomainDnsRequest, ValidateDomainDnsResponse,
};
use crate::core::cache::CacheKey;
use crate::services::networking::domain::{CreateDomainDto, PatchDomainDto};

use super::helper::*;
use super::init::DomainController;

#[auto_grpc::service(connect = true, auth = true)]
impl DomainService for DomainController {
    #[permission(Application, CanRead)]
    async fn list_domains<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListDomainsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDomainsResponse> + Send + use<'a>> {
        let items = self
            .service
            .list_all(auth.organization_id)
            .await
            .map_err(map_error)?;
        let domains = items.into_iter().map(domain_to_proto).collect();
        Response::ok(ListDomainsResponse {
            domains,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn get_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetDomainRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetDomainResponse> + Send + use<'a>> {
        let item = self
            .service
            .get_by_id(request.id, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(GetDomainResponse {
            domain: ::buffa::MessageField::some(domain_to_proto(item)),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn list_application_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ListApplicationDomainsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListApplicationDomainsResponse> + Send + use<'a>>
    {
        let items = self
            .service
            .list_by_application(request.application_id)
            .await
            .map_err(map_error)?;
        let domains = items.into_iter().map(domain_to_proto).collect();
        Response::ok(ListApplicationDomainsResponse {
            domains,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn list_compose_domains<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ListComposeDomainsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListComposeDomainsResponse> + Send + use<'a>>
    {
        let items = self
            .service
            .list_by_compose(request.compose_id)
            .await
            .map_err(map_error)?;
        let domains = items.into_iter().map(domain_to_proto).collect();
        Response::ok(ListComposeDomainsResponse {
            domains,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateDomainRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateDomainResponse> + Send + use<'a>> {
        let dto = CreateDomainDto {
            host: request.host.to_owned(),
            https: request.https,
            port: request.port,
            path: request
                .path
                .map(str::to_owned)
                .unwrap_or_else(|| "/".into()),
            internal_path: request
                .internal_path
                .map(str::to_owned)
                .unwrap_or_else(|| "/".into()),
            custom_entrypoint: request.custom_entrypoint.map(str::to_owned),
            service_name: request.service_name.map(str::to_owned),
            custom_cert_resolver: request.custom_cert_resolver.map(str::to_owned),
            strip_path: request.strip_path,
            middlewares: request
                .middlewares
                .map(str::to_owned)
                .unwrap_or_else(|| "[]".into()),
            // The service derives the stored type from the owner ids; the DTO
            // field only needs a valid fallback.
            domain_type: request
                .domain_type
                .map(proto_to_domain_type)
                .unwrap_or_else(|| "APPLICATION".into()),
            certificate_type: request
                .certificate_type
                .map(proto_to_certificate_type)
                .unwrap_or_else(|| "NONE".into()),
            application_id: request.application_id,
            compose_id: request.compose_id,
        };
        let app_id = dto.application_id;
        let comp_id = dto.compose_id;
        let created = self
            .service
            .create(dto, auth.organization_id)
            .await
            .map_err(map_error)?;
        if let Some(id) = app_id {
            self.cache.invalidate(&CacheKey::DomainsApp(id)).await;
        }
        if let Some(id) = comp_id {
            self.cache.invalidate(&CacheKey::DomainsCompose(id)).await;
        }
        Response::ok(CreateDomainResponse {
            domain: ::buffa::MessageField::some(domain_to_proto(created)),
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateDomainRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateDomainResponse> + Send + use<'a>> {
        let dto = PatchDomainDto {
            host: request.host.map(str::to_owned),
            https: request.https,
            port: request.port,
            path: request.path.map(str::to_owned),
            internal_path: request.internal_path.map(str::to_owned),
            custom_entrypoint: request.custom_entrypoint.map(str::to_owned),
            service_name: request.service_name.map(str::to_owned),
            custom_cert_resolver: request.custom_cert_resolver.map(str::to_owned),
            strip_path: request.strip_path,
            middlewares: request.middlewares.map(str::to_owned),
            certificate_type: request.certificate_type.map(proto_to_certificate_type),
        };
        let updated = self
            .service
            .patch(request.id, dto, auth.organization_id)
            .await
            .map_err(map_error)?;
        self.cache.invalidate_all().await;
        Response::ok(UpdateDomainResponse {
            domain: ::buffa::MessageField::some(domain_to_proto(updated)),
            ..Default::default()
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_domain<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteDomainRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_error)?;
        self.cache.invalidate_all().await;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn validate_domain_dns<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ValidateDomainDnsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ValidateDomainDnsResponse> + Send + use<'a>> {
        let result = self
            .service
            .validate_dns(&request.host, request.server_id, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(ValidateDomainDnsResponse {
            is_valid: result.is_valid,
            resolved_ips: result.resolved_ips,
            expected_ips: result.expected_ips,
            error: result.error,
            ..Default::default()
        })
    }
}
