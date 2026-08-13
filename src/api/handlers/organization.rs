use crate::core::middleware::permission::{CanWrite, Organization};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::organization::{
        CreateOrganizationDto, OrganizationResponseDto, PatchOrganizationDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::organization::OrganizationService,
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct OrganizationController {
    service: Arc<OrganizationService>,
}

#[controller("/organizations")]
impl OrganizationController {
    fn new(service: Arc<OrganizationService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    #[live(tables = ["organization","organization_members"])]
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<OrganizationResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(OrganizationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get]
    #[live(tables = ["organization","organization_members"])]
    async fn list_by_owner(
        &self,
        claims: Claims,
    ) -> Result<Json<Vec<OrganizationResponseDto>>, ApiError> {
        self.service
            .list_by_owner(claims.user.user_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(OrganizationResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        claims: Claims,
        ValidatedJson(body): ValidatedJson<CreateOrganizationDto>,
    ) -> Result<(StatusCode, Json<OrganizationResponseDto>), ApiError> {
        self.service
            .create(claims.user.user_id, body)
            .await
            .map(OrganizationResponseDto::from)
            .map(|organization| (StatusCode::CREATED, Json(organization)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Organization, CanWrite>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchOrganizationDto>,
    ) -> Result<Json<OrganizationResponseDto>, ApiError> {
        self.service
            .update(id, body)
            .await
            .map(OrganizationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Organization, CanWrite>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "organization not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "owner not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "organization database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
