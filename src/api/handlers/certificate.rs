use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CertificateOperationsService, CertificateOperationsServiceExt, Empty, OperationResult,
    ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::certificate::{
    CertificateService as AppCertificateService, CreateCertificateDto, PatchCertificateDto,
    RenewCertificateDto,
};

pub struct CertificateController {
    service: Arc<AppCertificateService>,
}

#[singleton]
impl CertificateController {
    pub fn new(service: Arc<AppCertificateService>) -> Self {
        Self { service }
    }
}

fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("certificate not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "certificate database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn cert_to_resource(dto: CertificateResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: 0,
        kind: "certificate".into(),
        payload,
        ..Default::default()
    }
}

fn renewal_to_resource(dto: CertificateRenewalDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "certificate_renewal".into(),
        payload,
        ..Default::default()
    }
}

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

// ── Helper DTOs for serializing ResourceResponse payloads ────────────

use crate::flash_gen::models::{CertificateRenewals, Certificates};

#[derive(Debug, Clone, serde::Serialize)]
struct CertificateResponseDto {
    id: String,
    name: String,
    certificate_data: String,
    has_private_key: bool,
    certificate_path: String,
    auto_renew: i64,
    server_id: Option<String>,
    organization_id: i64,
    created_at: i64,
    updated_at: i64,
}

impl From<Certificates> for CertificateResponseDto {
    fn from(val: Certificates) -> Self {
        Self {
            id: val.id.to_string(),
            name: val.name,
            certificate_data: val.certificate_data,
            has_private_key: !val.private_key.is_empty(),
            certificate_path: val.certificate_path,
            auto_renew: val.auto_renew,
            server_id: val.server_id.map(|id| id.to_string()),
            organization_id: val.organization_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
struct CertificateRenewalDto {
    id: i64,
    certificate_id: i64,
    organization_id: i64,
    status: String,
    previous_expires_at: Option<i64>,
    new_expires_at: Option<i64>,
    error: Option<String>,
    started_at: i64,
    finished_at: Option<i64>,
}

impl From<CertificateRenewals> for CertificateRenewalDto {
    fn from(v: CertificateRenewals) -> Self {
        Self {
            id: v.id,
            certificate_id: v.certificate_id,
            organization_id: v.organization_id,
            status: v.status,
            previous_expires_at: v.previous_expires_at,
            new_expires_at: v.new_expires_at,
            error: v.error,
            started_at: v.started_at,
            finished_at: v.finished_at,
        }
    }
}
