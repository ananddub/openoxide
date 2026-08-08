use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};

use crate::{
    api::dto::permission::{
        AssignPermissionGroupDto, PermissionGroupDto, PermissionPolicyDto, ReplaceUserPoliciesDto,
        SavePermissionGroupDto,
    },
    core::middleware::{
        permission::{
            CanCreate, CanDelete, CanRead, CanUpdate, Groups, Members, PermissionOrganization,
            RequirePermission,
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
        Path(group_id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete_group(organization_id, group_id)
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
