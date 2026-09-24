use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    NetworkingDiagnoseDomainsRequest, NetworkingDiagnoseDomainsResponse,
    NetworkingDiagnoseRootNetworkRequest, NetworkingDiagnoseRootNetworkResponse,
    NetworkingOperationsService, NetworkingOperationsServiceExt, NetworkingPurgeCdnRequest,
    NetworkingPurgeCdnResponse, NetworkingRepairRootNetworkRequest,
    NetworkingRepairRootNetworkResponse,
};

use super::helper::*;
use super::init::NetworkingController;

#[auto_grpc::service(connect = true, auth = true)]
impl NetworkingOperationsService for NetworkingController {
    #[permission(Application, CanCreate)]
    async fn purge_cdn<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, NetworkingPurgeCdnRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<NetworkingPurgeCdnResponse> + Send + use<'a>> {
        let provider = proto_to_cdn_provider(request.provider)?;
        let dto = typed_dto(serde_json::json!({
            "provider": provider,
            "api_token": request.api_token,
            "zone_id": request.zone_id,
            "service_id": request.service_id,
            "pull_zone_id": request.pull_zone_id,
            "purge_all": request.purge_all,
            "urls": request.urls.iter().collect::<Vec<_>>(),
        }))?;
        let result = self.cdn.purge(dto).await.map_err(map_error)?;
        Response::ok(NetworkingPurgeCdnResponse {
            provider: result.provider,
            success: result.success,
            status: u32::from(result.status),
            request_id: result.request_id,
            message: result.message,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn diagnose_domains<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, NetworkingDiagnoseDomainsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<NetworkingDiagnoseDomainsResponse> + Send + use<'a>>
    {
        let port = request
            .port
            .map(|port| {
                u16::try_from(port)
                    .map_err(|_| connectrpc::ConnectError::invalid_argument("port out of range"))
            })
            .transpose()?;
        let dto = typed_dto(serde_json::json!({
            "host": request.host,
            "port": port,
            "https": request.https,
            "path": request.path,
        }))?;
        let result = self.diagnostics.diagnose(dto).await;
        Response::ok(NetworkingDiagnoseDomainsResponse {
            diagnostic: ::buffa::MessageField::some(diagnostic_to_proto(&result)),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn diagnose_root_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, NetworkingDiagnoseRootNetworkRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<NetworkingDiagnoseRootNetworkResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        let result = self.root_network.diagnose(server_id).await.map_err(map_error)?;
        Response::ok(NetworkingDiagnoseRootNetworkResponse {
            status: ::buffa::MessageField::some(root_network_to_proto(&result)),
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn repair_root_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, NetworkingRepairRootNetworkRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<NetworkingRepairRootNetworkResponse> + Send + use<'a>,
    > {
        let server_id = request.server_id;
        let result = self.root_network.repair(server_id).await.map_err(map_error)?;
        Response::ok(NetworkingRepairRootNetworkResponse {
            status: ::buffa::MessageField::some(root_network_to_proto(&result)),
            ..Default::default()
        })
    }
}
