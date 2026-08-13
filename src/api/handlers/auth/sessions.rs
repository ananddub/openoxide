use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::auth::AuthSessionDto, services::auth::AuthService, utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct AuthSessionController {
    service: Arc<AuthService>,
}

#[controller("/auth/sessions")]
impl AuthSessionController {
    fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(table = "jwt_tokens")]
    async fn list(&self, claims: Claims) -> Result<Json<Vec<AuthSessionDto>>, ApiError> {
        self.service
            .list_sessions(claims.user.user_id, &claims.jti)
            .await
            .map(Json)
            .map_err(internal)
    }

    #[delete("/{session_id}")]
    async fn revoke(
        &self,
        claims: Claims,
        Path(session_id): Path<String>,
    ) -> Result<StatusCode, ApiError> {
        if self
            .service
            .revoke_session(claims.user.user_id, &session_id)
            .await
            .map_err(internal)?
        {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((StatusCode::NOT_FOUND, "session not found".into()))
        }
    }
}

fn internal(error: impl std::fmt::Display) -> ApiError {
    tracing::error!(error = %error, "authentication session operation failed");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "session operation failed".into(),
    )
}
