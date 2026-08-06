use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    body::Body,
    extract::{Path, Query},
    http::{HeaderValue, Response, StatusCode, header},
    response::sse::Sse,
};

use crate::{
    api::dto::deployment::{
        ActiveDeploymentDto, ComposeLogQuery, DeploymentListQuery, DeploymentResponseDto,
        DeploymentSseEventDto, DockerLogQuery, DockerStatsQuery, LogSearchQuery,
    },
    services::deployment::{
        CancelDeploymentResult, ComposeLogOptions, DeploymentListFilter, DeploymentService,
        DockerLogOptions, LogSearchOptions,
    },
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

    #[get("/logs/search")]
    async fn search_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<LogSearchQuery>,
    ) -> Result<Json<Vec<crate::services::deployment::LogSearchResult>>, ApiError> {
        self.service
            .search_logs(LogSearchOptions {
                query: query.query,
                limit: query.limit.unwrap_or(200).clamp(1, 5000),
            })
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/logs/cleanup")]
    async fn cleanup_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
    ) -> Result<Json<serde_json::Value>, ApiError> {
        let cutoff = chrono::Utc::now().timestamp() - 30 * 24 * 60 * 60;
        let removed = self
            .service
            .cleanup_logs_before(cutoff)
            .await
            .map_err(map_sqlx_error)?;
        Ok(Json(
            serde_json::json!({ "removed": removed, "cutoff": cutoff }),
        ))
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

    #[get("/database/{id}/stats", sse = DeploymentSseEventDto)]
    async fn database_stats(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
        Query(query): Query<DockerStatsQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_database_stats(id, query.stream.unwrap_or(true))
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
                docker_log_options(
                    query.tail,
                    query.timestamps,
                    query.follow,
                    query.since,
                    query.until,
                ),
            )
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(
            receiver,
            query.stream.unwrap_or_default(),
        )))
    }

    #[get("/docker/container/{target}/logs/export")]
    async fn export_docker_container_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<Response<Body>, ApiError> {
        let selector = query.stream.unwrap_or_default();
        let output = self
            .service
            .docker_container_logs(
                query.server_id,
                target.clone(),
                docker_log_options(
                    query.tail,
                    query.timestamps,
                    Some(false),
                    query.since,
                    query.until,
                ),
            )
            .await
            .map_err(map_sqlx_error)?;
        let bytes = match selector {
            crate::api::dto::deployment::DockerLogStream::All => {
                format!("{}{}", output.stdout, output.stderr).into_bytes()
            }
            crate::api::dto::deployment::DockerLogStream::Stdout => output.stdout.into_bytes(),
            crate::api::dto::deployment::DockerLogStream::Stderr => output.stderr.into_bytes(),
        };
        let filename = format!("{}.log", sanitize_filename(&target));
        let mut response = Response::new(Body::from(bytes));
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
                .map_err(|error| (StatusCode::BAD_REQUEST, error.to_string()))?,
        );
        Ok(response)
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

    #[get("/docker/containers")]
    async fn docker_global_containers(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<Json<Vec<crate::utils::docker::ContainerSummary>>, ApiError> {
        self.service
            .list_docker_containers(query.server_id, true)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
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
                docker_log_options(
                    query.tail,
                    query.timestamps,
                    query.follow,
                    query.since,
                    query.until,
                ),
            )
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(
            receiver,
            query.stream.unwrap_or_default(),
        )))
    }

    #[get("/docker/service/{target}/logs/export")]
    async fn export_docker_service_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(target): Path<String>,
        Query(query): Query<DockerLogQuery>,
    ) -> Result<Response<Body>, ApiError> {
        let selector = query.stream.unwrap_or_default();
        let output = self
            .service
            .export_container_logs(
                query.server_id,
                &target,
                docker_log_options(
                    query.tail,
                    query.timestamps,
                    Some(false),
                    query.since,
                    query.until,
                ),
            )
            .await
            .map_err(map_sqlx_error)?;
        log_download_response(&target, select_log_output(output, selector))
    }

    #[get("/docker/compose/logs", sse = DeploymentSseEventDto)]
    async fn docker_compose_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<ComposeLogQuery>,
    ) -> Result<DeploymentSse, ApiError> {
        let server_id = query.server_id;
        let stream = query.stream.unwrap_or_default();
        let options = compose_log_options(query);
        let receiver = self
            .service
            .stream_docker_compose_logs(server_id, options)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Sse::new(docker_stream(receiver, stream)))
    }

    #[get("/docker/compose/logs/export")]
    async fn export_docker_compose_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Query(query): Query<ComposeLogQuery>,
    ) -> Result<Response<Body>, ApiError> {
        let server_id = query.server_id;
        let filename = query.service.clone().unwrap_or_else(|| "compose".into());
        let selector = query.stream.unwrap_or_default();
        let output = self
            .service
            .export_compose_logs(server_id, compose_log_options(query))
            .await
            .map_err(map_sqlx_error)?;
        log_download_response(&filename, select_log_output(output, selector))
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

    #[get("/database/{id}/logs", sse = DeploymentSseEventDto)]
    async fn database_latest_logs(
        &self,
        _claims: crate::utils::jwt::claim::Claims,
        Path(id): Path<i64>,
    ) -> Result<DeploymentSse, ApiError> {
        let receiver = self
            .service
            .stream_database_latest_log(id)
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
    since: Option<String>,
    until: Option<String>,
) -> DockerLogOptions {
    DockerLogOptions {
        tail: tail.unwrap_or(200).clamp(1, 100_000),
        timestamps: timestamps.unwrap_or(false),
        follow: follow.unwrap_or(true),
        since,
        until,
    }
}

fn sanitize_filename(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => character,
            _ => '_',
        })
        .collect();
    if sanitized.is_empty() {
        "container".into()
    } else {
        sanitized
    }
}

fn log_download_response(name: &str, bytes: Vec<u8>) -> Result<Response<Body>, ApiError> {
    let filename = format!("{}.log", sanitize_filename(name));
    let mut response = Response::new(Body::from(bytes));
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{filename}\""))
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?,
    );
    Ok(response)
}

fn select_log_output(
    output: crate::utils::docker::DockerOutput,
    selector: crate::api::dto::deployment::DockerLogStream,
) -> Vec<u8> {
    match selector {
        crate::api::dto::deployment::DockerLogStream::All => {
            format!("{}{}", output.stdout, output.stderr).into_bytes()
        }
        crate::api::dto::deployment::DockerLogStream::Stdout => output.stdout.into_bytes(),
        crate::api::dto::deployment::DockerLogStream::Stderr => output.stderr.into_bytes(),
    }
}

fn compose_log_options(query: ComposeLogQuery) -> ComposeLogOptions {
    ComposeLogOptions {
        file: query.file,
        project_directory: query.project_dir,
        project_name: query.project_name,
        service: query.service,
        logs: docker_log_options(
            query.tail,
            query.timestamps,
            query.follow,
            query.since,
            query.until,
        ),
    }
}

pub mod stream;
