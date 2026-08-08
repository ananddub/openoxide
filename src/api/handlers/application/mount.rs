use crate::core::middleware::permission::{Application, CanCreate, CanDelete, CanRead};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::application::mount::{ApplicationMountResponseDto, UpsertApplicationMountDto},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::application::mount::MountService,
};

type ApiError = (StatusCode, String);

pub struct MountController {
    service: Arc<MountService>,
}

#[controller("/applications/{application_id}/mounts")]
impl MountController {
    fn new(service: Arc<MountService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ApplicationMountResponseDto>>, ApiError> {
        self.service
            .list(application_id)
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(application_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpsertApplicationMountDto>,
    ) -> Result<(StatusCode, Json<ApplicationMountResponseDto>), ApiError> {
        self.service
            .create(application_id, body)
            .await
            .map(|item| (StatusCode::CREATED, Json(item.into())))
            .map_err(map_error)
    }

    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path((application_id, id)): Path<(i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertApplicationMountDto>,
    ) -> Result<Json<ApplicationMountResponseDto>, ApiError> {
        self.service
            .update(application_id, id, body)
            .await
            .map(|item| Json(item.into()))
            .map_err(map_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDelete>,
        Path((application_id, id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.delete(application_id, id).await {
            Ok(true) => Ok(StatusCode::NO_CONTENT),
            Ok(false) => Err((StatusCode::NOT_FOUND, "mount not found".into())),
            Err(error) => Err(map_error(error)),
        }
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            "application or mount not found".into(),
        ),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "application mount operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
