use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

use crate::{
    api::dto::auth::{
        RecoveryCodesResponseDto, TwoFactorCodeDto, TwoFactorDisableDto, TwoFactorSetupDto,
        TwoFactorSetupResponseDto, TwoFactorStatusDto,
    },
    core::middleware::validator::ValidatedJson,
    services::auth::{AuthError, AuthService},
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, Json<Value>);

pub struct AuthTwoFactorController {
    service: Arc<AuthService>,
}

#[controller("/auth/2fa")]
impl AuthTwoFactorController {
    fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[get("/status")]
    async fn status(&self, claims: Claims) -> Result<Json<TwoFactorStatusDto>, ApiError> {
        self.service
            .two_factor_status(claims.user.user_id)
            .await
            .map(Json)
            .map_err(map_auth_error)
    }

    #[post("/setup")]
    async fn setup(
        &self,
        claims: Claims,
        ValidatedJson(body): ValidatedJson<TwoFactorSetupDto>,
    ) -> Result<Json<TwoFactorSetupResponseDto>, ApiError> {
        self.service
            .setup_two_factor(claims.user.user_id, body.password)
            .await
            .map(Json)
            .map_err(map_auth_error)
    }

    #[post("/enable")]
    async fn enable(
        &self,
        claims: Claims,
        ValidatedJson(body): ValidatedJson<TwoFactorCodeDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .enable_two_factor(claims.user.user_id, &body.code)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_auth_error)
    }

    #[post("/disable")]
    async fn disable(
        &self,
        claims: Claims,
        ValidatedJson(body): ValidatedJson<TwoFactorDisableDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .disable_two_factor(claims.user.user_id, body.password, &body.code)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_auth_error)
    }

    #[post("/recovery-codes/regenerate")]
    async fn regenerate_recovery_codes(
        &self,
        claims: Claims,
        ValidatedJson(body): ValidatedJson<TwoFactorDisableDto>,
    ) -> Result<Json<RecoveryCodesResponseDto>, ApiError> {
        self.service
            .regenerate_recovery_codes(claims.user.user_id, body.password, &body.code)
            .await
            .map(Json)
            .map_err(map_auth_error)
    }
}

fn map_auth_error(error: AuthError) -> ApiError {
    let (status, message) = match error {
        AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid password".into()),
        AuthError::InvalidSecondFactor => (
            StatusCode::UNAUTHORIZED,
            "invalid two-factor authentication code".into(),
        ),
        AuthError::InvalidOperation(message) => (StatusCode::CONFLICT, message),
        AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid or revoked token".into()),
        AuthError::TwoFactorRequired => (
            StatusCode::UNAUTHORIZED,
            "two-factor authentication code is required".into(),
        ),
        AuthError::InvalidResetToken => (
            StatusCode::BAD_REQUEST,
            "invalid or expired password reset token".into(),
        ),
        AuthError::InvalidVerificationToken => (
            StatusCode::BAD_REQUEST,
            "invalid or expired email verification token".into(),
        ),
        AuthError::Database(database_error) => {
            tracing::error!(error = %database_error, "two-factor database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "two-factor operation failed".into(),
            )
        }
        AuthError::Internal => {
            tracing::error!("internal two-factor operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "two-factor operation failed".into(),
            )
        }
    };
    (status, Json(json!({ "error": message })))
}
