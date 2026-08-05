use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplicationMiddlewareType {
    Compress,
    Headers,
    RateLimit,
    IpAllowlist,
}
impl ApplicationMiddlewareType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compress => "COMPRESS",
            Self::Headers => "HEADERS",
            Self::RateLimit => "RATE_LIMIT",
            Self::IpAllowlist => "IP_ALLOWLIST",
        }
    }
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, poem_openapi::Object)]
pub struct MiddlewareHeaderDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 4_096))]
    pub value: String,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, poem_openapi::Object)]
pub struct UpsertApplicationMiddlewareDto {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub middleware_type: ApplicationMiddlewareType,
    pub enabled: bool,
    pub headers: Option<Vec<MiddlewareHeaderDto>>,
    pub average: Option<i64>,
    pub burst: Option<i64>,
    pub source_ranges: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationMiddlewareResponseDto {
    pub id: i64,
    pub application_id: i64,
    pub name: String,
    pub middleware_type: String,
    pub enabled: bool,
    pub headers: Vec<MiddlewareHeaderDto>,
    pub average: Option<i64>,
    pub burst: Option<i64>,
    pub source_ranges: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
