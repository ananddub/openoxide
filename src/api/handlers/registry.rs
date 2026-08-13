use crate::core::middleware::permission::{Application, CanCreate, CanDelete, CanRead};
use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::dto::registry::{
        CreateRegistryDto, PatchRegistryDto, RegistryRepositoriesDto, RegistryResponseDto,
        RegistryTagsDto, RegistryUsageDto, RotateRegistryCredentialsDto, TestRegistryDto,
    },
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::registry::RegistryService,
};

type ApiError = (StatusCode, String);

#[derive(Debug, Serialize, Deserialize, poem_openapi::Object)]
struct RegistryTagsQuery {
    repository: String,
}

pub struct RegistryController {
    service: Arc<RegistryService>,
    cache: Arc<AppStateCache>,
}

#[controller("/registries")]
impl RegistryController {
    fn new(service: Arc<RegistryService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
    ) -> Result<Json<Vec<RegistryResponseDto>>, ApiError> {
        if let Some(CacheEnum::RegistriesList(cached)) =
            self.cache.get(&CacheKey::RegistriesList).await
        {
            return Ok(Json(
                cached.into_iter().map(RegistryResponseDto::from).collect(),
            ));
        }

        let items = self.service.list().await.map_err(map_sqlx_error)?;
        self.cache
            .insert(
                CacheKey::RegistriesList,
                CacheEnum::RegistriesList(items.clone()),
            )
            .await;

        Ok(Json(
            items.into_iter().map(RegistryResponseDto::from).collect(),
        ))
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<RegistryResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(RegistryResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        ValidatedJson(body): ValidatedJson<CreateRegistryDto>,
    ) -> Result<(StatusCode, Json<RegistryResponseDto>), ApiError> {
        let created = self
            .service
            .create(body)
            .await
            .map(RegistryResponseDto::from)
            .map(|registry| (StatusCode::CREATED, Json(registry)))
            .map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::RegistriesList).await;
        Ok(created)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchRegistryDto>,
    ) -> Result<Json<RegistryResponseDto>, ApiError> {
        let updated = self
            .service
            .patch(id, body)
            .await
            .map(RegistryResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::RegistriesList).await;
        Ok(updated)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDelete>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service.delete(id).await.map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::RegistriesList).await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/test")]
    async fn test_registry(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        ValidatedJson(body): ValidatedJson<TestRegistryDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection_raw(&body.registry_url, &body.username, &body.password)
            .await
            .map(|_| StatusCode::OK)
            .map_err(|e| (StatusCode::BAD_REQUEST, e))
    }

    #[post("/{id}/test")]
    async fn test_saved_registry(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection(id)
            .await
            .map(|_| StatusCode::OK)
            .map_err(|error| (StatusCode::BAD_REQUEST, error))
    }

    #[get("/{id}/repositories")]
    async fn repositories(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<RegistryRepositoriesDto>, ApiError> {
        self.service
            .repositories(id)
            .await
            .map(|repositories| Json(RegistryRepositoriesDto { repositories }))
            .map_err(|error| (StatusCode::BAD_GATEWAY, error))
    }

    #[get("/{id}/tags")]
    async fn tags(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
        Query(query): Query<RegistryTagsQuery>,
    ) -> Result<Json<RegistryTagsDto>, ApiError> {
        let repository = query.repository;
        self.service
            .tags(id, &repository)
            .await
            .map(|tags| Json(RegistryTagsDto { repository, tags }))
            .map_err(|error| (StatusCode::BAD_GATEWAY, error))
    }

    #[post("/{id}/rotate-credentials")]
    async fn rotate_credentials(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(id): Path<i64>,
        Json(body): Json<RotateRegistryCredentialsDto>,
    ) -> Result<Json<RegistryResponseDto>, ApiError> {
        self.service
            .rotate_credentials(id, body.username, body.password)
            .await
            .map(RegistryResponseDto::from)
            .map(Json)
            .map_err(|error| (StatusCode::BAD_REQUEST, error))
    }

    #[get("/{id}/usage")]
    async fn usage(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<RegistryUsageDto>>, ApiError> {
        self.service
            .usage(id)
            .await
            .map(|rows| Json(rows.into_iter().map(Into::into).collect()))
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "registry not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::CONFLICT, message),
        other => {
            tracing::error!(error = %other, "registry database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
