use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};

use crate::{
    api::dto::permission::{
        AcceptOrganizationInviteDto, AssignPermissionGroupDto, CreateOrganizationInviteDto,
        OrganizationInviteDto, OrganizationMemberDto, PermissionGroupDto, PermissionPolicyDto,
        ReplaceUserPoliciesDto, SavePermissionGroupDto, UpdateMemberRoleDto,
    },
    core::middleware::{
        permission::{
            CanCreate, CanDelete, CanRead, CanUpdate, Groups, Invitation, Members,
            PermissionOrganization, RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::permission::{PermissionGroupError, PermissionGroupService},
};

type ApiError = (StatusCode, String);

pub struct PermissionGroupController {
    service: Arc<PermissionGroupService>,
}

#[controller("/permission-groups")]
impl PermissionGroupController {
    fn new(service: Arc<PermissionGroupService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(tables = ["groups","policy","group_policy","user_policy","organization_members","organization_invites"])]
    async fn list(
        &self,
        RequirePermission(_, _): RequirePermission<Groups, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
    ) -> Result<Json<Vec<PermissionGroupDto>>, ApiError> {
        self.service
            .list_groups(organization_id)
            .await
            .map(Json)
            .map_err(map_error)
    }

    #[get("/policies")]
    #[live(tables = ["groups","policy","group_policy","user_policy","organization_members","organization_invites"])]
    async fn policies(
        &self,
        RequirePermission(_, _): RequirePermission<Groups, CanRead>,
    ) -> Result<Json<Vec<PermissionPolicyDto>>, ApiError> {
        self.service
            .list_policies()
            .await
            .map(Json)
            .map_err(map_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(claims, _): RequirePermission<Groups, CanCreate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        ValidatedJson(body): ValidatedJson<SavePermissionGroupDto>,
    ) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
        let id = self
            .service
            .create_group(claims.user.user_id, organization_id, body)
            .await
            .map_err(map_error)?;
        Ok((StatusCode::CREATED, Json(serde_json::json!({ "id": id }))))
    }

    #[put("/{group_id}")]
    async fn update(
        &self,
        RequirePermission(claims, _): RequirePermission<Groups, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(group_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<SavePermissionGroupDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .update_group(claims.user.user_id, organization_id, group_id, body)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[delete("/{group_id}")]
    async fn delete(
        &self,
        RequirePermission(_, _): RequirePermission<Groups, CanDelete>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        claims: crate::utils::jwt::claim::Claims,
        Path(group_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete_group(claims.user.user_id, organization_id, group_id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[put("/members/{user_id}/group")]
    async fn assign_group(
        &self,
        RequirePermission(claims, _): RequirePermission<Members, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(user_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<AssignPermissionGroupDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .assign_group(claims.user.user_id, organization_id, user_id, body.group_id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[get("/members")]
    #[live(tables = ["groups","policy","group_policy","user_policy","organization_members","organization_invites"])]
    async fn members(
        &self,
        RequirePermission(_, _): RequirePermission<Members, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
    ) -> Result<Json<Vec<OrganizationMemberDto>>, ApiError> {
        self.service
            .list_members(organization_id)
            .await
            .map(|members| {
                members
                    .into_iter()
                    .map(|member| OrganizationMemberDto {
                        id: member.id,
                        user_id: member.user_id,
                        email: member.email,
                        avatar: member.avatar,
                        organization_id: member.organization_id,
                        role: member.role,
                        created_at: member.created_at,
                        updated_at: member.updated_at,
                    })
                    .collect()
            })
            .map(Json)
            .map_err(map_error)
    }

    #[post("/members")]
    async fn add_member(
        &self,
        RequirePermission(claims, _): RequirePermission<Members, CanCreate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        ValidatedJson(body): ValidatedJson<crate::api::dto::permission::AddOrganizationMemberDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .add_member(claims.user.user_id, organization_id, body)
            .await
            .map(|()| StatusCode::CREATED)
            .map_err(map_error)
    }

    #[put("/members/{user_id}/role")]
    async fn update_member_role(
        &self,
        RequirePermission(claims, _): RequirePermission<Members, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(user_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpdateMemberRoleDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .update_member_role(claims.user.user_id, organization_id, user_id, body.role)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[delete("/members/{user_id}")]
    async fn remove_member(
        &self,
        RequirePermission(claims, _): RequirePermission<Members, CanDelete>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(user_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .remove_member(claims.user.user_id, organization_id, user_id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[get("/invites")]
    #[live(tables = ["groups","policy","group_policy","user_policy","organization_members","organization_invites"])]
    async fn invites(
        &self,
        RequirePermission(_, _): RequirePermission<Invitation, CanRead>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
    ) -> Result<Json<Vec<OrganizationInviteDto>>, ApiError> {
        self.service
            .list_invites(organization_id)
            .await
            .map(|items| items.into_iter().map(OrganizationInviteDto::from).collect())
            .map(Json)
            .map_err(map_error)
    }

    #[post("/invites")]
    async fn invite(
        &self,
        RequirePermission(claims, _): RequirePermission<Invitation, CanCreate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        ValidatedJson(body): ValidatedJson<CreateOrganizationInviteDto>,
    ) -> Result<(StatusCode, Json<serde_json::Value>), ApiError> {
        let token = self
            .service
            .create_invite(claims.user.user_id, organization_id, body)
            .await
            .map_err(map_error)?;
        Ok((
            StatusCode::CREATED,
            Json(serde_json::json!({ "token": token })),
        ))
    }

    #[post("/invites/accept")]
    async fn accept_invite(
        &self,
        claims: crate::utils::jwt::claim::Claims,
        Json(body): Json<AcceptOrganizationInviteDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .accept_invite(claims.user.user_id, body.token)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[delete("/invites/{invite_id}")]
    async fn cancel_invite(
        &self,
        RequirePermission(_, _): RequirePermission<Invitation, CanDelete>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        claims: crate::utils::jwt::claim::Claims,
        Path(invite_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .cancel_invite(claims.user.user_id, organization_id, invite_id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }

    #[put("/members/{user_id}/policies")]
    async fn replace_user_policies(
        &self,
        RequirePermission(claims, _): RequirePermission<Members, CanUpdate>,
        Extension(PermissionOrganization(organization_id)): Extension<PermissionOrganization>,
        Path(user_id): Path<i64>,
        Json(body): Json<ReplaceUserPoliciesDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .replace_user_policies(claims.user.user_id, organization_id, user_id, body)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }
}

fn map_error(error: PermissionGroupError) -> ApiError {
    match error {
        PermissionGroupError::Invalid(message) => (StatusCode::BAD_REQUEST, message),
        PermissionGroupError::Escalation(message) => (StatusCode::FORBIDDEN, message),
        PermissionGroupError::NotFound => (StatusCode::NOT_FOUND, error.to_string()),
        PermissionGroupError::Database(database_error)
            if database_error
                .as_database_error()
                .is_some_and(|error| error.is_unique_violation()) =>
        {
            (StatusCode::CONFLICT, "group name already exists".into())
        }
        PermissionGroupError::Database(database_error) => {
            tracing::error!(%database_error, "permission group operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "permission operation failed".into(),
            )
        }
    }
}
