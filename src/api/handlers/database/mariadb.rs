use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use super::ApiError;
use crate::{
    api::dto::database::{
        CreateDatabaseDto, DatabaseOperationResponseDto, DatabaseResponseDto, PatchDatabaseDto,
    },
    core::middleware::{
        permission::{
            AppDeployPermission, DatabaseCreatePermission, DatabaseDeletePermission,
            DatabaseReadPermission, DatabaseUpdatePermission, RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::database::{DatabaseKind, DatabaseOperation, DatabaseService},
};

pub struct MariadbController {
    service: Arc<DatabaseService>,
}

#[controller("/mariadb")]
impl MariadbController {
    fn new(service: Arc<DatabaseService>) -> Self {
        Self { service }
    }

    #[get("/environment/{environment_id}")]
    async fn list_by_environment(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(environment_id): Path<i64>,
    ) -> Result<Json<Vec<DatabaseResponseDto>>, ApiError> {
        self.service
            .list_by_environment(environment_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .filter(|item| item.kind == DatabaseKind::Mariadb)
                    .map(DatabaseResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        self.service
            .get_by_id(DatabaseKind::Mariadb, id)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateDatabaseDto>,
    ) -> Result<(StatusCode, Json<DatabaseResponseDto>), ApiError> {
        self.service
            .create(DatabaseKind::Mariadb, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(|database| (StatusCode::CREATED, Json(database)))
            .map_err(super::map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseUpdatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDatabaseDto>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        self.service
            .patch(DatabaseKind::Mariadb, id, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post("/{id}/deploy")]
    async fn deploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mariadb,
            id,
            DatabaseOperation::Deploy,
        )
        .await
    }

    #[post("/{id}/redeploy")]
    async fn redeploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mariadb,
            id,
            DatabaseOperation::Redeploy,
        )
        .await
    }

    #[post("/{id}/reload")]
    async fn reload(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mariadb,
            id,
            DatabaseOperation::Reload,
        )
        .await
    }

    #[post("/{id}/start")]
    async fn start(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mariadb,
            id,
            DatabaseOperation::Start,
        )
        .await
    }

    #[post("/{id}/stop")]
    async fn stop(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mariadb,
            id,
            DatabaseOperation::Stop,
        )
        .await
    }

    #[post("/{id}/cancel")]
    async fn cancel(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.cancel_operation(DatabaseKind::Mariadb, id).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(error) => Err(super::map_sqlx_error(error)),
        }
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(DatabaseKind::Mariadb, id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(super::map_sqlx_error)
    }
}
