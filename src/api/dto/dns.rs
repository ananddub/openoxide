use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct DnsProviderDto {
    pub id: i64,
    pub name: String,
    pub provider_type: String,
    pub credentials_json: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct CreateDnsProviderDto {
    pub name: String,
    pub provider_type: String,
    pub credentials_json: String,
}

#[derive(Debug, Deserialize, Validate, poem_openapi::Object)]
pub struct UpdateDnsProviderDto {
    pub name: Option<String>,
    pub provider_type: Option<String>,
    pub credentials_json: Option<String>,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct DnsTestResultDto {
    pub success: bool,
    pub message: String,
}
