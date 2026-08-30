use connectrpc::{ConnectError, ErrorCode};
use serde::Deserialize;

use crate::services::platform::ai::{
    AiDeployInput, AiGenerationOutput, AiSettingInput, AiSettingUpdate,
};

pub fn map_error(error: String) -> ConnectError {
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

// ── AI payload DTOs ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateAiSettingDto {
    pub name: String,
    pub api_url: String,
    pub api_key: String,
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

#[derive(Debug, Deserialize)]
pub struct UpdateAiSettingDto {
    pub name: Option<String>,
    pub api_url: Option<String>,
    pub api_key: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct DiscoverAiModelsDto {
    pub api_url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct TestAiConnectionDto {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateComposeDto {
    pub ai_setting_id: i64,
    pub request: String,
}

#[derive(Debug, Deserialize)]
pub struct ReviewAiGenerationDto {
    pub output: AiGenerationOutput,
}

#[derive(Debug, Deserialize)]
pub struct DeployAiGenerationDto {
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

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiLogContextDto {
    Build,
    Runtime,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeLogsDto {
    pub ai_setting_id: i64,
    pub context: AiLogContextDto,
    pub logs: String,
}

pub fn default_true() -> bool {
    true
}
