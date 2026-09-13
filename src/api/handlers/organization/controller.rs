use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    CreateOrganizationRequest, DeleteOrganizationRequest, Empty, GetOrganizationRequest,
    LeaveOrganizationRequest, ListOrganizationsRequest, ListOrganizationsResponse,
    Organization as OrganizationProto, OrganizationService, OrganizationServiceExt,
    UpdateOrganizationRequest,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::Organization;

use super::helper::*;
use super::init::OrganizationController;

#[auto_grpc::service(connect = true, auth = true)]
impl OrganizationService for OrganizationController {
    async fn stream_get_organization(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetOrganizationRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<OrganizationProto> + Send + use<>,
        >,
    > {
        let id = request.id;
        if !self
            .service
            .is_member(id, auth.user_id())
            .await
            .map_err(map_db_error)?
        {
            return Err(ConnectError::permission_denied("not a member of this organization"));
        }
        let stream = {
            let sql = format!("SELECT * FROM organization WHERE id = {id}");
            realtime()
                .watch_query::<Organization>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(org_to_proto(item)),
            None => Err(ConnectError::not_found("organization not found")),
        }))
    }

    async fn get_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OrganizationProto> + Send + use<'a>> {
        // Membership check: reading an arbitrary organization by id must
        // not leak its data to users outside it.
        if !self
            .service
            .is_member(request.id, auth.user_id())
            .await
            .map_err(map_db_error)?
        {
            return Err(ConnectError::permission_denied("not a member of this organization"));
        }
        let org = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
        Response::ok(org_to_proto(org))
    }

    async fn list_organizations<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListOrganizationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListOrganizationsResponse> + Send + use<'a>> {
        let orgs = self.service.list_for_user(auth.user_id()).await.map_err(map_db_error)?;
        let organizations = orgs
            .into_iter()
            .map(|(org, role)| org_with_role_to_proto(org, role))
            .collect();
        Response::ok(ListOrganizationsResponse {
            organizations,
            ..Default::default()
        })
    }

    // Auth-only (no permission macro): leaving is a self-service action;
    // the owner of an organization cannot leave it (delete or transfer
    // instead), which the service enforces.
    async fn leave_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, LeaveOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .leave(request.id, auth.user_id())
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
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
        // The permission macro scoped the ACTIVE organization; refuse to
        // touch a different one through this call.
        if dto.id != auth.organization_id {
            return Err(ConnectError::permission_denied(
                "switch to the organization before editing it",
            ));
        }
        let org = self.service.update(dto.id, dto).await.map_err(map_db_error)?;
        Response::ok(org_to_proto(org))
    }

    #[permission(Organization, CanWrite)]
    async fn delete_organization<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteOrganizationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        if request.id != auth.organization_id {
            return Err(ConnectError::permission_denied(
                "switch to the organization before deleting it",
            ));
        }
        // Only the owner may delete the organization itself; the org
        // ADMIN role must not be able to destroy someone else's workspace.
        let org = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
        if org.owner_id != auth.user_id() {
            return Err(ConnectError::permission_denied(
                "only the organization owner can delete it",
            ));
        }
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
