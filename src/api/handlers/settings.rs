use crate::core::middleware::permission::{CanWrite, Organization};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};

use crate::{
    api::dto::settings::{SettingsResponseDto, UpdateSettingsDto},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::settings::SettingsService,
};

type ApiError = (StatusCode, String);

pub struct SettingsController {
    service: Arc<SettingsService>,
}

#[controller("/settings")]
impl SettingsController {
    fn new(service: Arc<SettingsService>) -> Self {
        Self { service }
    }

    #[get]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Organization, CanWrite>,
    ) -> Result<Json<SettingsResponseDto>, ApiError> {
        self.service.get().await.map(Json).map_err(map_error)
    }

    #[put]
    async fn update(
        &self,
        RequirePermission(_claims, _): RequirePermission<Organization, CanWrite>,
        ValidatedJson(body): ValidatedJson<UpdateSettingsDto>,
    ) -> Result<Json<SettingsResponseDto>, ApiError> {
        self.service.update(body).await.map(Json).map_err(map_error)
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "settings operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "settings operation failed".into(),
            )
        }
    }
}
