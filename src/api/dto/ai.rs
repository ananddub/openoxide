use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::ai::{
    AiDeployInput, AiGenerationOutput, AiGenerationView, AiSettingInput, AiSettingUpdate,
    AiSettingView,
};

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct CreateAiSettingDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(url)]
    pub api_url: String,
    pub api_key: String,
    #[validate(length(min = 1, max = 200))]
    pub model: String,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
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

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct UpdateAiSettingDto {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(url)]
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub model: Option<String>,
    pub is_enabled: Option<bool>,
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

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AiSettingResponseDto {
    pub id: i64,
    pub name: String,
    pub api_url: String,
    pub model: String,
    pub is_enabled: bool,
    pub provider: String,
    pub has_api_key: bool,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<AiSettingView> for AiSettingResponseDto {
    fn from(value: AiSettingView) -> Self {
        Self {
            id: value.id,
            name: value.name,
            api_url: value.api_url,
            model: value.model,
            is_enabled: value.is_enabled,
            provider: value.provider,
            has_api_key: value.has_api_key,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct DiscoverAiModelsDto {
    #[validate(url)]
    pub api_url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct TestAiConnectionDto {
    #[validate(url)]
    pub api_url: String,
    pub api_key: String,
    #[validate(length(min = 1, max = 200))]
    pub model: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AiModelsResponseDto {
    pub models: Vec<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AiConnectionResponseDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct GenerateComposeDto {
    pub ai_setting_id: i64,
    #[validate(length(min = 1, max = 8_000))]
    pub request: String,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct ReviewAiGenerationDto {
    pub output: AiGenerationOutput,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct DeployAiGenerationDto {
    #[validate(length(min = 1, max = 200))]
    pub suggestion_id: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    #[serde(default)]
    pub deploy_now: bool,
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

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AiGenerationResponseDto {
    pub id: i64,
    pub ai_setting_id: i64,
    pub organization_id: i64,
    pub created_by: i64,
    pub prompt: String,
    pub output: AiGenerationOutput,
    pub status: String,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<AiGenerationView> for AiGenerationResponseDto {
    fn from(value: AiGenerationView) -> Self {
        Self {
            id: value.id,
            ai_setting_id: value.ai_setting_id,
            organization_id: value.organization_id,
            created_by: value.created_by,
            prompt: value.prompt,
            output: value.output,
            status: value.status,
            compose_id: value.compose_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AiDeploymentResponseDto {
    pub generation: AiGenerationResponseDto,
    pub compose_id: i64,
    pub deployment_id: Option<i64>,
}

#[derive(Debug, Clone, Copy, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "snake_case")]
pub enum AiLogContextDto {
    Build,
    Runtime,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct AnalyzeLogsDto {
    pub ai_setting_id: i64,
    pub context: AiLogContextDto,
    #[validate(length(min = 1, max = 250_000))]
    pub logs: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct AnalyzeLogsResponseDto {
    pub analysis: String,
}

#[derive(Debug, Deserialize, Serialize, poem_openapi::Object)]
pub struct AiGenerationListQueryDto {
    pub limit: Option<i64>,
}

fn default_true() -> bool {
    true
}
