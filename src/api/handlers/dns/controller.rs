use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateDnsProviderRequest, CreateDnsProviderResponse, CreateDnsRecordRequest,
    CreateDnsRecordResponse, DeleteDnsProviderRequest, DeleteDnsRecordRequest, DnsService,
    DnsServiceExt, Empty, GetDnsProviderRequest, GetDnsProviderResponse, ListDnsProvidersRequest,
    ListDnsProvidersResponse, ListDnsRecordsRequest, ListDnsRecordsResponse, ListDnsZonesRequest,
    ListDnsZonesResponse, TestDnsConnectionRequest, TestDnsConnectionResponse,
    TestDnsProviderRequest, TestDnsProviderResponse, UpdateDnsProviderRequest,
    UpdateDnsProviderResponse, UpdateDnsRecordRequest, UpdateDnsRecordResponse,
};
use crate::services::networking::dns::{
    CreateDnsProviderDto, DnsRecordTargetDto, UpdateDnsProviderDto, UpdateDnsRecordDto,
    UpsertDnsRecordDto,
};

use super::helper::*;
use super::init::DnsController;

#[auto_grpc::service(connect = true, auth = true)]
impl DnsService for DnsController {
    #[permission(Server, CanRead)]
    async fn list_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListDnsProvidersRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDnsProvidersResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let items = self
            .service
            .list_providers(org_id)
            .await
            .map_err(map_dns_error)?;
        let providers = items.into_iter().map(provider_to_proto).collect();
        Response::ok(ListDnsProvidersResponse {
            providers,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetDnsProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetDnsProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .get_provider(request.id, org_id)
            .await
            .map_err(map_dns_error)?;
        Response::ok(GetDnsProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn create_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateDnsProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateDnsProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let dto = CreateDnsProviderDto {
            name: request.name.to_owned(),
            provider_type: proto_to_provider_type(request.provider_type)?,
            credentials_json: request.credentials_json.to_owned(),
        };
        let provider = self
            .service
            .create_provider(org_id, dto)
            .await
            .map_err(map_dns_error)?;
        Response::ok(CreateDnsProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            ..Default::default()
        })
    }

    #[permission(Server, CanUpdate)]
    async fn update_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateDnsProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateDnsProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let dto = UpdateDnsProviderDto {
            name: request.name.map(str::to_owned),
            provider_type: request
                .provider_type
                .map(proto_to_provider_type)
                .transpose()?,
            credentials_json: request.credentials_json.map(str::to_owned),
        };
        let provider = self
            .service
            .update_provider(request.id, org_id, dto)
            .await
            .map_err(map_dns_error)?;
        Response::ok(UpdateDnsProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteDnsProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = auth.organization_id;
        self.service
            .delete_provider(request.id, org_id)
            .await
            .map_err(map_dns_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanRead)]
    async fn test_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestDnsProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestDnsProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let result = self
            .service
            .test_connection(request.id, org_id)
            .await
            .map_err(map_dns_error)?;
        Response::ok(TestDnsProviderResponse {
            success: result.success,
            message: result.message,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestDnsConnectionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestDnsConnectionResponse> + Send + use<'a>> {
        let provider_type = proto_to_provider_type(request.provider_type)?;
        let credentials_json = request.credentials_json.to_owned();
        let result = self
            .service
            .test_credentials(provider_type, &credentials_json)
            .await
            .map_err(map_dns_error)?;
        Response::ok(TestDnsConnectionResponse {
            success: result.success,
            message: result.message,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_zones<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListDnsZonesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDnsZonesResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let zones = self
            .service
            .list_zones(request.provider_id, org_id)
            .await
            .map_err(map_dns_error)?;
        let zones = zones.into_iter().map(zone_to_proto).collect();
        Response::ok(ListDnsZonesResponse {
            zones,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_records<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListDnsRecordsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDnsRecordsResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let zone_id = request.zone_id.to_owned();
        let records = self
            .service
            .list_records(request.provider_id, org_id, &zone_id)
            .await
            .map_err(map_dns_error)?;
        let records = records.into_iter().map(record_to_proto).collect();
        Response::ok(ListDnsRecordsResponse {
            records,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn create_record<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateDnsRecordRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateDnsRecordResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let dto = UpsertDnsRecordDto {
            zone_id: request.zone_id.to_owned(),
            record_type: request.record_type.to_owned(),
            name: request.name.to_owned(),
            content: request.content.to_owned(),
            ttl: request.ttl,
            proxied: request.proxied,
        };
        let record = self
            .service
            .upsert_record(request.provider_id, org_id, dto)
            .await
            .map_err(map_dns_error)?;
        Response::ok(CreateDnsRecordResponse {
            record: ::buffa::MessageField::some(record_to_proto(record)),
            ..Default::default()
        })
    }

    #[permission(Server, CanUpdate)]
    async fn update_record<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateDnsRecordRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateDnsRecordResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let target = DnsRecordTargetDto {
            zone_id: request.zone_id.to_owned(),
            id: opt_str(&request.id),
            name: request.old_name.to_owned(),
            record_type: opt_str(&request.old_record_type),
            content: opt_str(&request.old_content),
        };
        let next = UpsertDnsRecordDto {
            zone_id: request.zone_id.to_owned(),
            record_type: request.record_type.to_owned(),
            name: request.name.to_owned(),
            content: request.content.to_owned(),
            ttl: request.ttl,
            proxied: request.proxied,
        };
        let record = self
            .service
            .update_record(
                request.provider_id,
                org_id,
                UpdateDnsRecordDto { target, next },
            )
            .await
            .map_err(map_dns_error)?;
        Response::ok(UpdateDnsRecordResponse {
            record: ::buffa::MessageField::some(record_to_proto(record)),
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_record<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteDnsRecordRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let target = DnsRecordTargetDto {
            zone_id: request.zone_id.to_owned(),
            id: opt_str(&request.id),
            name: request.name.to_owned(),
            record_type: opt_str(&request.record_type),
            content: opt_str(&request.content),
        };
        self.service
            .delete_record(request.provider_id, org_id, target)
            .await
            .map_err(map_dns_error)?;
        Response::ok(Empty::default())
    }
}

/// Optional proto strings arrive as `Option<BytesStr>`; an absent one means the caller
/// could not narrow the match, which the service reads as an empty field.
fn opt_str(value: &Option<impl AsRef<str>>) -> String {
    match value {
        Some(value) => value.as_ref().to_string(),
        None => String::new(),
    }
}
