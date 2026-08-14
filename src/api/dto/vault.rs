use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Object, ts_rs::TS)]
pub struct VaultProviderDto {
    pub id: i64,
    pub name: String,
    pub provider_type: String,
    pub api_url: String,
    pub auth_token: String,
    pub namespace: Option<String>,
    pub config_json: Option<String>,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct CreateVaultProviderDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub provider_type: String, // "HASHICORP", "INFISICAL", "DOPPLER", "AWS"
    pub api_url: String,
    pub auth_token: String,
    pub namespace: Option<String>,
    pub config_json: Option<String>,
}

#[derive(Debug, Deserialize, Validate, Object)]
pub struct UpdateVaultProviderDto {
    pub name: Option<String>,
    pub api_url: Option<String>,
    pub auth_token: Option<String>,
    pub namespace: Option<String>,
    pub config_json: Option<String>,
}

#[derive(Debug, Serialize, Object, ts_rs::TS)]
pub struct VaultTestResultDto {
    pub success: bool,
    pub message: String,
}
