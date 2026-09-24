use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreatePermissionRequest, CreatePermissionResponse, DeletePermissionRequest,
    Empty, GetMyPermissionsRequest, GetMyPermissionsResponse, GetPermissionRequest,
    GetPermissionResponse, ListPermissionsRequest, ListPermissionsResponse,
    PermissionAcceptInviteRequest, PermissionAddMemberRequest, PermissionAssignMemberGroupRequest,
    PermissionCancelInviteRequest, PermissionCreateGroupRequest, PermissionCreateGroupResponse,
    PermissionCreateInviteRequest, PermissionCreateInviteResponse, PermissionDeleteGroupRequest,
    PermissionGetGroupRequest, PermissionGetGroupResponse, PermissionListGroupsRequest,
    PermissionListGroupsResponse, PermissionListInvitesRequest, PermissionListInvitesResponse,
    PermissionListMembersRequest, PermissionListMembersResponse, PermissionListPoliciesRequest,
    PermissionListPoliciesResponse, PermissionRemoveMemberRequest,
    PermissionReplaceUserPoliciesRequest, PermissionService, PermissionServiceExt,
    PermissionUpdateGroupRequest, PermissionUpdateMemberRoleRequest, UpdatePermissionRequest,
};
use crate::services::identity::permission::group_service::{
    AddOrganizationMemberDto, CreateOrganizationInviteDto, ReplaceUserPoliciesDto,
    SavePermissionGroupDto, UserPolicyEntryDto,
};
use crate::services::identity::permission::PermissionGroupError;

use super::helper::*;
use super::init::PermissionGroupController;

#[auto_grpc::service(connect = true, auth = true)]
impl PermissionService for PermissionGroupController {
    // ── Permission* aliases (share the PermissionGroup payload) ────

    #[permission(Groups, CanRead)]
    async fn list_permissions<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListPermissionsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListPermissionsResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let groups = groups.into_iter().map(group_to_proto).collect();
        Response::ok(ListPermissionsResponse { groups, ..Default::default() })
    }

    #[permission(Groups, CanRead)]
    async fn get_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetPermissionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetPermissionResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let group = groups
            .into_iter()
            .find(|g| g.id == request.id)
            .ok_or_else(|| ConnectError::not_found("group not found"))?;
        Response::ok(GetPermissionResponse {
            group: ::buffa::MessageField::some(group_to_proto(group)),
            ..Default::default()
        })
    }

    #[permission(Groups, CanCreate)]
    async fn create_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreatePermissionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreatePermissionResponse> + Send + use<'a>> {
        let dto = SavePermissionGroupDto {
            name: request.name.to_owned(),
            actions: request.actions.iter().map(|s| s.to_string()).collect(),
        };
        let id = self
            .service
            .create_group(auth.user_id(), auth.organization_id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(CreatePermissionResponse { id, ..Default::default() })
    }

    #[permission(Groups, CanUpdate)]
    async fn update_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdatePermissionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let dto = SavePermissionGroupDto {
            name: request.name.to_owned(),
            actions: request.actions.iter().map(|s| s.to_string()).collect(),
        };
        self.service
            .update_group(auth.user_id(), auth.organization_id, request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Groups, CanDelete)]
    async fn delete_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeletePermissionRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete_group(auth.user_id(), auth.organization_id, request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    // ── Groups ─────────────────────────────────────────────────────

    #[permission(Groups, CanRead)]
    async fn list_groups<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, PermissionListGroupsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionListGroupsResponse> + Send + use<'a>>
    {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let groups = groups.into_iter().map(group_to_proto).collect();
        Response::ok(PermissionListGroupsResponse {
            groups,
            ..Default::default()
        })
    }

    #[permission(Groups, CanRead)]
    async fn get_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionGetGroupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionGetGroupResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let group = groups
            .into_iter()
            .find(|g| g.id == request.id)
            .ok_or_else(|| ConnectError::not_found("group not found"))?;
        Response::ok(PermissionGetGroupResponse {
            group: ::buffa::MessageField::some(group_to_proto(group)),
            ..Default::default()
        })
    }

    #[permission(Groups, CanCreate)]
    async fn create_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionCreateGroupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionCreateGroupResponse> + Send + use<'a>>
    {
        let dto = SavePermissionGroupDto {
            name: request.name.to_owned(),
            actions: request.actions.iter().map(|s| s.to_string()).collect(),
        };
        let id = self
            .service
            .create_group(auth.user_id(), auth.organization_id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(PermissionCreateGroupResponse { id, ..Default::default() })
    }

    #[permission(Groups, CanUpdate)]
    async fn update_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionUpdateGroupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let dto = SavePermissionGroupDto {
            name: request.name.to_owned(),
            actions: request.actions.iter().map(|s| s.to_string()).collect(),
        };
        self.service
            .update_group(auth.user_id(), auth.organization_id, request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Groups, CanDelete)]
    async fn delete_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionDeleteGroupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete_group(auth.user_id(), auth.organization_id, request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    // ── Policies ───────────────────────────────────────────────────

    async fn list_policies<'a>(
        &'a self,
        _ctx: RequestContext,
        _request: ServiceRequest<'_, PermissionListPoliciesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionListPoliciesResponse> + Send + use<'a>>
    {
        let policies = self.service.list_policies().await.map_err(map_error)?;
        let policies = policies.into_iter().map(policy_to_proto).collect();
        Response::ok(PermissionListPoliciesResponse {
            policies,
            ..Default::default()
        })
    }

    // Auth-only: members must be able to read their own grants to gate the
    // UI; the payload only ever describes the caller.
    async fn get_my_permissions<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, GetMyPermissionsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetMyPermissionsResponse> + Send + use<'a>> {
        let (role, actions) = self
            .permissions
            .my_permissions(auth.user_id(), auth.organization_id)
            .await
            .map_err(PermissionGroupError::from)
            .map_err(map_error)?;
        Response::ok(GetMyPermissionsResponse {
            role: ::buffa::EnumValue::Known(user_role_str_to_proto(&role)),
            organization_id: auth.organization_id,
            actions,
            ..Default::default()
        })
    }

    // ── Members ────────────────────────────────────────────────────

    #[permission(Members, CanUpdate)]
    async fn assign_member_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionAssignMemberGroupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .assign_group(auth.user_id(), auth.organization_id, request.id, request.group_id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Members, CanRead)]
    async fn list_members<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, PermissionListMembersRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionListMembersResponse> + Send + use<'a>>
    {
        let members = self.service.list_members(auth.organization_id).await.map_err(map_error)?;
        let members = members.into_iter().map(member_to_proto).collect();
        Response::ok(PermissionListMembersResponse {
            members,
            ..Default::default()
        })
    }

    #[permission(Members, CanCreate)]
    async fn add_member<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionAddMemberRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let dto = AddOrganizationMemberDto {
            email: request.email.to_owned(),
            password: request.password.map(str::to_owned),
            role: proto_to_user_role(request.role)?,
        };
        self.service
            .add_member(auth.user_id(), auth.organization_id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Members, CanUpdate)]
    async fn update_member_role<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionUpdateMemberRoleRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let role = proto_to_user_role(request.role)?;
        self.service
            .update_member_role(auth.user_id(), auth.organization_id, request.id, role)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Members, CanDelete)]
    async fn remove_member<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionRemoveMemberRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .remove_member(auth.user_id(), auth.organization_id, request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    // ── Invites ────────────────────────────────────────────────────

    #[permission(Invitation, CanRead)]
    async fn list_invites<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, PermissionListInvitesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionListInvitesResponse> + Send + use<'a>>
    {
        let invites = self.service.list_invites(auth.organization_id).await.map_err(map_error)?;
        let invites = invites.into_iter().map(invite_to_proto).collect();
        Response::ok(PermissionListInvitesResponse {
            invites,
            ..Default::default()
        })
    }

    #[permission(Invitation, CanCreate)]
    async fn create_invite<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionCreateInviteRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<PermissionCreateInviteResponse> + Send + use<'a>>
    {
        let dto = CreateOrganizationInviteDto {
            email: request.email.to_owned(),
            role: proto_to_user_role(request.role)?,
            group_id: request.group_id,
        };
        let token = self
            .service
            .create_invite(auth.user_id(), auth.organization_id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(PermissionCreateInviteResponse {
            token,
            ..Default::default()
        })
    }

    async fn accept_invite<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionAcceptInviteRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let token = request.token.to_owned();
        self.service
            .accept_invite(auth.user_id(), token)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Invitation, CanDelete)]
    async fn cancel_invite<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionCancelInviteRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .cancel_invite(auth.user_id(), auth.organization_id, request.id)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }

    // ── Per-user policy overrides ──────────────────────────────────

    #[permission(Members, CanUpdate)]
    async fn replace_user_policies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, PermissionReplaceUserPoliciesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let mut policies = Vec::new();
        for entry in request.policies.iter() {
            policies.push(UserPolicyEntryDto {
                policy_id: entry.policy_id,
                effect: proto_to_policy_effect(entry.effect)?,
            });
        }
        let dto = ReplaceUserPoliciesDto { policies };
        self.service
            .replace_user_policies(auth.user_id(), auth.organization_id, request.id, dto)
            .await
            .map_err(map_error)?;
        Response::ok(Empty::default())
    }
}
