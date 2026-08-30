use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use serde::Deserialize;

use crate::api::connect::generated::openoxide::api::v1::{
    AiService, AiServiceExt, Empty, OperationResult, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::ai::{
    AiDeployInput, AiGenerationOutput, AiLogContext, AiSettingInput, AiSettingUpdate,
    AiService as AppAiService,
};

pub struct AiController {
    service: Arc<AppAiService>,
}

#[singleton]
impl AiController {
    pub fn new(service: Arc<AppAiService>) -> Self {
        Self { service }
    }
}

fn map_error(error: String) -> ConnectError {
    let lower = error.to_ascii_lowercase();
    if lower.contains("not found") {
        ConnectError::not_found(error)
    } else if lower.contains("already") || lower.contains("concurrently") {
        ConnectError::new(ErrorCode::AlreadyExists, error)
    } else if lower.contains("does not belong") || lower.contains("authenticated organization") {
        ConnectError::new(ErrorCode::PermissionDenied, error)
    } else if lower.contains("database") || lower.contains("sqlite") {
        tracing::error!(error = %error, "AI backend database operation failed");
        ConnectError::new(ErrorCode::Internal, "database operation failed")
    } else {
        ConnectError::invalid_argument(error)
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl AiService for AiController {
    #[permission(Application, CanRead)]
    async fn list_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let items = self
            .service
            .list_settings(organization_id)
            .await
            .map_err(map_error)?;
        let items = items
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: dto.id,
                    kind: "ai_setting".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn get_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let setting = self
            .service
            .get_setting(request.id, organization_id)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&setting).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: setting.id,
            kind: "ai_setting".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Organization, CanWrite)]
    async fn create_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let dto: CreateAiSettingDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let setting = self
            .service
            .create_setting(organization_id, dto.into())
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&setting).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: setting.id,
            kind: "ai_setting".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Organization, CanWrite)]
    async fn update_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let dto: UpdateAiSettingDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let setting = self
            .service
            .update_setting(request.id, organization_id, dto.into())
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&setting).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: setting.id,
            kind: "ai_setting".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Organization, CanWrite)]
    async fn delete_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let deleted = self
            .service
            .delete_setting(request.id, organization_id)
            .await
            .map_err(map_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("AI setting not found"))
        }
    }

    #[permission(Application, CanRead)]
    async fn list_models<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let models = self
            .service
            .discover_setting_models(request.id, organization_id)
            .await
            .map_err(map_error)?;
        let items = models
            .into_iter()
            .map(|m| {
                let payload = serde_json::to_string(&m).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "ai_model".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn test_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        self.service
            .test_setting(request.id, organization_id)
            .await
            .map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "test_settings".into(),
            message: Some("connection successful".into()),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn discover_models<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let dto: DiscoverAiModelsDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let models = self
            .service
            .discover_models(dto.api_url, dto.api_key)
            .await
            .map_err(map_error)?;
        let items = models
            .into_iter()
            .map(|m| {
                let payload = serde_json::to_string(&m).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "ai_model".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: TestAiConnectionDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .test_connection(dto.api_url, dto.api_key, dto.model)
            .await
            .map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "test_connection".into(),
            message: Some("connection successful".into()),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn list_generations<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let limit = request.limit.unwrap_or(50);
        let items = self
            .service
            .list_generations(organization_id, limit)
            .await
            .map_err(map_error)?;
        let items = items
            .into_iter()
            .map(|dto| {
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                ResourceResponse {
                    id: dto.id,
                    kind: "ai_generation".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Application, CanRead)]
    async fn get_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let generation = self
            .service
            .get_generation(request.id, organization_id)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&generation).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: generation.id,
            kind: "ai_generation".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let auth = Self::extract_auth(&ctx).await?;
        let organization_id = auth.organization_id;
        let dto: GenerateComposeDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let generation = self
            .service
            .generate(dto.ai_setting_id, organization_id, auth.user_id(), dto.request)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&generation).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: generation.id,
            kind: "ai_generation".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn review_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let dto: ReviewAiGenerationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .review_generation(request.id, organization_id, dto.output)
            .await
            .map_err(map_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "review_generation".into(),
            ..Default::default()
        })
    }

    #[permission(Application, CanDeploy)]
    async fn deploy_generation<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let dto: DeployAiGenerationDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let result = self
            .service
            .deploy_generation(request.id, organization_id, dto.into())
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&result).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "deploy_generation".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn analyze_logs<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let organization_id = request.organization_id.unwrap_or(0);
        let dto: AnalyzeLogsDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let context = match dto.context {
            AiLogContextDto::Build => AiLogContext::Build,
            AiLogContextDto::Runtime => AiLogContext::Runtime,
        };
        let analysis = self
            .service
            .analyze_logs(dto.ai_setting_id, organization_id, context, dto.logs)
            .await
            .map_err(map_error)?;
        let payload = serde_json::to_string(&analysis).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "ai_log_analysis".into(),
            payload,
            ..Default::default()
        })
    }
}

// ── AI payload DTOs ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CreateAiSettingDto {
    name: String,
    api_url: String,
    api_key: String,
    model: String,
    #[serde(default = "default_true")]
    is_enabled: bool,
}

impl From<CreateAiSettingDto> for AiSettingInput {
    fn from(value: CreateAiSettingDto) -> Self {
        Self {
            name: value.name,
            api_url: value.api_url,
            api_key: value.api_key,
            model: value.model,
            is_enabled: value.is_enabled,
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdateAiSettingDto {
    name: Option<String>,
    api_url: Option<String>,
    api_key: Option<String>,
    model: Option<String>,
    is_enabled: Option<bool>,
}

impl From<UpdateAiSettingDto> for AiSettingUpdate {
    fn from(value: UpdateAiSettingDto) -> Self {
        Self {
            name: value.name,
            api_url: value.api_url,
            api_key: value.api_key,
            model: value.model,
            is_enabled: value.is_enabled,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DiscoverAiModelsDto {
    api_url: String,
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct TestAiConnectionDto {
    api_url: String,
    api_key: String,
    model: String,
}

#[derive(Debug, Deserialize)]
struct GenerateComposeDto {
    ai_setting_id: i64,
    request: String,
}

#[derive(Debug, Deserialize)]
struct ReviewAiGenerationDto {
    output: AiGenerationOutput,
}

#[derive(Debug, Deserialize)]
struct DeployAiGenerationDto {
    suggestion_id: String,
    environment_id: i64,
    server_id: Option<i64>,
    #[serde(default)]
    deploy_now: bool,
}

impl From<DeployAiGenerationDto> for AiDeployInput {
    fn from(value: DeployAiGenerationDto) -> Self {
        Self {
            suggestion_id: value.suggestion_id,
            environment_id: value.environment_id,
            server_id: value.server_id,
            deploy_now: value.deploy_now,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum AiLogContextDto {
    Build,
    Runtime,
}

#[derive(Debug, Deserialize)]
struct AnalyzeLogsDto {
    ai_setting_id: i64,
    context: AiLogContextDto,
    logs: String,
}

fn default_true() -> bool {
    true
}
