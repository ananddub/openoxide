use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1 as proto;
use crate::services::identity::permission::group_service::{
    AddOrganizationMemberDto, CreateOrganizationInviteDto,
    ReplaceUserPoliciesDto, SavePermissionGroupDto,
};
use crate::services::identity::permission::PermissionGroupError;

use super::helper::*;
use super::init::PermissionGroupController;

#[auto_grpc::service(connect = true, auth = true)]
impl proto::PermissionService for PermissionGroupController {
    #[permission(Groups, CanRead)]
    async fn list_permissions<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let items = groups.into_iter().map(group_to_resource).collect();
        Response::ok(proto::ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Groups, CanRead)]
    async fn get_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let group = groups.into_iter().find(|g| g.id == request.id)
            .ok_or_else(|| ConnectError::not_found("group not found"))?;
        Response::ok(group_to_resource(group))
    }

    #[permission(Groups, CanCreate)]
    async fn create_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let dto: SavePermissionGroupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let id = self.service.create_group(auth.user_id(), auth.organization_id, dto)
            .await.map_err(map_error)?;
        let payload = serde_json::to_string(&serde_json::json!({ "id": id })).unwrap_or_default();
        Response::ok(proto::ResourceResponse {
            id,
            kind: "permission_group".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Groups, CanUpdate)]
    async fn update_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let dto: SavePermissionGroupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.update_group(auth.user_id(), auth.organization_id, request.id, dto)
            .await.map_err(map_error)?;
        Response::ok(proto::ResourceResponse {
            id: request.id,
            kind: "permission_group".into(),
            payload: "{}".into(),
            ..Default::default()
        })
    }

    #[permission(Groups, CanDelete)]
    async fn delete_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::Empty> + Send + use<'a>> {
        self.service.delete_group(auth.user_id(), auth.organization_id, request.id)
            .await.map_err(map_error)?;
        Response::ok(proto::Empty::default())
    }

    #[permission(Groups, CanRead)]
    async fn list_groups<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let items = groups.into_iter().map(group_to_resource).collect();
        Response::ok(proto::ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Groups, CanRead)]
    async fn get_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let groups = self.service.list_groups(auth.organization_id).await.map_err(map_error)?;
        let group = groups.into_iter().find(|g| g.id == request.id)
            .ok_or_else(|| ConnectError::not_found("group not found"))?;
        Response::ok(group_to_resource(group))
    }

    #[permission(Groups, CanCreate)]
    async fn create_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let dto: SavePermissionGroupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let id = self.service.create_group(auth.user_id(), auth.organization_id, dto)
            .await.map_err(map_error)?;
        let payload = serde_json::to_string(&serde_json::json!({ "id": id })).unwrap_or_default();
        Response::ok(proto::ResourceResponse {
            id,
            kind: "permission_group".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Groups, CanUpdate)]
    async fn update_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let dto: SavePermissionGroupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.update_group(auth.user_id(), auth.organization_id, request.id, dto)
            .await.map_err(map_error)?;
        Response::ok(proto::ResourceResponse {
            id: request.id,
            kind: "permission_group".into(),
            payload: "{}".into(),
            ..Default::default()
        })
    }

    #[permission(Groups, CanDelete)]
    async fn delete_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::Empty> + Send + use<'a>> {
        self.service.delete_group(auth.user_id(), auth.organization_id, request.id)
            .await.map_err(map_error)?;
        Response::ok(proto::Empty::default())
    }

    async fn list_policies<'a>(
        &'a self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<'a>> {
        let policies = self.service.list_policies().await.map_err(map_error)?;
        let items = policies.into_iter().map(policy_to_resource).collect();
        Response::ok(proto::ResourceListResponse { items, ..Default::default() })
    }

    // Auth-only: members must be able to read their own grants to gate the
    // UI; the payload only ever describes the caller.
    async fn get_my_permissions<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let (role, actions) = self
            .permissions
            .my_permissions(auth.user_id(), auth.organization_id)
            .await
            .map_err(PermissionGroupError::from)
            .map_err(map_error)?;
        let payload = serde_json::to_string(&MyPermissionsDto {
            role,
            organization_id: auth.organization_id,
            actions,
        })
        .unwrap_or_default();
        Response::ok(proto::ResourceResponse {
            id: auth.organization_id,
            kind: "my_permissions".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Members, CanUpdate)]
    async fn assign_member_group<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::OperationResult> + Send + use<'a>> {
        let dto: AssignPermissionGroupDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.assign_group(auth.user_id(), auth.organization_id, request.id, dto.group_id)
            .await.map_err(map_error)?;
        Response::ok(proto::OperationResult {
            success: true,
            operation: "assign_member_group".into(),
            ..Default::default()
        })
    }

    #[permission(Members, CanRead)]
    async fn list_members<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<'a>> {
        let members = self.service.list_members(auth.organization_id).await.map_err(map_error)?;
        let items = members.into_iter()
            .map(|m| OrganizationMemberDto {
                id: m.id,
                user_id: m.user_id.unwrap_or_default(),
                email: m.email.unwrap_or_default(),
                avatar: Some(m.avatar),
                organization_id: m.organization_id.unwrap_or_default(),
                role: m.role,
                created_at: m.created_at.unwrap_or_default(),
                updated_at: m.updated_at.unwrap_or_default(),
            })
            .map(member_to_resource)
            .collect();
        Response::ok(proto::ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Members, CanCreate)]
    async fn add_member<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::OperationResult> + Send + use<'a>> {
        let dto: AddOrganizationMemberDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.add_member(auth.user_id(), auth.organization_id, dto)
            .await.map_err(map_error)?;
        Response::ok(proto::OperationResult {
            success: true,
            operation: "add_member".into(),
            ..Default::default()
        })
    }

    #[permission(Members, CanUpdate)]
    async fn update_member_role<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::OperationResult> + Send + use<'a>> {
        let dto: UpdateMemberRoleDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.update_member_role(auth.user_id(), auth.organization_id, request.id, dto.role)
            .await.map_err(map_error)?;
        Response::ok(proto::OperationResult {
            success: true,
            operation: "update_member_role".into(),
            ..Default::default()
        })
    }

    #[permission(Members, CanDelete)]
    async fn remove_member<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::Empty> + Send + use<'a>> {
        self.service.remove_member(auth.user_id(), auth.organization_id, request.id)
            .await.map_err(map_error)?;
        Response::ok(proto::Empty::default())
    }

    #[permission(Invitation, CanRead)]
    async fn list_invites<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<'a>> {
        let invites = self.service.list_invites(auth.organization_id).await.map_err(map_error)?;
        let items = invites.into_iter().map(OrganizationInviteDto::from).map(invite_to_resource).collect();
        Response::ok(proto::ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Invitation, CanCreate)]
    async fn create_invite<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<'a>> {
        let dto: CreateOrganizationInviteDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let token = self.service.create_invite(auth.user_id(), auth.organization_id, dto)
            .await.map_err(map_error)?;
        let payload = serde_json::to_string(&serde_json::json!({ "token": token })).unwrap_or_default();
        Response::ok(proto::ResourceResponse {
            id: 0,
            kind: "organization_invite".into(),
            payload,
            ..Default::default()
        })
    }

    async fn accept_invite<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::OperationResult> + Send + use<'a>> {
        let dto: AcceptOrganizationInviteDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.accept_invite(auth.user_id(), dto.token)
            .await.map_err(map_error)?;
        Response::ok(proto::OperationResult {
            success: true,
            operation: "accept_invite".into(),
            ..Default::default()
        })
    }

    #[permission(Invitation, CanDelete)]
    async fn cancel_invite<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::Empty> + Send + use<'a>> {
        self.service.cancel_invite(auth.user_id(), auth.organization_id, request.id)
            .await.map_err(map_error)?;
        Response::ok(proto::Empty::default())
    }

    #[permission(Members, CanUpdate)]
    async fn replace_user_policies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<proto::OperationResult> + Send + use<'a>> {
        let dto: ReplaceUserPoliciesDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service.replace_user_policies(auth.user_id(), auth.organization_id, request.id, dto)
            .await.map_err(map_error)?;
        Response::ok(proto::OperationResult {
            success: true,
            operation: "replace_user_policies".into(),
            ..Default::default()
        })
    }
}
