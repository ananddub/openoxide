use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;
use std::sync::Arc;

use crate::api::connect::generated::openoxide::api::v1 as proto;
use crate::db::reactive::realtime;
use crate::flash_gen::models::{Groups, OrganizationInvites, OrganizationMembers, Policy};
use crate::flash_gen::organization_members::OrganizationMembersListWithUserForOrganizationRow;
use crate::services::identity::permission::group_service::{
    AddOrganizationMemberDto, CreateOrganizationInviteDto, PermissionPolicyDto,
    ReplaceUserPoliciesDto, SavePermissionGroupDto,
};

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
    async fn stream_list_permissions(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT g.* FROM groups g LEFT JOIN group_policy gp ON gp.group_id = g.id LEFT JOIN policy p ON p.id = gp.policy_id WHERE g.organization_id IS NULL OR g.organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<Groups>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.list_groups(organization_id).await {
                    Ok(groups) => {
                        let items = groups.into_iter().map(group_to_resource).collect();
                        Ok(proto::ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_error(e)),
                }
            }
        }))
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

    #[permission(Groups, CanRead)]
    async fn stream_permission(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let id = request.id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT g.* FROM groups g LEFT JOIN group_policy gp ON gp.group_id = g.id LEFT JOIN policy p ON p.id = gp.policy_id WHERE g.id = {id} AND (g.organization_id IS NULL OR g.organization_id = {organization_id})"
            );
            realtime()
                .watch_query::<Groups>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.then(move |mut rows| {
            let service = Arc::clone(&service);
            async move {
                match rows.pop() {
                    Some(_) => match service.list_groups(organization_id).await {
                        Ok(groups) => {
                            let group = groups.into_iter().find(|g| g.id == id)
                                .ok_or_else(|| ConnectError::not_found("group not found"))?;
                            Ok(group_to_resource(group))
                        }
                        Err(e) => Err(map_error(e)),
                    },
                    None => Err(ConnectError::not_found("group not found")),
                }
            }
        }))
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
    async fn stream_list_groups(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT g.* FROM groups g LEFT JOIN group_policy gp ON gp.group_id = g.id LEFT JOIN policy p ON p.id = gp.policy_id WHERE g.organization_id IS NULL OR g.organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<Groups>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.list_groups(organization_id).await {
                    Ok(groups) => {
                        let items = groups.into_iter().map(group_to_resource).collect();
                        Ok(proto::ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_error(e)),
                }
            }
        }))
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

    #[permission(Groups, CanRead)]
    async fn stream_group(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, proto::ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let id = request.id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT g.* FROM groups g LEFT JOIN group_policy gp ON gp.group_id = g.id LEFT JOIN policy p ON p.id = gp.policy_id WHERE g.id = {id} AND (g.organization_id IS NULL OR g.organization_id = {organization_id})"
            );
            realtime()
                .watch_query::<Groups>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.then(move |mut rows| {
            let service = Arc::clone(&service);
            async move {
                match rows.pop() {
                    Some(_) => match service.list_groups(organization_id).await {
                        Ok(groups) => {
                            let group = groups.into_iter().find(|g| g.id == id)
                                .ok_or_else(|| ConnectError::not_found("group not found"))?;
                            Ok(group_to_resource(group))
                        }
                        Err(e) => Err(map_error(e)),
                    },
                    None => Err(ConnectError::not_found("group not found")),
                }
            }
        }))
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

    async fn stream_list_policies(
        &self,
        _ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<>,
        >,
    > {
        let stream = {
            let sql = "SELECT * FROM policy".to_string();
            realtime()
                .watch_query::<Policy>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(proto::ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(|p| PermissionPolicyDto { id: p.id, action: p.action })
                    .map(policy_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
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

    #[permission(Members, CanRead)]
    async fn stream_list_members(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT om.* FROM organization_members om JOIN users u ON u.id = om.user_id WHERE om.organization_id = {organization_id}"
            );
            realtime()
                .watch_query::<OrganizationMembers>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.list_members(organization_id).await {
                    Ok(members) => {
                        let items = members
                            .into_iter()
                            .map(|m: OrganizationMembersListWithUserForOrganizationRow| OrganizationMemberDto {
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
                        Ok(proto::ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_error(e)),
                }
            }
        }))
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

    #[permission(Invitation, CanRead)]
    async fn stream_list_invites(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, proto::ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<proto::ResourceListResponse> + Send + use<>,
        >,
    > {
        let organization_id = auth.organization_id;
        let service = Arc::clone(&self.service);
        let stream = {
            let sql = format!(
                "SELECT * FROM organization_invites WHERE organization_id = {organization_id} AND status = 'PENDING' AND expired_at > CURRENT_TIMESTAMP"
            );
            realtime()
                .watch_query::<OrganizationInvites>(sql)
                .await
                .map_err(|e| map_error(e.into()))?
        };
        Response::stream_ok(stream.then(move |_rows| {
            let service = Arc::clone(&service);
            async move {
                match service.list_invites(organization_id).await {
                    Ok(invites) => {
                        let items = invites
                            .into_iter()
                            .map(OrganizationInviteDto::from)
                            .map(invite_to_resource)
                            .collect();
                        Ok(proto::ResourceListResponse { items, ..Default::default() })
                    }
                    Err(e) => Err(map_error(e)),
                }
            }
        }))
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
