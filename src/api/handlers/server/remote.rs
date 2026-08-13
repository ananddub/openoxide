use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use super::error::{ApiError, map_sqlx_error};
use crate::{
    api::dto::server::{
        CreateRemoteServerDto, MigrateServerDependenciesDto, PatchRemoteServerDto,
        RemoteServerAction, RemoteServerActionResponseDto, RemoteServerResponseDto,
        ServerDependencyMigrationDto,
    },
    core::middleware::validator::ValidatedJson,
    services::server::{RemoteServerService, RemoteServerStatus},
    utils::jwt::claim::Claims,
};

pub struct RemoteServerController {
    service: Arc<RemoteServerService>,
}

#[controller("/remote-servers")]
impl RemoteServerController {
    fn new(service: Arc<RemoteServerService>) -> Self {
        Self { service }
    }

    #[get]
    #[live(tables = ["servers","server_migrations"])]
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
    #[live(tables = ["servers","server_migrations"])]
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
        self.action(id, RemoteServerStatus::Active, RemoteServerAction::Activate)
            .await
    }

    #[post("/{id}/deactivate")]
    async fn deactivate(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<RemoteServerActionResponseDto>, ApiError> {
        self.action(
            id,
            RemoteServerStatus::Inactive,
            RemoteServerAction::Deactivate,
        )
        .await
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
                action: RemoteServerAction::TestConnection,
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

    #[post("/{id}/dependencies/migrate")]
    async fn migrate_dependencies(
        &self,
        _claims: Claims,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<MigrateServerDependenciesDto>,
    ) -> Result<Json<ServerDependencyMigrationDto>, ApiError> {
        self.service
            .migrate_dependencies(id, body.target_server_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/migrations/{migration_id}")]
    #[live(tables = ["servers","server_migrations"])]
    async fn migration_status(
        &self,
        _claims: Claims,
        Path(migration_id): Path<String>,
    ) -> Result<Json<ServerDependencyMigrationDto>, ApiError> {
        self.service
            .migration_status(&migration_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/migrations/{migration_id}/rollback")]
    async fn rollback_migration(
        &self,
        _claims: Claims,
        Path(migration_id): Path<String>,
    ) -> Result<Json<ServerDependencyMigrationDto>, ApiError> {
        self.service
            .rollback_migration(&migration_id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    async fn action(
        &self,
        id: i64,
        status: RemoteServerStatus,
        action: RemoteServerAction,
    ) -> Result<Json<RemoteServerActionResponseDto>, ApiError> {
        self.service
            .set_status(id, status)
            .await
            .map(|server| RemoteServerActionResponseDto {
                server: RemoteServerResponseDto::from(server),
                action,
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }
}
