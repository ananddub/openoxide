use crate::{
    api::dto::application::middleware::{
        ApplicationMiddlewareResponseDto, UpsertApplicationMiddlewareDto,
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppReadPermission, RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::application::middleware::ApplicationMiddlewareService,
};
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use std::sync::Arc;
type ApiError = (StatusCode, String);
pub struct ApplicationMiddlewareController {
    service: Arc<ApplicationMiddlewareService>,
}
#[controller("/applications/{application_id}/middlewares")]
impl ApplicationMiddlewareController {
    fn new(service: Arc<ApplicationMiddlewareService>) -> Self {
        Self { service }
    }
    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ApplicationMiddlewareResponseDto>>, ApiError> {
        self.service
            .list(application_id)
            .await
            .map(Json)
            .map_err(map_error)
    }
    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(application_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpsertApplicationMiddlewareDto>,
    ) -> Result<(StatusCode, Json<ApplicationMiddlewareResponseDto>), ApiError> {
        self.service
            .create(application_id, body)
            .await
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(map_error)
    }
    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path((application_id, id)): Path<(i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertApplicationMiddlewareDto>,
    ) -> Result<Json<ApplicationMiddlewareResponseDto>, ApiError> {
        self.service
            .update(application_id, id, body)
            .await
            .map(Json)
            .map_err(map_error)
    }
    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path((application_id, id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.delete(application_id, id).await {
            Ok(true) => Ok(StatusCode::NO_CONTENT),
            Ok(false) => Err((StatusCode::NOT_FOUND, "middleware not found".into())),
            Err(error) => Err(map_error(error)),
        }
    }
}
fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            "application or middleware not found".into(),
        ),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        sqlx::Error::Database(ref error) if error.is_unique_violation() => (
            StatusCode::CONFLICT,
            "middleware name already exists".into(),
        ),
        other => {
            tracing::error!(error=%other,"application middleware operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
