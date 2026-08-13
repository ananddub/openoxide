use crate::core::middleware::permission::{Application, CanCreate, CanDelete, CanRead};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::application::port::{ApplicationPortResponseDto, UpsertApplicationPortDto},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::application::port::PortService,
};

type ApiError = (StatusCode, String);

pub struct PortController {
    service: Arc<PortService>,
}

#[controller("/applications/{application_id}/ports")]
impl PortController {
    fn new(service: Arc<PortService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(table = "ports")]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ApplicationPortResponseDto>>, ApiError> {
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
        ValidatedJson(body): ValidatedJson<UpsertApplicationPortDto>,
    ) -> Result<(StatusCode, Json<ApplicationPortResponseDto>), ApiError> {
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
        ValidatedJson(body): ValidatedJson<UpsertApplicationPortDto>,
    ) -> Result<Json<ApplicationPortResponseDto>, ApiError> {
        self.service
            .update(application_id, id, body)
            .await
            .map(Into::into)
            .map(Json)
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
            Ok(false) => Err((StatusCode::NOT_FOUND, "port not found".into())),
            Err(error) => Err(map_error(error)),
        }
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            "application or port not found".into(),
        ),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        sqlx::Error::Database(ref error) if error.is_unique_violation() => {
            (StatusCode::CONFLICT, error.message().into())
        }
        other => {
            tracing::error!(error = %other, "application port operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
