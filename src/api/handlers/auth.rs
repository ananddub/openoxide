use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

use crate::{
    api::dto::auth::{AuthResponseDto, LoginDto, RefreshTokenDto, SignupDto},
    core::middleware::validator::ValidatedJson,
    services::auth::{AuthError, AuthService},
    utils::jwt::claim::{Claims, JwtSubject},
};

type ApiError = (StatusCode, Json<Value>);

pub struct AuthController {
    service: Arc<AuthService>,
}

#[controller("/auth")]
impl AuthController {
    fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[post("/signup")]
    async fn signup(
        &self,
        ValidatedJson(body): ValidatedJson<SignupDto>,
    ) -> Result<(StatusCode, Json<AuthResponseDto>), ApiError> {
        self.service
            .signup(body)
            .await
            .map(|response| (StatusCode::CREATED, Json(response)))
            .map_err(map_auth_error)
    }

    #[post("/login")]
    async fn login(
        &self,
        ValidatedJson(body): ValidatedJson<LoginDto>,
    ) -> Result<Json<AuthResponseDto>, ApiError> {
        self.service
            .login(body)
            .await
            .map(Json)
            .map_err(map_auth_error)
    }

    #[post("/refresh")]
    async fn refresh(
        &self,
        ValidatedJson(body): ValidatedJson<RefreshTokenDto>,
    ) -> Result<Json<AuthResponseDto>, ApiError> {
        self.service
            .refresh(&body.refresh_token)
            .await
            .map(Json)
            .map_err(map_auth_error)
    }

    #[get("/whoami")]
    async fn who_am_i(&self, claims: Claims) -> Json<JwtSubject> {
        Json(claims.user)
    }

    #[post("/logout")]
    async fn logout(&self, claims: Claims) -> Result<StatusCode, ApiError> {
        self.service
            .logout_all(claims.user.user_id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_auth_error)
    }

    #[get("/setup")]
    async fn setup(&self) -> Result<Json<Value>, ApiError> {
        self.service
            .is_owner_present()
            .await
            .map(|present| Json(json!({ "isOwnerPresent": present })))
            .map_err(map_auth_error)
    }
}

fn map_auth_error(error: AuthError) -> ApiError {
    let (status, message) = match error {
        AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid email or password"),
        AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid or revoked token"),
        AuthError::Database(sqlx::Error::Database(ref database_error))
            if database_error.is_unique_violation() =>
        {
            (StatusCode::CONFLICT, "email is already registered")
        }
        AuthError::Database(database_error) => {
            tracing::error!(error = %database_error, "authentication database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "authentication operation failed",
            )
        }
        AuthError::Internal => {
            tracing::error!("internal authentication operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "authentication operation failed",
            )
        }
    };
    (status, Json(json!({ "error": message })))
}
