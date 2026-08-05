use std::sync::Arc;

use auto_route::controller;
use axum::http::StatusCode;

use crate::{
    api::dto::auth::{PasswordResetConfirmDto, PasswordResetRequestDto},
    core::middleware::validator::ValidatedJson,
    services::auth::{AuthError, AuthService},
};

type ApiError = (StatusCode, String);

pub struct AuthPasswordController {
    service: Arc<AuthService>,
}

#[controller("/auth/password")]
impl AuthPasswordController {
    fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[post("/forgot")]
    async fn forgot(
        &self,
        ValidatedJson(body): ValidatedJson<PasswordResetRequestDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .request_password_reset(&body.email)
            .await
            .map(|()| StatusCode::ACCEPTED)
            .map_err(map_error)
    }

    #[post("/reset")]
    async fn reset(
        &self,
        ValidatedJson(body): ValidatedJson<PasswordResetConfirmDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .reset_password(&body.token, body.password)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }
}

fn map_error(error: AuthError) -> ApiError {
    match error {
        AuthError::InvalidResetToken => (StatusCode::BAD_REQUEST, error.to_string()),
        other => {
            tracing::error!(error = %other, "password reset operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "password reset operation failed".into(),
            )
        }
    }
}
