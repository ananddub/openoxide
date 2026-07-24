use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::destination::{
        CreateDestinationDto, DestinationResponseDto, PatchDestinationDto, TestDestinationDto,
    },
    core::middleware::{
        permission::{
            DatabaseCreatePermission, DatabaseDeletePermission, DatabaseReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::destination::DestinationService,
};

type ApiError = (StatusCode, String);

pub struct DestinationController {
    service: Arc<DestinationService>,
}

#[controller("/destinations")]
impl DestinationController {
    fn new(service: Arc<DestinationService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
    ) -> Result<Json<Vec<DestinationResponseDto>>, ApiError> {
        self.service
            .list()
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(DestinationResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<String>,
    ) -> Result<Json<DestinationResponseDto>, ApiError> {
        self.service
            .get_by_id(&id)
            .await
            .map(DestinationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateDestinationDto>,
    ) -> Result<(StatusCode, Json<DestinationResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(DestinationResponseDto::from)
            .map(|dest| (StatusCode::CREATED, Json(dest)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        Path(id): Path<String>,
        ValidatedJson(body): ValidatedJson<PatchDestinationDto>,
    ) -> Result<Json<DestinationResponseDto>, ApiError> {
        self.service
            .patch(&id, body)
            .await
            .map(DestinationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseDeletePermission>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(&id)
            .await
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/test")]
    async fn test_connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseReadPermission>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection(&id)
            .await
            .map(|()| StatusCode::OK)
            .map_err(|err| (StatusCode::BAD_REQUEST, err))
    }

    #[post("/test-raw")]
    async fn test_connection_raw(
        &self,
        RequirePermission(_claims, _): RequirePermission<DatabaseCreatePermission>,
        ValidatedJson(body): ValidatedJson<TestDestinationDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection_raw(
                &body.provider,
                &body.access_key,
                &body.secret_access_key,
                &body.bucket,
                &body.region,
                &body.endpoint,
                body.additional_flags.as_deref(),
            )
            .await
            .map(|()| StatusCode::OK)
            .map_err(|err| (StatusCode::BAD_REQUEST, err))
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "destination not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "destination database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
