use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::database_network::{
        CreateDatabaseNetworkDto, DatabaseNetworkResponseDto, PatchDatabaseNetworkDto,
    },
    core::middleware::{
        permission::{
            DatabaseCreatePermission, DatabaseDeletePermission, DatabaseReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::database_network::DatabaseNetworkService,
};

type ApiError = (StatusCode, String);

pub struct DatabaseNetworkController {
    service: Arc<DatabaseNetworkService>,
}

#[controller("/database-networks")]
impl DatabaseNetworkController {
    fn new(service: Arc<DatabaseNetworkService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
    ) -> Result<Json<Vec<DatabaseNetworkResponseDto>>, ApiError> {
        self.service
            .list()
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(DatabaseNetworkResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/server/{server_id}")]
    async fn list_by_server(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<Vec<DatabaseNetworkResponseDto>>, ApiError> {
        self.service
            .list_by_server(Some(server_id))
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(DatabaseNetworkResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<DatabaseNetworkResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(DatabaseNetworkResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateDatabaseNetworkDto>,
    ) -> Result<(StatusCode, Json<DatabaseNetworkResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(DatabaseNetworkResponseDto::from)
            .map(|network| (StatusCode::CREATED, Json(network)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDatabaseNetworkDto>,
    ) -> Result<Json<DatabaseNetworkResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(DatabaseNetworkResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/dependencies")]
    async fn dependencies(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<crate::repository::NetworkDependencyCounts>, ApiError> {
        self.service
            .dependencies(id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "database network not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::CONFLICT, message),
        other => {
            tracing::error!(error = %other, "database network operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
