use std::sync::Arc;

use auto_route::controller;
use axum::{
    Json,
    extract::{Multipart, Path},
    http::StatusCode,
};

use crate::{
    api::dto::compose::{
        ComposeOperationResponseDto, ComposeResponseDto, CreateComposeDto,
        PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto, PatchComposeDto,
        PatchComposeGiteaSourceDto, PatchComposeGithubSourceDto, PatchComposeGitlabSourceDto,
        PatchComposeRawSourceDto,
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppDeployPermission, AppReadPermission,
            RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::compose::{ComposeOperation, ComposeService},
};

type ApiError = (StatusCode, String);

pub mod management;

pub struct ComposeController {
    service: Arc<ComposeService>,
}

#[controller("/compose")]
impl ComposeController {
    fn new(service: Arc<ComposeService>) -> Self {
        Self { service }
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .get_by_id(id)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[get("/environment/{environment_id}")]
    async fn list_by_environment(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(environment_id): Path<i64>,
    ) -> Result<Json<Vec<ComposeResponseDto>>, ApiError> {
        self.service
            .list_by_environment(environment_id)
            .await
            .map(|items| items.into_iter().map(ComposeResponseDto::from).collect())
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post]
    async fn create(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateComposeDto>,
    ) -> Result<(StatusCode, Json<ComposeResponseDto>), ApiError> {
        self.service
            .create(body)
            .await
            .map(ComposeResponseDto::from)
            .map(|compose| (StatusCode::CREATED, Json(compose)))
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}")]
    async fn patch(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .patch(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/github")]
    async fn patch_github_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeGithubSourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .set_github_source(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/gitlab")]
    async fn patch_gitlab_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeGitlabSourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .set_gitlab_source(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/bitbucket")]
    async fn patch_bitbucket_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeBitbucketSourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .set_bitbucket_source(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/gitea")]
    async fn patch_gitea_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeGiteaSourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .set_gitea_source(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/git")]
    async fn patch_git_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeCustomGitSourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .set_custom_git_source(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[patch("/{id}/source/raw")]
    async fn patch_raw_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<PatchComposeRawSourceDto>,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .set_raw_source(id, body)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(map_sqlx_error)
    }

    #[post("/{id}/source/upload")]
    async fn upload_source(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        multipart: Multipart,
    ) -> Result<Json<ComposeResponseDto>, ApiError> {
        self.service
            .upload_drop_source(id, multipart)
            .await
            .map(ComposeResponseDto::from)
            .map(Json)
            .map_err(|err| (StatusCode::BAD_REQUEST, err))
    }

    #[post("/{id}/deploy")]
    async fn deploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ComposeOperationResponseDto>), ApiError> {
        self.operation(id, ComposeOperation::Deploy).await
    }

    #[post("/{id}/redeploy")]
    async fn redeploy(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ComposeOperationResponseDto>), ApiError> {
        self.operation(id, ComposeOperation::Redeploy).await
    }

    #[post("/{id}/reload")]
    async fn reload(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ComposeOperationResponseDto>), ApiError> {
        self.operation(id, ComposeOperation::Reload).await
    }

    #[post("/{id}/start")]
    async fn start(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<(StatusCode, Json<ComposeOperationResponseDto>), ApiError> {
        self.operation(id, ComposeOperation::Start).await
    }

    #[post("/{id}/stop")]
    async fn stop(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ComposeOperationResponseDto>, ApiError> {
        self.service
            .cancel_operation(id)
            .await
            .map_err(map_sqlx_error)?;
        let compose = self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        Ok(Json(ComposeOperationResponseDto {
            compose: compose.into(),
            deployment_id: None,
            operation: ComposeOperation::Stop.as_str().to_string(),
        }))
    }

    #[post("/{id}/cancel")]
    async fn cancel(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeployPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<ComposeOperationResponseDto>, ApiError> {
        self.service
            .cancel_operation(id)
            .await
            .map_err(map_sqlx_error)?;
        let compose = self.service.get_by_id(id).await.map_err(map_sqlx_error)?;
        Ok(Json(ComposeOperationResponseDto {
            compose: compose.into(),
            deployment_id: None,
            operation: ComposeOperation::Stop.as_str().to_string(),
        }))
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
        operation: ComposeOperation,
    ) -> Result<(StatusCode, Json<ComposeOperationResponseDto>), ApiError> {
        self.service
            .run_operation(id, operation)
            .await
            .map(ComposeOperationResponseDto::from)
            .map(|response| (StatusCode::ACCEPTED, Json(response)))
            .map_err(map_sqlx_error)
    }
}

fn map_sqlx_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "compose project not found".into()),
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
            tracing::error!(error = %other, "compose database operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database operation failed".into(),
            )
        }
    }
}
