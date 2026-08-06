use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::remote_server::{
        CreateRemoteServerDto, PatchRemoteServerDto, RemoteServerActionResponseDto,
        RemoteServerResponseDto,
    },
    core::middleware::validator::ValidatedJson,
    services::remote_server::ServerService,
    utils::jwt::claim::Claims,
};

type ApiError = (StatusCode, String);

pub struct RemoteServerController {
    service: Arc<ServerService>,
}

#[controller("/remote-servers")]
impl RemoteServerController {
    fn new(service: Arc<ServerService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(&self, _claims: Claims) -> Result<Json<Vec<RemoteServerResponseDto>>, ApiError> {
        self.service
            .list()
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(RemoteServerResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<RemoteServerResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(RemoteServerResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        _claims: Claims,
        ValidatedJson(body): ValidatedJson<CreateRemoteServerDto>,
    ) -> Result<(StatusCode, Json<RemoteServerResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(RemoteServerResponseDto::from)
            .map(|server| (StatusCode::CREATED, Json(server)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchRemoteServerDto>,
    ) -> Result<Json<RemoteServerResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(RemoteServerResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/activate")]
    async fn activate(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<RemoteServerActionResponseDto>, ApiError> {
        self.service
            .set_status(id, "ACTIVE")
            .await
            .map(|server| RemoteServerActionResponseDto {
                server: RemoteServerResponseDto::from(server),
                action: "activate".into(),
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/deactivate")]
    async fn deactivate(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<RemoteServerActionResponseDto>, ApiError> {
        self.service
            .set_status(id, "INACTIVE")
            .await
            .map(|server| RemoteServerActionResponseDto {
                server: RemoteServerResponseDto::from(server),
                action: "deactivate".into(),
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/test-connection")]
    async fn test_connection(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<RemoteServerActionResponseDto>, ApiError> {
        self.service
            .touch_test_connection(id)
            .await
            .map(|server| RemoteServerActionResponseDto {
                server: RemoteServerResponseDto::from(server),
                action: "test-connection".into(),
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(&self, _claims: Claims, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "remote server not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "ssh key not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_check_violation() => {
            (StatusCode::BAD_REQUEST, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::CONFLICT, message),
        other => {
            tracing::error!(error = %other, "remote server database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
