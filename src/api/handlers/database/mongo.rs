use crate::core::middleware::permission::{
    Application, CanCreate, CanDelete, CanDeploy, CanRead, CanUpdate, Database,
};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use super::ApiError;
use crate::{
    api::dto::database::{
        CreateDatabaseDto, DatabaseOperationResponseDto, DatabaseResponseDto, PatchDatabaseDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::database::{DatabaseKind, DatabaseOperation, DatabaseService},
};

pub struct MongoController {
    service: Arc<DatabaseService>,
}

#[controller("/mongo")]
impl MongoController {
    fn new(service: Arc<DatabaseService>) -> Self {
        Self { service }
    }

    #[get("/environment/{environment_id}")]
    #[live(table = "mongo_dbs")]
    async fn list_by_environment(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path(environment_id): Path<i64>,
    ) -> Result<Json<Vec<DatabaseResponseDto>>, ApiError> {
        self.service
            .list_by_environment(environment_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .filter(|item| item.kind == DatabaseKind::Mongo)
                    .map(DatabaseResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[get("/{id}")]
    #[live(table = "mongo_dbs")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        self.service
            .get_by_id(DatabaseKind::Mongo, id)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanCreate>,
        ValidatedJson(body): ValidatedJson<CreateDatabaseDto>,
    ) -> Result<(StatusCode, Json<DatabaseResponseDto>), ApiError> {
        self.service
            .create(DatabaseKind::Mongo, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(|database| (StatusCode::CREATED, Json(database)))
            .map_err(super::map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanUpdate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDatabaseDto>,
    ) -> Result<Json<DatabaseResponseDto>, ApiError> {
        self.service
            .patch(DatabaseKind::Mongo, id, body)
            .await
            .map(DatabaseResponseDto::from)
            .map(Json)
            .map_err(super::map_sqlx_error)
    }

    #[post("/{id}/deploy")]
    async fn deploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mongo,
            id,
            DatabaseOperation::Deploy,
        )
        .await
    }

    #[post("/{id}/redeploy")]
    async fn redeploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mongo,
            id,
            DatabaseOperation::Redeploy,
        )
        .await
    }

    #[post("/{id}/reload")]
    async fn reload(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mongo,
            id,
            DatabaseOperation::Reload,
        )
        .await
    }

    #[post("/{id}/start")]
    async fn start(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mongo,
            id,
            DatabaseOperation::Start,
        )
        .await
    }

    #[post("/{id}/stop")]
    async fn stop(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<DatabaseOperationResponseDto>), ApiError> {
        super::run_operation(
            &self.service,
            DatabaseKind::Mongo,
            id,
            DatabaseOperation::Stop,
        )
        .await
    }

    #[post("/{id}/cancel")]
    async fn cancel(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.cancel_operation(DatabaseKind::Mongo, id).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(error) => Err(super::map_sqlx_error(error)),
        }
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanDelete>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(DatabaseKind::Mongo, id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(super::map_sqlx_error)
    }
}
