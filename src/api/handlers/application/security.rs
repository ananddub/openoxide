use crate::core::middleware::permission::{Application, CanCreate, CanDelete, CanRead};
use crate::{
    api::dto::application::security::{
        ApplicationSecurityResponseDto, UpsertApplicationSecurityDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::application::security::SecurityService,
};
use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};
use std::sync::Arc;
type ApiError = (StatusCode, String);
pub struct SecurityController {
    service: Arc<SecurityService>,
}
#[controller("/applications/{application_id}/security")]
impl SecurityController {
    fn new(service: Arc<SecurityService>) -> Self {
        Self { service }
    }
    #[get]
    #[live(table = "security")]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ApplicationSecurityResponseDto>>, ApiError> {
        self.service
            .list(application_id)
            .await
            .map(|x| Json(x.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }
    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(application_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpsertApplicationSecurityDto>,
    ) -> Result<(StatusCode, Json<ApplicationSecurityResponseDto>), ApiError> {
        self.service
            .create(application_id, body)
            .await
            .map(|x| (StatusCode::CREATED, Json(x.into())))
            .map_err(map_error)
    }
    #[put("/{id}")]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path((application_id, id)): Path<(i64, i64)>,
        ValidatedJson(body): ValidatedJson<UpsertApplicationSecurityDto>,
    ) -> Result<Json<ApplicationSecurityResponseDto>, ApiError> {
        self.service
            .update(application_id, id, body)
            .await
            .map(|x| Json(x.into()))
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
            Ok(false) => Err((StatusCode::NOT_FOUND, "security entry not found".into())),
            Err(e) => Err(map_error(e)),
        }
    }
}
fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (
            StatusCode::NOT_FOUND,
            "application or security entry not found".into(),
        ),
        sqlx::Error::Database(ref e) if e.is_unique_violation() => (
            StatusCode::CONFLICT,
            "username already exists for application".into(),
        ),
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error=%other,"application security operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
