use crate::core::middleware::permission::{Application, CanCreate, CanDelete, CanDeploy, CanRead};
use std::sync::Arc;

use auto_route::controller;
use axum::response::sse::{Event, Sse};
use axum::{Json, extract::Path, http::StatusCode};
use futures::StreamExt;
use std::{convert::Infallible, pin::Pin};
use tokio_stream::wrappers::BroadcastStream;

use crate::{
    api::dto::schedule::{
        CreateScheduleDto, PatchScheduleDto, ScheduleExecutionDto, ScheduleLogDto,
        ScheduleResponseDto, ScheduleRunResponseDto, ScheduleRuntimePolicyDto,
        UpdateScheduleRuntimePolicyDto,
    },
    core::cache::{AppStateCache, CacheKey},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::schedule::ScheduleService,
};

type ApiError = (StatusCode, String);
type ScheduleLogStream = Pin<Box<dyn futures::Stream<Item = Result<Event, Infallible>> + Send>>;
type ScheduleLogSse = Sse<ScheduleLogStream>;

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
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
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
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn list_by_application(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(application_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        let items = self
            .service
            .list_by_application(application_id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(
            items.into_iter().map(ScheduleResponseDto::from).collect(),
        ))
    }

    #[get("/compose/{compose_id}")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn list_by_compose(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(compose_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        let items = self
            .service
            .list_by_compose(compose_id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Json(
            items.into_iter().map(ScheduleResponseDto::from).collect(),
        ))
    }

    #[get("/database/{database_id}")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn list_by_database(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(_database_id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleResponseDto>>, ApiError> {
        Ok(Json(vec![]))
    }

    #[get("/server/{server_id}")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn list_by_server(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
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
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn list_by_organization(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
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
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        ValidatedJson(body): ValidatedJson<CreateScheduleDto>,
    ) -> Result<(StatusCode, Json<ScheduleResponseDto>), ApiError> {
        let app_id = body.application_id;
        let comp_id = body.compose_id;

        let created = self
            .service
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
        RequirePermission(_claims, _): RequirePermission<Application, CanCreate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchScheduleDto>,
    ) -> Result<Json<ScheduleResponseDto>, ApiError> {
        let updated = self
            .service
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
        RequirePermission(_claims, _): RequirePermission<Application, CanDelete>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service.delete(id).await.map_err(map_sqlx_error)?;

        self.cache.invalidate_all().await;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/{id}/trigger")]
    async fn trigger(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleRunResponseDto>, ApiError> {
        self.service
            .run_now(id)
            .await
            .map(ScheduleRunResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/runtime-policy")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn runtime_policy(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleRuntimePolicyDto>, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        self.service
            .repo_runtime
            .policy(id)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[put("/{id}/runtime-policy")]
    async fn update_runtime_policy(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
        Json(body): Json<UpdateScheduleRuntimePolicyDto>,
    ) -> Result<Json<ScheduleRuntimePolicyDto>, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        if !(0..=10).contains(&body.retry_count)
            || !(1..=86400).contains(&body.retry_delay_seconds)
            || !(30..=86400).contains(&body.lease_seconds)
        {
            return Err((
                StatusCode::BAD_REQUEST,
                "invalid schedule runtime policy".into(),
            ));
        }
        self.service
            .repo_runtime
            .update_policy(
                id,
                body.retry_count,
                body.retry_delay_seconds,
                body.missed_run_policy.as_str(),
                body.concurrency_policy.as_str(),
                body.lease_seconds,
                body.notify_on_success,
                body.notify_on_failure,
            )
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/executions")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn executions(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<ScheduleExecutionDto>>, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        self.service
            .repo_runtime
            .list(id, 200)
            .await
            .map(|rows| Json(rows.into_iter().map(Into::into).collect()))
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/logs")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn logs(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<ScheduleLogDto>, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        let content = crate::services::schedule::file_log::read(&format!("schedule-{id}"))
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        Ok(Json(ScheduleLogDto { content }))
    }

    #[get("/{id}/logs/stream", sse = ScheduleLogDto)]

    async fn logs_stream(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<ScheduleLogSse, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        let receiver = crate::services::schedule::file_log::subscribe(&format!("schedule-{id}"));
        let stream = BroadcastStream::new(receiver).filter_map(|item| async move {
            item.ok()
                .map(|entry| Ok(Event::default().event("log").data(entry)))
        });
        Ok(Sse::new(Box::pin(stream)))
    }

    #[get("/{id}/executions/{execution_id}/logs")]
    #[live(tables = ["schedules","schedule_runtime_policies","schedule_executions"])]
    async fn execution_logs(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path((id, execution_id)): Path<(i64, i64)>,
    ) -> Result<Json<ScheduleLogDto>, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        let belongs = self
            .service
            .repo_runtime
            .execution_exists(id, execution_id)
            .await
            .map_err(map_sqlx_error)?;
        if !belongs {
            return Err((StatusCode::NOT_FOUND, "schedule execution not found".into()));
        }
        let content = crate::services::schedule::file_log::read(&format!(
            "schedule-{id}-execution-{execution_id}"
        ))
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        Ok(Json(ScheduleLogDto { content }))
    }

    #[get("/{id}/executions/{execution_id}/logs/stream", sse = ScheduleLogDto)]

    async fn execution_logs_stream(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path((id, execution_id)): Path<(i64, i64)>,
    ) -> Result<ScheduleLogSse, ApiError> {
        self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        let belongs = self
            .service
            .repo_runtime
            .execution_exists(id, execution_id)
            .await
            .map_err(map_sqlx_error)?;
        if !belongs {
            return Err((StatusCode::NOT_FOUND, "schedule execution not found".into()));
        }
        let receiver = crate::services::schedule::file_log::subscribe(&format!(
            "schedule-{id}-execution-{execution_id}"
        ));
        let stream = BroadcastStream::new(receiver).filter_map(|item| async move {
            item.ok()
                .map(|entry| Ok(Event::default().event("log").data(entry)))
        });
        Ok(Sse::new(Box::pin(stream)))
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "schedule not found".into()),
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Protocol(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
        other => {
            tracing::error!(error = %other, "schedule database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
