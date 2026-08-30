use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    DnsService, DnsServiceExt, Empty, OperationResult, ResourceListResponse, ResourceQuery,
    ResourceRequest, ResourceResponse,
};
use crate::services::networking::dns::{CreateDnsProviderDto, UpdateDnsProviderDto, UpsertDnsRecordDto};

use super::helper::*;
use super::init::DnsController;

#[auto_grpc::service(connect = true, auth = true)]
impl DnsService for DnsController {
    #[permission(Server, CanRead)]
    async fn list_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = _query.organization_id.unwrap_or(0);
        let items = self.service.list_providers(org_id).await.map_err(map_dns_error)?;
        let items = items.into_iter().map(provider_to_resource).collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let provider = self
            .service
            .get_provider(request.id, org_id)
            .await
            .map_err(map_dns_error)?;
        Response::ok(provider_to_resource(provider))
    }

    #[permission(Server, CanCreate)]
    async fn create_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: CreateDnsProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .create_provider(org_id, dto)
            .await
            .map_err(map_dns_error)?;
        Response::ok(provider_to_resource(provider))
    }

    #[permission(Server, CanUpdate)]
    async fn update_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: UpdateDnsProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .update_provider(request.id, org_id, dto)
            .await
            .map_err(map_dns_error)?;
        Response::ok(provider_to_resource(provider))
    }

    #[permission(Server, CanDelete)]
    async fn delete_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
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
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let result = self
            .service
            .test_connection(request.id, org_id)
            .await
            .map_err(map_dns_error)?;
        Response::ok(OperationResult {
            success: result.success,
            operation: "test_provider".into(),
            message: Some(result.message),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: CreateDnsProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .test_credentials(dto.provider_type, &dto.credentials_json)
            .await
            .map_err(map_dns_error)?;
        Response::ok(OperationResult {
            success: result.success,
            operation: "test_connection".into(),
            message: Some(result.message),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_zones<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let zones = self
            .service
            .list_zones(request.id, org_id)
            .await
            .map_err(map_dns_error)?;
        let items = zones.into_iter().map(zone_to_resource).collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_records<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: UpsertDnsRecordDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let records = self
            .service
            .list_records(request.id, org_id, &dto.zone_id)
            .await
            .map_err(map_dns_error)?;
        let items = records.into_iter().map(record_to_resource).collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn create_record<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: UpsertDnsRecordDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let record = self
            .service
            .upsert_record(request.id, org_id, dto)
            .await
            .map_err(map_dns_error)?;
        Response::ok(record_to_resource(record))
    }

    #[permission(Server, CanDelete)]
    async fn delete_record<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = request.organization_id.unwrap_or(0);
        let dto: UpsertDnsRecordDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .delete_record(request.id, org_id, &dto.zone_id, &dto.name)
            .await
            .map_err(map_dns_error)?;
        Response::ok(Empty::default())
    }
}
