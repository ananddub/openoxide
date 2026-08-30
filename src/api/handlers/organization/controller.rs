use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateOrganizationRequest, DeleteOrganizationRequest, Empty, GetOrganizationRequest,
    ListOrganizationsRequest, ListOrganizationsResponse,
    Organization as OrganizationProto, OrganizationService, OrganizationServiceExt,
    UpdateOrganizationRequest,
};

use super::helper::*;
use super::init::OrganizationController;

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
