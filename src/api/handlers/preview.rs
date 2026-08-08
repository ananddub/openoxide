use crate::core::middleware::permission::{Application, CanDelete, CanDeploy, CanRead};
use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use std::sync::Arc;

use crate::{
    api::dto::preview::{CreatePreviewDeploymentDto, PreviewListQueryDto},
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::{
        preview::{PreviewDeploymentOutcome, PreviewDeploymentService, PreviewDeploymentView},
        webhook::{GitProviderKind, PullRequestEvent},
    },
};

type ApiError = (StatusCode, String);

pub struct PreviewDeploymentController {
    service: Arc<PreviewDeploymentService>,
}

#[controller("/preview-deployments")]
impl PreviewDeploymentController {
    fn new(service: Arc<PreviewDeploymentService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Query(query): Query<PreviewListQueryDto>,
    ) -> Result<Json<Vec<PreviewDeploymentView>>, ApiError> {
        self.service
            .list(query.active_only.unwrap_or(true))
            .await
            .map(Json)
            .map_err(map_sqlx)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<PreviewDeploymentView>, ApiError> {
        self.service.get(id).await.map(Json).map_err(map_sqlx)
    }

    #[post("/application/{application_id}")]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(application_id): Path<i64>,
        ValidatedJson(body): ValidatedJson<CreatePreviewDeploymentDto>,
    ) -> Result<(StatusCode, Json<PreviewDeploymentOutcome>), ApiError> {
        let event = event(body)?;
        self.service
            .deploy_application(application_id, &event)
            .await
            .map(|outcome| (StatusCode::ACCEPTED, Json(outcome)))
            .map_err(map_service)
    }

    #[post("/{id}/redeploy")]
    async fn redeploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<PreviewDeploymentOutcome>), ApiError> {
        self.service
            .redeploy(id)
            .await
            .map(|outcome| (StatusCode::ACCEPTED, Json(outcome)))
            .map_err(map_service)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanDelete>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .remove(id, true)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_service)
    }
}

fn event(body: CreatePreviewDeploymentDto) -> Result<PullRequestEvent, ApiError> {
    let provider: GitProviderKind = body
        .provider
        .parse()
        .map_err(|e: String| (StatusCode::BAD_REQUEST, e))?;
    Ok(PullRequestEvent {
        provider,
        owner: body.owner,
        repository: body.repository,
        number: body.pull_request_number,
        action: "opened".into(),
        source_branch: body.source_branch,
        source_owner: body.source_owner,
        source_repository: body.source_repository,
        target_branch: body.target_branch,
        commit: body.commit_sha,
        author: body.author,
    })
}

fn map_sqlx(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "preview deployment not found".into()),
        other => {
            tracing::error!(error = %other, "preview deployment database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "preview deployment operation failed".into(),
            )
        }
    }
}

fn map_service(error: String) -> ApiError {
    let status = if error.to_ascii_lowercase().contains("not found") {
        StatusCode::NOT_FOUND
    } else if error.contains("already queued or running") {
        StatusCode::CONFLICT
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, error)
}
