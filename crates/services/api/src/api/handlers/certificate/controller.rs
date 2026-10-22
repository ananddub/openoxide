use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CertificateService, CertificateServiceExt, CreateCertificateRequest, CreateCertificateResponse,
    DeleteCertificateRequest, Empty, GetCertificateRequest, GetCertificateResponse,
    ListCertificateDependenciesRequest, ListCertificateDependenciesResponse,
    ListCertificateRenewalsRequest, ListCertificateRenewalsResponse, ListCertificatesRequest,
    ListCertificatesResponse, RenewCertificateRequest, RenewCertificateResponse,
    UpdateCertificateRequest, UpdateCertificateResponse,
};
use crate::services::networking::certificate::{
    CreateCertificateDto, PatchCertificateDto, RenewCertificateDto,
};

use super::helper::*;
use super::init::CertificateController;

#[auto_grpc::service(connect = true, auth = true)]
impl CertificateService for CertificateController {
    #[permission(Certificate, CanRead)]
    async fn list_certificates<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListCertificatesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListCertificatesResponse> + Send + use<'a>> {
        let items = self
            .service
            .list(auth.organization_id)
            .await
            .map_err(map_error)?;
        let certificates = items.into_iter().map(cert_to_proto).collect();
        Response::ok(ListCertificatesResponse { certificates })
    }

    #[permission(Certificate, CanRead)]
    async fn get_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetCertificateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetCertificateResponse> + Send + use<'a>> {
        let item = self
            .service
            .get_by_id(request.id, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(GetCertificateResponse {
            certificate: ::buffa::MessageField::some(cert_to_proto(item)),
        })
    }

    #[permission(Certificate, CanCreate)]
    async fn create_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateCertificateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateCertificateResponse> + Send + use<'a>> {
        let dto = CreateCertificateDto {
            name: request.name.to_owned(),
            certificate_data: request.certificate_data.to_owned(),
            private_key: request.private_key.to_owned(),
            certificate_path: request.certificate_path.to_owned(),
            auto_renew: i64::from(request.auto_renew),
            server_id: request.server_id.map(str::to_owned),
        };
        let item = self
            .service
            .create(dto, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(CreateCertificateResponse {
            certificate: ::buffa::MessageField::some(cert_to_proto(item)),
        })
    }

    #[permission(Certificate, CanUpdate)]
    async fn update_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateCertificateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateCertificateResponse> + Send + use<'a>> {
        let dto = PatchCertificateDto {
            name: request.name.map(str::to_owned),
            certificate_data: request.certificate_data.map(str::to_owned),
            private_key: request.private_key.map(str::to_owned),
            certificate_path: request.certificate_path.map(str::to_owned),
            auto_renew: request.auto_renew.map(i64::from),
            // Present-but-empty detaches the server (Some(None)).
            server_id: request
                .server_id
                .map(|value| (!value.is_empty()).then(|| value.to_owned())),
        };
        let item = self
            .service
            .patch(request.id, dto, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(UpdateCertificateResponse {
            certificate: ::buffa::MessageField::some(cert_to_proto(item)),
        })
    }

    #[permission(Certificate, CanDelete)]
    async fn delete_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteCertificateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Certificate, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListCertificateDependenciesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ListCertificateDependenciesResponse> + Send + use<'a>,
    > {
        let deps = self
            .service
            .dependencies(request.certificate_id, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(ListCertificateDependenciesResponse {
            dependencies: ::buffa::MessageField::some(dependencies_to_proto(deps)),
        })
    }

    #[permission(Certificate, CanUpdate)]
    async fn renew_certificate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RenewCertificateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RenewCertificateResponse> + Send + use<'a>> {
        let dto = RenewCertificateDto {
            certificate_data: request.certificate_data.to_owned(),
            private_key: request.private_key.to_owned(),
        };
        let item = self
            .service
            .renew(request.id, dto, auth.organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(RenewCertificateResponse {
            certificate: ::buffa::MessageField::some(cert_to_proto(item)),
        })
    }

    #[permission(Certificate, CanRead)]
    async fn list_renewals<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListCertificateRenewalsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListCertificateRenewalsResponse> + Send + use<'a>>
    {
        let rows = self
            .service
            .renewal_history(request.certificate_id, auth.organization_id)
            .await
            .map_err(map_error)?;
        let renewals = rows.into_iter().map(renewal_to_proto).collect();
        Response::ok(ListCertificateRenewalsResponse { renewals })
    }
}
