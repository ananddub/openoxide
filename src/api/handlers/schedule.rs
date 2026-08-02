use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::schedule::{
        CreateScheduleDto, PatchScheduleDto, ScheduleResponseDto, ScheduleRunResponseDto,
    },
    core::cache::{AppStateCache, CacheKey},
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppDeployPermission, AppReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::schedule::ScheduleService,
};

type ApiError = (StatusCode, String);

pub struct ScheduleController {
    service: Arc<ScheduleService>,
    cache: Arc<AppStateCache>,
}

#[controller("/schedules")]
impl ScheduleController {
    fn new(service: Arc<ScheduleService>, cache: Arc<AppStateCache>) -> Self {
        Self { service, cache }
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(ScheduleResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/application/{application_id}")]
    async fn list_by_application(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        let items = self.service
            .list_by_application(application_id)
            .await
            .map_err(map_sqlx_error)?;
        
        Ok(Json(items.into_iter().map(ScheduleResponseDto::from).collect()))
    }

    #[get("/compose/{compose_id}")]
    async fn list_by_compose(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        let items = self.service
            .list_by_compose(compose_id)
            .await
            .map_err(map_sqlx_error)?;
        
        Ok(Json(items.into_iter().map(ScheduleResponseDto::from).collect()))
    }

    #[get("/database/{database_id}")]
    async fn list_by_database(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(_database_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        Ok(Json(vec![]))
    }

    #[get("/server/{server_id}")]
    async fn list_by_server(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(server_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        self.service
            .list_by_server(server_id)
            .await
            .map(|items| items.into_iter().map(ScheduleResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/organization/{organization_id}")]
    async fn list_by_organization(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(organization_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        self.service
            .list_by_organization(organization_id)
            .await
            .map(|items| items.into_iter().map(ScheduleResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateScheduleDto>,
    ) -> Result<(StatusCode, Json<ScheduleResponseDto>), ApiError> {
        let app_id = body.application_id;
        let comp_id = body.compose_id;

        let created = self.service
            .create(body)
            .await
            .map(ScheduleResponseDto::from)
            .map(|schedule| (StatusCode::CREATED, Json(schedule)))
            .map_err(map_sqlx_error)?;

        if let Some(id) = app_id {
            self.cache.invalidate(&CacheKey::SchedulesApp(id)).await;
        }
        if let Some(id) = comp_id {
            self.cache.invalidate(&CacheKey::SchedulesCompose(id)).await;
        }

        Ok(created)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchScheduleDto>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        let updated = self.service
            .patch(id, body)
            .await
            .map(ScheduleResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)?;

        self.cache.invalidate_all().await;
        Ok(updated)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map_err(map_sqlx_error)?;
            
        self.cache.invalidate_all().await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/{id}/trigger")]
    async fn trigger(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleRunResponseDto>, ApiError> {
        self.service
            .run_now(id)
            .await
            .map(ScheduleRunResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "schedule not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "schedule database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
