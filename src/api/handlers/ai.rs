use crate::core::middleware::permission::{
    Application, CanCreate, CanDeploy, CanRead, CanWrite, Organization,
};
use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use std::sync::Arc;

use crate::{
    api::dto::ai::{
        AiConnectionResponseDto, AiDeploymentResponseDto, AiGenerationListQueryDto,
        AiGenerationResponseDto, AiLogContextDto, AiModelsResponseDto, AiSettingResponseDto,
        AnalyzeLogsDto, AnalyzeLogsResponseDto, CreateAiSettingDto, DeployAiGenerationDto,
        DiscoverAiModelsDto, GenerateComposeDto, ReviewAiGenerationDto, TestAiConnectionDto,
        UpdateAiSettingDto,
    },
    core::middleware::{permission::RequirePermission, validator::ValidatedJson},
    services::ai::{AiLogContext, AiService},
};

type ApiError = (StatusCode, String);

pub struct AiController {
    service: Arc<AiService>,
}

#[controller("/api/ai")]
impl AiController {
    fn new(service: Arc<AiService>) -> Self {
        Self { service }
    }

    #[get("/settings")]
    async fn list_settings(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
    ) -> Result<Json<Vec<AiSettingResponseDto>>, ApiError> {
        self.service
            .list_settings(claims.user.group_id)
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }

    #[get("/settings/{id}")]
    async fn get_setting(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<AiSettingResponseDto>, ApiError> {
        self.service
            .get_setting(id, claims.user.group_id)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_error)
    }

    #[post("/settings")]
    async fn create_setting(
        &self,
        RequirePermission(claims, _): RequirePermission<Organization, CanWrite>,
        ValidatedJson(body): ValidatedJson<CreateAiSettingDto>,
    ) -> Result<(StatusCode, Json<AiSettingResponseDto>), ApiError> {
        self.service
            .create_setting(claims.user.group_id, body.into())
            .await
            .map(Into::into)
            .map(Json)
            .map(|body| (StatusCode::CREATED, body))
            .map_err(map_error)
    }

    #[put("/settings/{id}")]
    async fn update_setting(
        &self,
        RequirePermission(claims, _): RequirePermission<Organization, CanWrite>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<UpdateAiSettingDto>,
    ) -> Result<Json<AiSettingResponseDto>, ApiError> {
        self.service
            .update_setting(id, claims.user.group_id, body.into())
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_error)
    }

    #[delete("/settings/{id}")]
    async fn delete_setting(
        &self,
        RequirePermission(claims, _): RequirePermission<Organization, CanWrite>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        match self
            .service
            .delete_setting(id, claims.user.group_id)
            .await
            .map_err(map_error)?
        {
            true => Ok(StatusCode::NO_CONTENT),
            false => Err((StatusCode::NOT_FOUND, "AI setting not found".into())),
        }
    }

    #[get("/settings/{id}/models")]
    async fn setting_models(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<AiModelsResponseDto>, ApiError> {
        self.service
            .discover_setting_models(id, claims.user.group_id)
            .await
            .map(|models| Json(AiModelsResponseDto { models }))
            .map_err(map_error)
    }

    #[post("/settings/{id}/test")]
    async fn test_setting(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<AiConnectionResponseDto>, ApiError> {
        self.service
            .test_setting(id, claims.user.group_id)
            .await
            .map(|()| {
                Json(AiConnectionResponseDto {
                    success: true,
                    message: "connection successful".into(),
                })
            })
            .map_err(map_error)
    }

    #[post("/models/discover")]
    async fn discover_models(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        ValidatedJson(body): ValidatedJson<DiscoverAiModelsDto>,
    ) -> Result<Json<AiModelsResponseDto>, ApiError> {
        self.service
            .discover_models(body.api_url, body.api_key)
            .await
            .map(|models| Json(AiModelsResponseDto { models }))
            .map_err(map_error)
    }

    #[post("/connection/test")]
    async fn test_connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<Application, CanRead>,
        ValidatedJson(body): ValidatedJson<TestAiConnectionDto>,
    ) -> Result<Json<AiConnectionResponseDto>, ApiError> {
        self.service
            .test_connection(body.api_url, body.api_key, body.model)
            .await
            .map(|()| {
                Json(AiConnectionResponseDto {
                    success: true,
                    message: "connection successful".into(),
                })
            })
            .map_err(map_error)
    }

    #[get("/generations")]
    async fn list_generations(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
        Query(query): Query<AiGenerationListQueryDto>,
    ) -> Result<Json<Vec<AiGenerationResponseDto>>, ApiError> {
        self.service
            .list_generations(claims.user.group_id, query.limit.unwrap_or(50))
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }

    #[post("/generations")]
    async fn generate(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanCreate>,
        ValidatedJson(body): ValidatedJson<GenerateComposeDto>,
    ) -> Result<(StatusCode, Json<AiGenerationResponseDto>), ApiError> {
        self.service
            .generate(
                body.ai_setting_id,
                claims.user.group_id,
                claims.user.user_id,
                body.request,
            )
            .await
            .map(Into::into)
            .map(Json)
            .map(|body| (StatusCode::CREATED, body))
            .map_err(map_error)
    }

    #[get("/generations/{id}")]
    async fn get_generation(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
        Path(id): Path<i64>,
    ) -> Result<Json<AiGenerationResponseDto>, ApiError> {
        self.service
            .get_generation(id, claims.user.group_id)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_error)
    }

    #[put("/generations/{id}/review")]
    async fn review_generation(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanCreate>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<ReviewAiGenerationDto>,
    ) -> Result<Json<AiGenerationResponseDto>, ApiError> {
        self.service
            .review_generation(id, claims.user.group_id, body.output)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_error)
    }

    #[post("/generations/{id}/deploy")]
    async fn deploy_generation(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanDeploy>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<DeployAiGenerationDto>,
    ) -> Result<(StatusCode, Json<AiDeploymentResponseDto>), ApiError> {
        self.service
            .deploy_generation(id, claims.user.group_id, body.into())
            .await
            .map(|result| {
                Json(AiDeploymentResponseDto {
                    generation: result.generation.into(),
                    compose_id: result.compose_id,
                    deployment_id: result.deployment_id,
                })
            })
            .map(|body| (StatusCode::ACCEPTED, body))
            .map_err(map_error)
    }

    #[post("/logs/analyze")]
    async fn analyze_logs(
        &self,
        RequirePermission(claims, _): RequirePermission<Application, CanRead>,
        ValidatedJson(body): ValidatedJson<AnalyzeLogsDto>,
    ) -> Result<Json<AnalyzeLogsResponseDto>, ApiError> {
        let context = match body.context {
            AiLogContextDto::Build => AiLogContext::Build,
            AiLogContextDto::Runtime => AiLogContext::Runtime,
        };
        self.service
            .analyze_logs(body.ai_setting_id, claims.user.group_id, context, body.logs)
            .await
            .map(|analysis| Json(AnalyzeLogsResponseDto { analysis }))
            .map_err(map_error)
    }
}

fn map_error(error: String) -> ApiError {
    let lower = error.to_ascii_lowercase();
    let status = if lower.contains("not found") {
        StatusCode::NOT_FOUND
    } else if lower.contains("already") || lower.contains("concurrently") {
        StatusCode::CONFLICT
    } else if lower.contains("does not belong") || lower.contains("authenticated organization") {
        StatusCode::FORBIDDEN
    } else if lower.contains("database") || lower.contains("sqlite") {
        tracing::error!(error = %error, "AI backend database operation failed");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database operation failed".into(),
        );
    } else {
        StatusCode::BAD_REQUEST
    };
    (status, error)
}
