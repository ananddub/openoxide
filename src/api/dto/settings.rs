use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Object)]
pub struct SettingsResponseDto {
    pub id: i64,
    pub server_ip: Option<String>,
    pub certificate_type: String,
    pub custom_cert_resolver: Option<String>,
    pub https: bool,
    pub host: Option<String>,
    pub lets_encrypt_email: Option<String>,
    pub enable_docker_cleanup: bool,
    pub log_cleanup_cron: Option<String>,
    pub metrics_config: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize, Object, Validate)]
pub struct UpdateSettingsDto {
    pub server_ip: Option<String>,
    pub certificate_type: Option<SettingsCertificateTypeDto>,
    pub custom_cert_resolver: Option<String>,
    pub https: Option<bool>,
    pub host: Option<String>,
    pub lets_encrypt_email: Option<String>,
    pub enable_docker_cleanup: Option<bool>,
    pub log_cleanup_cron: Option<String>,
    pub metrics_config: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettingsCertificateTypeDto {
    None,
    Letsencrypt,
    Custom,
}

impl SettingsCertificateTypeDto {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Letsencrypt => "LETSENCRYPT",
            Self::Custom => "CUSTOM",
        }
    }
}
