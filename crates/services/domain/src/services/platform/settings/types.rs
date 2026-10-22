use serde::{Deserialize, Serialize};

use crate::flash_gen::models::Settings;

#[derive(Debug, Clone, Serialize)]
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
    pub panel_backup_cron: String,
    pub log_retention_days: i64,
    pub panel_backup_enabled: bool,
    pub log_cleanup_enabled: bool,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
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
    pub panel_backup_cron: Option<String>,
    pub log_retention_days: Option<i64>,
    pub panel_backup_enabled: Option<bool>,
    pub log_cleanup_enabled: Option<bool>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<Settings> for SettingsResponseDto {
    fn from(value: Settings) -> Self {
        Self {
            id: value.id,
            server_ip: value.server_ip,
            certificate_type: value.certificate_type,
            custom_cert_resolver: value.custom_cert_resolver,
            https: value.https != 0,
            host: value.host,
            lets_encrypt_email: value.lets_encrypt_email,
            enable_docker_cleanup: value.enable_docker_cleanup != 0,
            log_cleanup_cron: value.log_cleanup_cron,
            metrics_config: value.metrics_config,
            panel_backup_cron: "0 3 * * *".into(),
            log_retention_days: 30,
            panel_backup_enabled: true,
            log_cleanup_enabled: true,
            updated_at: value.updated_at,
        }
    }
}
