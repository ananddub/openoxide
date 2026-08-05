use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CdnProvider {
    Cloudflare,
    Fastly,
    Bunny,
}
impl CdnProvider {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cloudflare => "cloudflare",
            Self::Fastly => "fastly",
            Self::Bunny => "bunny",
        }
    }
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct DomainDiagnosticDto {
    #[validate(length(min = 1, max = 255))]
    pub host: String,
    pub port: Option<u16>,
    pub https: bool,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct DomainDiagnosticResponseDto {
    pub host: String,
    pub resolved_addresses: Vec<String>,
    pub dns_ok: bool,
    pub tcp_ok: bool,
    pub http_ok: bool,
    pub http_status: Option<u16>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RootNetworkDto {
    pub server_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RootNetworkStatusDto {
    pub name: String,
    pub exists: bool,
    pub healthy: bool,
    pub repaired: bool,
    pub driver: Option<String>,
    pub scope: Option<String>,
    pub attachable: Option<bool>,
    pub connected_resources: i64,
    pub issue: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CdnPurgeDto {
    pub provider: CdnProvider,
    #[validate(length(min = 1, max = 512))]
    pub api_token: String,
    pub zone_id: Option<String>,
    pub service_id: Option<String>,
    pub pull_zone_id: Option<i64>,
    pub purge_all: bool,
    pub urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct CdnPurgeResponseDto {
    pub provider: String,
    pub success: bool,
    pub status: u16,
    pub request_id: Option<String>,
    pub message: String,
}
