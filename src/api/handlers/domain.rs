use crate::core::middleware::permission::{Application, CanCreate, CanDelete, CanRead};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::domain::{CreateDomainDto, DomainResponseDto, PatchDomainDto},
    core::cache::{AppStateCache, CacheKey},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::domain::DomainService,
};

type ApiError = (StatusCode, String);

pub struct DomainController {
    service: Arc<DomainService>,
    cache: Arc<AppStateCache>,
}

#[controller("/domains")]
impl DomainController {
    fn new(service: Arc<DomainService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }

    #[get]
    #[live(table = "domains")]
    async fn list_all(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
    ) -> Result<Json<Vec<DomainResponseDto>>, ApiError> {
        let items = self
            .service
            .list_all()
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(
            items.into_iter().map(DomainResponseDto::from).collect(),
        ))
    }

    #[get("/{id}")]
    #[live(table = "domains")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<DomainResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(DomainResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/application/{application_id}")]
    #[live(table = "domains")]
    async fn list_by_application(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<DomainResponseDto>>, ApiError> {
        let items = self
            .service
            .list_by_application(application_id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(
            items.into_iter().map(DomainResponseDto::from).collect(),
        ))
    }

    #[get("/compose/{compose_id}")]
    #[live(table = "domains")]
    async fn list_by_compose(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<DomainResponseDto>>, ApiError> {
        let items = self
            .service
            .list_by_compose(compose_id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(
            items.into_iter().map(DomainResponseDto::from).collect(),
        ))
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        ValidatedJson(body): ValidatedJson<CreateDomainDto>,
    ) -> Result<(StatusCode, Json<DomainResponseDto>), ApiError> {
        let app_id = body.application_id;
        let comp_id = body.compose_id;

        let created = self
            .service
            .create(body)
            .await
            .map(DomainResponseDto::from)
            .map(|domain| (StatusCode::CREATED, Json(domain)))
            .map_err(map_sqlx_error)?;

        if let Some(id) = app_id {
            self.cache.invalidate(&CacheKey::DomainsApp(id)).await;
        }
        if let Some(id) = comp_id {
            self.cache.invalidate(&CacheKey::DomainsCompose(id)).await;
        }

        Ok(created)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDomainDto>,
    ) -> Result<Json<DomainResponseDto>, ApiError> {
        let updated = self
            .service
            .patch(id, body)
            .await
            .map(DomainResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)?;

        self.cache.invalidate_all().await;
        Ok(updated)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDelete>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service.delete(id).await.map_err(map_sqlx_error)?;

        self.cache.invalidate_all().await;
        Ok(StatusCode::NO_CONTENT)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "domain not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(message) => (StatusCode::CONFLICT, message),
        other => {
            tracing::error!(error = %other, "domain database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
