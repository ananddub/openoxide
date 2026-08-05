use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AiSettingInput {
    pub name: String,
    pub api_url: String,
    pub api_key: String,
    pub model: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct AiSettingUpdate {
    pub name: Option<String>,
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiSettingView {
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

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct AiEnvironmentVariable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct AiDomain {
    pub host: String,
    pub port: i64,
    pub service_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct AiGeneratedFile {
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct AiComposeSuggestion {
    pub id: String,
    pub name: String,
    pub short_description: String,
    pub description: String,
    pub docker_compose: String,
    #[serde(default)]
    pub env_variables: Vec<AiEnvironmentVariable>,
    #[serde(default)]
    pub domains: Vec<AiDomain>,
    #[serde(default)]
    pub config_files: Vec<AiGeneratedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, poem_openapi::Object)]
pub struct AiGenerationOutput {
    pub suggestions: Vec<AiComposeSuggestion>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiGenerationView {
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

#[derive(Debug, Clone)]
pub struct AiDeployInput {
    pub suggestion_id: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub deploy_now: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AiDeploymentResult {
    pub generation: AiGenerationView,
    pub compose_id: i64,
    pub deployment_id: Option<i64>,
}

#[derive(Debug, Clone, Copy)]
pub enum AiLogContext {
    Build,
    Runtime,
}

impl AiLogContext {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Build => "build/deployment",
            Self::Runtime => "runtime/container",
        }
    }
}
