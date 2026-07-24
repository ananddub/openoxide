use std::sync::Arc;

use auto_route::controller;
use axum::{Json, extract::Path, http::StatusCode};

use crate::{
    api::dto::application::{
        ApplicationOperationResponseDto, ApplicationResponseDto, CreateApplicationDto,
        PatchApplicationDto, PatchBitbucketSourceDto, PatchBuildConfigDto, PatchCustomGitSourceDto,
        PatchDockerSourceDto, PatchDropSourceDto, PatchGiteaSourceDto, PatchGithubSourceDto,
        PatchGitlabSourceDto, PatchResourceConfigDto,
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppDeployPermission, AppReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::application::{ApplicationOperation, ApplicationOperationResult, ApplicationService},
};

type ApiError = (StatusCode, String);

pub struct ApplicationController {
    service: Arc<ApplicationService>,
}

#[controller("/applications")]
impl ApplicationController {
    fn new(service: Arc<ApplicationService>) -> Self {
        Self { service }
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
        sqlx::Error::Protocol(message) if message.contains("already running") => {
            (StatusCode::CONFLICT, message)
        }
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
