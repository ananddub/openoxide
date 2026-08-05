use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikFileNodeDto {
    pub name: String,
    pub relative_path: String,
    pub size: u64,
    pub is_readonly: bool,
    pub modified_at: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikFileTreeNodeDto {
    pub name: String,
    pub relative_path: String,
    pub node_type: String,
    pub size: u64,
    pub is_readonly: bool,
    pub modified_at: u64,
    pub children: Vec<TraefikFileTreeNodeDto>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikFileQueryDto {
    pub server_id: Option<i64>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikWriteFileDto {
    pub server_id: Option<i64>,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikFileContentDto {
    pub path: String,
    pub content: String,
    pub is_readonly: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikHealthResponseDto {
    pub is_healthy: bool,
    pub rawdata_status: String,
    pub configuration_errors: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikRequestsStatusDto {
    pub is_active: bool,
    pub log_path: String,
    pub cron_expression: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikToggleRequestsDto {
    pub server_id: Option<i64>,
    pub enable: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikLogEntryDto {
    pub timestamp: String,
    pub client_ip: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub duration_ms: f64,
    pub service_name: String,
    pub router_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikStatsLogsQueryDto {
    pub server_id: Option<i64>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Object)]
pub struct TraefikStatsLogsResponseDto {
    pub items: Vec<TraefikLogEntryDto>,
    pub total_count: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct TraefikVersionDto {
    pub server_id: Option<i64>,
    pub current_image: String,
    pub desired_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct UpdateTraefikVersionDto {
    pub server_id: Option<i64>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct StructuredMiddlewareDto {
    pub kind: StructuredMiddlewareKind,
    pub name: String,
    pub values: std::collections::BTreeMap<String, String>,
    pub list: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StructuredMiddlewareKind {
    StripPrefix,
    AddPrefix,
    RedirectScheme,
    Compress,
    RateLimit,
    IpAllowList,
    RequestHeaders,
    ResponseHeaders,
}

#[derive(Debug, Clone, Serialize, Deserialize, Object)]
pub struct StructuredMiddlewareResponseDto {
    pub reference: String,
    pub labels: std::collections::BTreeMap<String, String>,
}
