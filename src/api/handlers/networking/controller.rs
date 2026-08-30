use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    NetworkingOperationsService, NetworkingOperationsServiceExt, OperationResult,
    ResourceRequest, ResourceResponse,
};

use super::helper::*;
use super::init::NetworkingController;

#[auto_grpc::service(connect = true, auth = true)]
impl NetworkingOperationsService for NetworkingController {
    #[permission(Application, CanCreate)]
    async fn purge_cdn<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.cdn.purge(dto).await.map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "purge_cdn".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn diagnose_domains<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.diagnostics.diagnose(dto).await;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "domain_diagnostic".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn diagnose_root_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: RootNetworkDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.root_network.diagnose(dto.server_id).await.map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "root_network_status".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn repair_root_network<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: RootNetworkDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self.root_network.repair(dto.server_id).await.map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "repair_root_network".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }
}
