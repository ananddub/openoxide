use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::sse::Sse,
};

use crate::{
    api::dto::deployment::{
        ActiveDeploymentDto, ComposeLogQuery, DeploymentListQuery, DeploymentResponseDto,
        DeploymentSseEventDto, DockerLogQuery, DockerStatsQuery,
    },
    services::deployment::{CancelDeploymentResult, DeploymentListFilter, DeploymentService},
    utils::builder::custom_type::IdType,
};

use stream::{
    DeploymentEventStream, deployment_event_stream, deployment_log_stream, docker_stats_stream,
    docker_stream,
};

type ApiError = (StatusCode, String);
type DeploymentSse = Sse<DeploymentEventStream>;

pub struct DeploymentController {
    service: Arc<DeploymentService>,
}

#[controller("/deployments")]
impl DeploymentController {
    fn new(service: Arc<DeploymentService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<DeploymentListQuery>,
    ) -> Result<Json<Vec<DeploymentResponseDto>>, ApiError> {
        self.service
            .list(DeploymentListFilter {
                status: query.status,
                state: query.state,
                application_id: query.application_id,
                compose_id: query.compose_id,
                database_id: query.database_id,
                server_id: query.server_id,
                limit: query.limit.unwrap_or(50),
                offset: query.offset.unwrap_or(0),
            })
            .await
            .map(|items| items.into_iter().map(DeploymentResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/active")]
    async fn active(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
    ) -> Result<Json<Vec<ActiveDeploymentDto>>, ApiError> {
        self.service
            .list_active_components()
            .await
            .map(|items| items.into_iter().map(ActiveDeploymentDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/running")]
    async fn running(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<DeploymentListQuery>,
    ) -> Result<Json<Vec<DeploymentResponseDto>>, ApiError> {
        self.service
            .list_running(query.limit.unwrap_or(50), query.offset.unwrap_or(0))
            .await
            .map(|items| items.into_iter().map(DeploymentResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<Json<DeploymentResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(DeploymentResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/application/{id}/events", sse = DeploymentSseEventDto)]
    async fn application_events(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        self.events(IdType::AppId(id)).await
    }

    #[get("/compose/{id}/events", sse = DeploymentSseEventDto)]
    async fn compose_events(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        self.events(IdType::ComposeId(id)).await
    }

    #[get("/database/{id}/events", sse = DeploymentSseEventDto)]
    async fn database_events(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        self.events(IdType::DatabaseId(id)).await
    }

    #[get("/application/{id}/stats", sse = DeploymentSseEventDto)]
    async fn application_stats(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_application_stats(id, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/compose/{id}/stats", sse = DeploymentSseEventDto)]
    async fn compose_stats(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_compose_stats(id, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/docker/container/{target}/logs", sse = DeploymentSseEventDto)]
    async fn docker_container_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_docker_container_logs(
                query.server_id,
                target,
                docker_log_options(query.tail, query.timestamps, query.follow),
            )
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver)))
    }

    #[get("/docker/stats", sse = DeploymentSseEventDto)]
    async fn docker_global_stats(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_global_stats(query.server_id, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/docker/container/{target}/stats", sse = DeploymentSseEventDto)]
    async fn docker_container_stats(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_docker_container_stats(query.server_id, target, query.stream.unwrap_or(true))
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stats_stream(receiver)))
    }

    #[get("/docker/service/{target}/logs", sse = DeploymentSseEventDto)]
    async fn docker_service_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_docker_service_logs(
                query.server_id,
                target,
                docker_log_options(query.tail, query.timestamps, query.follow),
            )
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver)))
    }

    #[get("/docker/compose/logs", sse = DeploymentSseEventDto)]
    async fn docker_compose_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<ComposeLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let server_id = query.server_id;
        let args = compose_log_args(query);
        let receiver = self
            .service
            .stream_docker_compose_logs(server_id, args)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver)))
    }

    #[post("/{id}/cancel")]
    async fn cancel(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.cancel(id).await {
            Ok(CancelDeploymentResult::CancelRequested) => Ok(StatusCode::ACCEPTED),
            Ok(CancelDeploymentResult::NotRunning) => Err((
                StatusCode::CONFLICT,
                "deployment is not running, so it cannot be cancelled".into(),
            )),
            Ok(CancelDeploymentResult::NotCancellable) => Err((
                StatusCode::BAD_REQUEST,
                "deployment is not attached to an application or compose project".into(),
            )),
            Ok(CancelDeploymentResult::NotActiveInThisProcess) => Err((
                StatusCode::CONFLICT,
                "deployment is not active in this process; it may already be finished or recovered after restart".into(),
            )),
            Err(error) => Err(map_sqlx_error(error)),
        }
    }

    #[get("/{id}/logs", sse = DeploymentSseEventDto)]
    async fn stream_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_deployment_log(id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(deployment_log_stream(receiver)))
    }

    #[get("/application/{id}/logs", sse = DeploymentSseEventDto)]
    async fn application_latest_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_application_latest_log(id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(deployment_log_stream(receiver)))
    }

    #[get("/compose/{id}/logs", sse = DeploymentSseEventDto)]
    async fn compose_latest_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_compose_latest_log(id)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(deployment_log_stream(receiver)))
    }

    async fn events(&self, id: IdType) -> Result<DeploymentSse, ApiError> {
        let Some(subscription) = self
            .service
            .subscribe_component(id)
            .await
            .map_err(map_sqlx_error)?
        else {
            return Err((
                StatusCode::NOT_FOUND,
                "active deployment stream not found".into(),
            ));
        };

        Ok(Sse::new(deployment_event_stream(subscription)))
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "deployment not found".into()),
        other => {
            tracing::error!(error = %other, "deployment operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "deployment operation failed".into(),
            )
        }
    }
}

fn docker_log_options(
    tail: Option<usize>,
    timestamps: Option<bool>,
    follow: Option<bool>,
) -> Vec<String> {
    let mut args = Vec::new();
    if follow.unwrap_or(true) {
        args.push("--follow".into());
    }
    if timestamps.unwrap_or(false) {
        args.push("--timestamps".into());
    }
    let tail = tail.unwrap_or(200).to_string();
    args.extend(["--tail".into(), tail]);
    args
}

fn compose_log_args(query: ComposeLogQuery) -> Vec<String> {
    let mut args = vec!["compose".into()];

    if let Some(file) = query.file {
        args.extend(["--file".into(), file]);
    }
    if let Some(project_dir) = query.project_dir {
        args.extend(["--project-directory".into(), project_dir]);
    }
    if let Some(project_name) = query.project_name {
        args.extend(["--project-name".into(), project_name]);
    }

    args.push("logs".into());
    args.extend(docker_log_options(
        query.tail,
        query.timestamps,
        query.follow,
    ));

    if let Some(service) = query.service {
        args.push(service);
    }

    args
}

pub mod stream;
