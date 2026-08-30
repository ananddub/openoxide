use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateOrganizationRequest, DeleteOrganizationRequest, Empty, GetOrganizationRequest,
    ListOrganizationsRequest, ListOrganizationsResponse,
    Organization as OrganizationProto, OrganizationService, OrganizationServiceExt,
    UpdateOrganizationRequest,
};
use crate::services::organization::OrganizationService as AppOrganizationService;

pub struct OrganizationController {
    service: Arc<AppOrganizationService>,
}

#[singleton]
impl OrganizationController {
    pub fn new(service: Arc<AppOrganizationService>) -> Self {
        Self { service }
    }
}

fn org_to_proto(org: crate::flash_gen::models::Organization) -> OrganizationProto {
    OrganizationProto {
        id: org.id,
        name: org.name,
        logo: org.logo,
        slug: org.slug,
        owner_id: org.owner_id,
        created_at: org.created_at,
        updated_at: org.updated_at,
        ..Default::default()
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("organization not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("owner not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "organization database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl OrganizationService for OrganizationController {
    async fn get_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OrganizationProto> + Send + use<'a>> {
        let org = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
        Response::ok(org_to_proto(org))
    }

    async fn list_organizations<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListOrganizationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListOrganizationsResponse> + Send + use<'a>> {
        let orgs = self.service.list_by_owner(auth.user_id()).await.map_err(map_db_error)?;
        let organizations = orgs.into_iter().map(org_to_proto).collect();
        Response::ok(ListOrganizationsResponse {
            organizations,
            ..Default::default()
        })
    }

    async fn create_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OrganizationProto> + Send + use<'a>> {
        let org = self
            .service
            .create(auth.user_id(), request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(org_to_proto(org))
    }

    #[permission(Organization, CanWrite)]
    async fn update_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OrganizationProto> + Send + use<'a>> {
        let dto = request.to_owned_message();
        let org = self.service.update(dto.id, dto).await.map_err(map_db_error)?;
        Response::ok(org_to_proto(org))
    }

    #[permission(Organization, CanWrite)]
    async fn delete_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
