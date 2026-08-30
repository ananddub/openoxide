use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CertificateOperationsService, CertificateOperationsServiceExt, Empty, OperationResult,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::networking::certificate::{CreateCertificateDto, PatchCertificateDto, RenewCertificateDto};

use super::helper::*;
use super::init::CertificateController;

#[auto_grpc::service(connect = true, auth = true)]
impl CertificateOperationsService for CertificateController {
    #[permission(Certificate, CanRead)]
    async fn list_certificates<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list(auth.organization_id).await.map_err(map_error)?;
        let items = items.into_iter().map(CertificateResponseDto::from).map(cert_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Certificate, CanRead)]
    async fn get_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let item = self.service.get_by_id(request.id, auth.organization_id).await.map_err(map_error)?;
        Response::ok(cert_to_resource(CertificateResponseDto::from(item)))
    }

    #[permission(Certificate, CanCreate)]
    async fn create_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateCertificateDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.service.create(dto, auth.organization_id).await.map_err(map_error)?;
        Response::ok(cert_to_resource(CertificateResponseDto::from(item)))
    }

    #[permission(Certificate, CanUpdate)]
    async fn update_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchCertificateDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.service.patch(request.id, dto, auth.organization_id).await.map_err(map_error)?;
        Response::ok(cert_to_resource(CertificateResponseDto::from(item)))
    }

    #[permission(Certificate, CanDelete)]
    async fn delete_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id, auth.organization_id).await.map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Certificate, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let deps = self.service.dependencies(request.id, auth.organization_id).await.map_err(map_error)?;
        let payload = serde_json::to_string(&deps).unwrap_or_default();
        let items = vec![ResourceResponse {
            id: request.id,
            kind: "certificate_dependencies".into(),
            payload,
            ..Default::default()
        }];
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Certificate, CanUpdate)]
    async fn renew_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: RenewCertificateDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let item = self.service.renew(request.id, dto, auth.organization_id).await.map_err(map_error)?;
        let payload = serde_json::to_string(&CertificateResponseDto::from(item)).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "renew_certificate".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Certificate, CanRead)]
    async fn list_renewals<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let rows = self.service.renewal_history(request.id, auth.organization_id).await.map_err(map_error)?;
        let items = rows.into_iter().map(CertificateRenewalDto::from).map(renewal_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }
}
