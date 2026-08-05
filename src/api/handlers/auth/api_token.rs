use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::auth::{
        CreatePersonalAccessTokenDto, CreatedPersonalAccessTokenDto, PersonalAccessTokenDto,
    },
    core::middleware::validator::ValidatedJson,
    services::auth::{AuthError, AuthService},
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct AuthApiTokenController {
    service: Arc<AuthService>,
}

#[controller("/auth/api-tokens")]
impl AuthApiTokenController {
    fn new(service: Arc<AuthService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(&self, claims: Claims) -> Result<Json<Vec<PersonalAccessTokenDto>>, ApiError> {
        self.service
            .list_personal_access_tokens(claims.user.user_id)
            .await
            .map(Json)
            .map_err(map_error)
    }

    #[post]
    async fn create(
        &self,
        claims: Claims,
        ValidatedJson(body): ValidatedJson<CreatePersonalAccessTokenDto>,
    ) -> Result<(StatusCode, Json<CreatedPersonalAccessTokenDto>), ApiError> {
        self.service
            .create_personal_access_token(
                claims.user.user_id,
                body.name,
                body.password,
                body.expires_at,
            )
            .await
            .map(|token| (StatusCode::CREATED, Json(token)))
            .map_err(map_error)
    }

    #[delete("/{id}")]
    async fn revoke(&self, claims: Claims, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        if self
            .service
            .revoke_personal_access_token(claims.user.user_id, id)
            .await
            .map_err(map_error)?
        {
            Ok(StatusCode::NO_CONTENT)
        } else {
            Err((
                StatusCode::NOT_FOUND,
                "personal access token not found".into(),
            ))
        }
    }
}

fn map_error(error: AuthError) -> ApiError {
    match error {
        AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "invalid password".into()),
        AuthError::InvalidOperation(message) => (StatusCode::BAD_REQUEST, message),
        other => {
            tracing::error!(error = %other, "personal access token operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "API token operation failed".into(),
            )
        }
    }
}
