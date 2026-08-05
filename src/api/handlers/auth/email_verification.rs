use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};

use crate::{
    api::dto::auth::{EmailVerificationConfirmDto, EmailVerificationStatusDto},
    core::middleware::validator::ValidatedJson,
    services::auth::{AuthError, AuthService},
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct AuthEmailVerificationController {
    service: Arc<AuthService>,
}

#[controller("/auth/email-verification")]
impl AuthEmailVerificationController {
    fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[get("/status")]
    async fn status(&self, claims: Claims) -> Result<Json<EmailVerificationStatusDto>, ApiError> {
        self.service
            .email_verification_status(claims.user.user_id)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[post("/request")]
    async fn request(&self, claims: Claims) -> Result<StatusCode, ApiError> {
        self.service
            .request_email_verification(claims.user.user_id)
            .await
            .map(|()| StatusCode::ACCEPTED)
            .map_err(internal)
    }

    #[post("/confirm")]
    async fn confirm(
        &self,
        ValidatedJson(body): ValidatedJson<EmailVerificationConfirmDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .verify_email(&body.token)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(|error| match error {
                AuthError::InvalidVerificationToken => (StatusCode::BAD_REQUEST, error.to_string()),
                other => internal(other),
            })
    }
}

fn internal(error: impl std::fmt::Display) -> ApiError {
    tracing::error!(error = %error, "email verification operation failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "email verification operation failed".into(),
    )
}
