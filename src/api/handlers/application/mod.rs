use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::dto::application::import_export::{
        ApplicationExportArchiveDto, ApplicationExportQueryDto, ImportApplicationDto,
    },
    api::dto::application::{
        ApplicationCleanupResponseDto, ApplicationForceKillResponseDto,
        ApplicationOperationResponseDto, ApplicationResponseDto, ApplicationRollbackResponseDto,
        ApplicationRollbackTriggerResponseDto, ApplicationWebhookTokenResponseDto,
        CreateApplicationDto, MoveApplicationDto, PatchApplicationDto, PatchBitbucketSourceDto,
        PatchBuildConfigDto, PatchCustomGitSourceDto, PatchDockerSourceDto, PatchDropSourceDto,
        PatchGiteaSourceDto, PatchGithubSourceDto, PatchGitlabSourceDto, PatchPreviewConfigDto,
        PatchResourceConfigDto,
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppDeployPermission, AppReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::application::{
        ApplicationOperation, ApplicationOperationResult, ApplicationService,
        import_export::ApplicationTransferService,
    },
};

pub mod middleware;
pub mod mount;
pub mod network;
pub mod patch;
pub mod port;
pub mod redirect;
pub mod security;

type ApiError = (StatusCode, String);

pub struct ApplicationController {
    service: Arc<ApplicationService>,
    transfer: Arc<ApplicationTransferService>,
}

#[controller("/applications")]
impl ApplicationController {
    fn new(service: Arc<ApplicationService>, transfer: Arc<ApplicationTransferService>) -> Self {
        Self { service, transfer }
    }

    #[post("/import")]
    async fn import_application(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<ImportApplicationDto>,
    ) -> Result<(StatusCode, Json<ApplicationResponseDto>), ApiError> {
        self.transfer
            .import(body)
            .await
            .map(ApplicationResponseDto::from)
            .map(|item| (StatusCode::CREATED, Json(item)))
            .map_err(map_sqlx_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/export")]
    async fn export_application(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
        Query(query): Query<ApplicationExportQueryDto>,
    ) -> Result<Json<ApplicationExportArchiveDto>, ApiError> {
        let bundle = self
            .transfer
            .export(id, query.include_secrets)
            .await
            .map_err(map_sqlx_error)?;
        let archive = serde_json::to_string_pretty(&bundle).map_err(|error| {
            tracing::error!(%error, "could not serialize application export");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not serialize application export".into(),
            )
        })?;
        Ok(Json(ApplicationExportArchiveDto {
            format: "rustploy.application+json".into(),
            schema_version: i64::from(bundle.schema_version),
            archive,
        }))
    }

    #[get("/environment/{environment_id}")]
    async fn list_by_environment(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(environment_id): Path<i64>,
    ) -> Result<Json<Vec<ApplicationResponseDto>>, ApiError> {
        self.service
            .list_by_environment(environment_id)
            .await
            .map(|items| {
                items
                    .into_iter()
                    .map(ApplicationResponseDto::from)
                    .collect()
            })
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateApplicationDto>,
    ) -> Result<(StatusCode, Json<ApplicationResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(ApplicationResponseDto::from)
            .map(|application| (StatusCode::CREATED, Json(application)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchApplicationDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/github")]
    async fn patch_github_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchGithubSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_github_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/gitlab")]
    async fn patch_gitlab_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchGitlabSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_gitlab_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/bitbucket")]
    async fn patch_bitbucket_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchBitbucketSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_bitbucket_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/gitea")]
    async fn patch_gitea_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchGiteaSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_gitea_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/docker")]
    async fn patch_docker_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDockerSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_docker_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/git")]
    async fn patch_git_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchCustomGitSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_custom_git_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/drop")]
    async fn patch_drop_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchDropSourceDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .set_drop_source(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/build")]
    async fn patch_build(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchBuildConfigDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .patch_build_config(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/resources")]
    async fn patch_resources(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchResourceConfigDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .patch_resource_config(id, body)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/preview")]
    async fn patch_preview(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchPreviewConfigDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .patch_preview_config(id, body)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/deploy")]
    async fn deploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ApplicationOperationResponseDto>), ApiError> {
        self.operation(id, ApplicationOperation::Deploy).await
    }

    #[post("/{id}/redeploy")]
    async fn redeploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ApplicationOperationResponseDto>), ApiError> {
        self.operation(id, ApplicationOperation::Redeploy).await
    }

    #[post("/{id}/rebuild")]
    async fn rebuild(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ApplicationOperationResponseDto>), ApiError> {
        self.operation(id, ApplicationOperation::Rebuild).await
    }

    #[post("/{id}/reload")]
    async fn reload(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ApplicationOperationResponseDto>), ApiError> {
        self.operation(id, ApplicationOperation::Reload).await
    }

    #[post("/{id}/start")]
    async fn start(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ApplicationOperationResponseDto>), ApiError> {
        self.operation(id, ApplicationOperation::Start).await
    }

    #[post("/{id}/cancel")]
    async fn cancel(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.cancel_operation(id).await {
            Ok(_) => Ok(StatusCode::OK),
            Err(error) => Err(map_sqlx_error(error)),
        }
    }

    #[post("/{id}/webhook-token/rotate")]
    async fn rotate_webhook_token(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ApplicationWebhookTokenResponseDto>, ApiError> {
        self.service
            .rotate_webhook_token(id)
            .await
            .map(|token| Json(ApplicationWebhookTokenResponseDto { token }))
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/move")]
    async fn move_to_environment(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<MoveApplicationDto>,
    ) -> Result<Json<ApplicationResponseDto>, ApiError> {
        self.service
            .move_to_environment(id, body.target_environment_id)
            .await
            .map(ApplicationResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}/deployments/history")]
    async fn clear_deployment_history(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ApplicationCleanupResponseDto>, ApiError> {
        self.service
            .clear_deployment_history(id)
            .await
            .map(|result| {
                Json(ApplicationCleanupResponseDto {
                    affected: result.affected as i64,
                })
            })
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/deployments/queue/cleanup")]
    async fn cleanup_deployment_queue(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ApplicationCleanupResponseDto>, ApiError> {
        self.service
            .cleanup_deployment_queue(id)
            .await
            .map(|result| {
                Json(ApplicationCleanupResponseDto {
                    affected: result.affected as i64,
                })
            })
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/deployments/force-kill")]
    async fn force_kill_deployment(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ApplicationForceKillResponseDto>, ApiError> {
        self.service
            .force_kill_deployment(id)
            .await
            .map(|result| {
                let status = match result.result {
                    crate::services::deployment::CancelDeploymentResult::CancelRequested => {
                        "CANCEL_REQUESTED"
                    }
                    crate::services::deployment::CancelDeploymentResult::NotRunning => {
                        "NOT_RUNNING"
                    }
                    crate::services::deployment::CancelDeploymentResult::NotCancellable => {
                        "NOT_CANCELLABLE"
                    }
                    crate::services::deployment::CancelDeploymentResult::NotActiveInThisProcess => {
                        "NOT_ACTIVE_IN_THIS_PROCESS"
                    }
                };
                Json(ApplicationForceKillResponseDto {
                    deployment_id: result.deployment_id,
                    status: status.into(),
                })
            })
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/rollbacks")]
    async fn list_rollbacks(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<ApplicationRollbackResponseDto>>, ApiError> {
        self.service
            .list_rollbacks(id)
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/rollbacks/{rollback_id}/trigger")]
    async fn trigger_rollback(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path((id, rollback_id)): Path<(i64, i64)>,
    ) -> Result<Json<ApplicationRollbackTriggerResponseDto>, ApiError> {
        self.service
            .trigger_rollback(id, rollback_id)
            .await
            .map(|result| {
                Json(ApplicationRollbackTriggerResponseDto {
                    deployment_id: result.deployment_id,
                    message: result.message,
                })
            })
            .map_err(map_sqlx_error)
    }

    #[delete("/{id}/rollbacks/{rollback_id}")]
    async fn delete_rollback(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path((id, rollback_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, ApiError> {
        match self.service.delete_rollback(id, rollback_id).await {
            Ok(true) => Ok(StatusCode::NO_CONTENT),
            Ok(false) => Err((StatusCode::NOT_FOUND, "rollback not found".into())),
            Err(error) => Err(map_sqlx_error(error)),
        }
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
            .map(|()| StatusCode::NO_CONTENT)
            .map_err(map_sqlx_error)
    }

    #[get("/{id}/dependencies")]
    async fn dependencies(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<crate::repository::ResourceDependencyCounts>, ApiError> {
        self.service
            .dependencies(id)
            .await
            .map(Json)
            .map_err(map_sqlx_error)
    }

    async fn operation(
        &self,
        id: i64,
        operation: ApplicationOperation,
    ) -> Result<(StatusCode, Json<ApplicationOperationResponseDto>), ApiError> {
        self.service
            .run_operation(id, operation)
            .await
            .map(operation_response)
            .map(|response| (StatusCode::ACCEPTED, Json(response)))
            .map_err(map_sqlx_error)
    }
}

fn operation_response(value: ApplicationOperationResult) -> ApplicationOperationResponseDto {
    ApplicationOperationResponseDto {
        application: ApplicationResponseDto::from(value.application),
        deployment_id: value.deployment_id,
        operation: value.operation.as_str().into(),
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "application not found".into()),
        sqlx::Error::Protocol(message) if message.contains("already queued or running") => {
            (StatusCode::CONFLICT, message)
        }
        sqlx::Error::Protocol(message) => (StatusCode::BAD_REQUEST, message),
        sqlx::Error::Database(ref database_error) if database_error.is_foreign_key_violation() => {
            (StatusCode::NOT_FOUND, "related resource not found".into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_unique_violation() => {
            (StatusCode::CONFLICT, database_error.message().into())
        }
        sqlx::Error::Database(ref database_error) if database_error.is_check_violation() => {
            (StatusCode::BAD_REQUEST, database_error.message().into())
        }
        other => {
            tracing::error!(error = %other, "application database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
