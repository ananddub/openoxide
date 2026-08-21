use crate::core::middleware::permission::{CanCreate, CanDelete, CanRead, Project};
use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::project::{CreateProjectDto, PatchProjectDto, ProjectResponseDto},
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::project::ProjectService,
};

type ApiError = (StatusCode, String);

pub struct ProjectController {
    service: Arc<ProjectService>,
    cache: Arc<AppStateCache>,
}

#[controller("/projects")]
impl ProjectController {
    fn new(service: Arc<ProjectService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }

    #[get("/{id}")]
    #[live(tables = ["projects","environments"])]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<ProjectResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(ProjectResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/organization/{organization_id}")]
    #[live(tables = ["projects","environments"])]
    async fn list_by_organization(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanRead>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<ProjectResponseDto>>, ApiError> {
        if let Some(CacheEnum::ProjectsList(cached)) = self
            .cache
            .get(&CacheKey::ProjectsList(organization_id))
            .await
        {
            return Ok(Json(
                cached.into_iter().map(ProjectResponseDto::from).collect(),
            ));
        }

        let items = self
            .service
            .list_by_organization(organization_id)
            .await
            .map_err(map_sqlx_error)?;

        self.cache
            .insert(
                CacheKey::ProjectsList(organization_id),
                CacheEnum::ProjectsList(items.clone()),
            )
            .await;

        Ok(Json(
            items.into_iter().map(ProjectResponseDto::from).collect(),
        ))
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanCreate>,
        ValidatedJson(body): ValidatedJson<CreateProjectDto>,
    ) -> Result<(StatusCode, Json<ProjectResponseDto>), ApiError> {
        let org_id = body.organization_id;
        let created = self
            .service
            .create(body)
            .await
            .map(ProjectResponseDto::from)
            .map(|project| (StatusCode::CREATED, Json(project)))
            .map_err(map_sqlx_error)?;

        self.cache.invalidate(&CacheKey::ProjectsList(org_id)).await;
        Ok(created)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanCreate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchProjectDto>,
    ) -> Result<Json<ProjectResponseDto>, ApiError> {
        let updated = self
            .service
            .update(id, body)
            .await
            .map(ProjectResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)?;

        self.cache.invalidate_all().await;
        Ok(updated)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Project, CanDelete>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service.delete(id).await.map_err(map_sqlx_error)?;

        // Project deletion cascades through environments/resources and removes
        // their schedules and backups. Remove those jobs from memory now.
        if let Ok(runner) = auto_di::resolve::<crate::services::schedule::ScheduleRunner>().await {
            if let Err(error) = runner.refresh_jobs().await {
                tracing::error!(%error, "could not refresh scheduler after project deletion");
            }
        }

        self.cache.invalidate_all().await;
        Ok(StatusCode::NO_CONTENT)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "project not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "project database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
