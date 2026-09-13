use os::string_enum;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum VaultProviderType {
        default = Hashicorp;

        Hashicorp => "HASHICORP",
        Infisical => "INFISICAL",
        Doppler => "DOPPLER",
        Aws => "AWS",
        AwsParameterStore => "AWS_PARAMETER_STORE",
        Scaleway => "SCALEWAY",
        Azure => "AZURE",
    }
}

#[derive(Debug, Serialize)]
pub struct VaultProviderDto {
    pub id: i64,
    pub name: String,
    pub provider_type: VaultProviderType,
    pub api_url: String,
    pub auth_token: String,
    pub namespace: Option<String>,
    pub config_json: Option<String>,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub assignments: Vec<VaultProviderAssignmentDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultProviderAssignmentDto {
    pub project_id: i64,
    pub environment_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVaultProviderDto {
    pub name: String,
    pub provider_type: VaultProviderType,
    pub api_url: String,
    pub auth_token: String,
    pub namespace: Option<String>,
    pub config_json: Option<String>,
    #[serde(default)]
    pub assignments: Vec<VaultProviderAssignmentDto>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVaultProviderDto {
    pub name: Option<String>,
    pub api_url: Option<String>,
    pub auth_token: Option<String>,
    pub namespace: Option<String>,
    pub config_json: Option<String>,
    pub assignments: Option<Vec<VaultProviderAssignmentDto>>,
}

#[derive(Debug, Serialize)]
pub struct VaultTestResultDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct VaultSecretListDto {
    pub secrets: Vec<String>,
}

pub const VAULT_MASK_TOKEN: &str = "••••••••";

#[derive(Debug, Error)]
pub enum VaultServiceError {
    #[error("Vault provider not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Vault provider failure: {0}")]
    ProviderError(String),
}

pub fn field(raw: Option<&str>, key: &str) -> Option<String> {
    raw.and_then(|value| serde_json::from_str::<Value>(value).ok())
        .and_then(|value| {
            value[key]
                .as_str()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_string)
        })
}

pub fn encode_path(path: &str) -> String {
    path.split('/')
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect::<Vec<_>>()
        .join("/")
}
