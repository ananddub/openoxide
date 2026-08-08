use crate::core::middleware::permission::{CanCreate, CanDelete, CanRead, Database};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::destination::{
        CreateDestinationDto, DestinationResponseDto, PatchDestinationDto, TestDestinationDto,
    },
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::destination::DestinationService,
};

type ApiError = (StatusCode, String);

pub struct DestinationController {
    service: Arc<DestinationService>,
    cache: Arc<AppStateCache>,
}

#[controller("/destinations")]
impl DestinationController {
    fn new(service: Arc<DestinationService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
    ) -> Result<Json<Vec<DestinationResponseDto>>, ApiError> {
        if let Some(CacheEnum::DestinationsList(cached)) =
            self.cache.get(&CacheKey::DestinationsList).await
        {
            return Ok(Json(
                cached
                    .into_iter()
                    .map(DestinationResponseDto::from)
                    .collect(),
            ));
        }

        let items = self.service.list().await.map_err(map_sqlx_error)?;
        self.cache
            .insert(
                CacheKey::DestinationsList,
                CacheEnum::DestinationsList(items.clone()),
            )
            .await;

        Ok(Json(
            items
                .into_iter()
                .map(DestinationResponseDto::from)
                .collect(),
        ))
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanRead>,
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
        RequirePermission(_claims, _): RequirePermission<Database, CanCreate>,
        ValidatedJson(body): ValidatedJson<CreateDestinationDto>,
    ) -> Result<(StatusCode, Json<DestinationResponseDto>), ApiError> {
        let created = self
            .service
            .create(body)
            .await
            .map(DestinationResponseDto::from)
            .map(|dest| (StatusCode::CREATED, Json(dest)))
            .map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::DestinationsList).await;
        Ok(created)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanCreate>,
        Path(id): Path<String>,
        ValidatedJson(body): ValidatedJson<PatchDestinationDto>,
    ) -> Result<Json<DestinationResponseDto>, ApiError> {
        let updated = self
            .service
            .patch(&id, body)
            .await
            .map(DestinationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::DestinationsList).await;
        Ok(updated)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanDelete>,
        Path(id): Path<String>,
    ) -> Result<StatusCode, ApiError> {
        self.service.delete(&id).await.map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::DestinationsList).await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/test")]
    async fn test_destination(
        &self,
        RequirePermission(_claims, _): RequirePermission<Database, CanCreate>,
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
            )
            .await
            .map(|_| StatusCode::OK)
            .map_err(|e| (StatusCode::BAD_REQUEST, e))
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
