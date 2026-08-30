use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    background_policies::{BackgroundPoliciesGetRow, BackgroundPoliciesUpdateParams},
    models::Settings,
    settings::{SettingsCreateParams, SettingsUpdateParams},
};

pub struct SettingsService {
    queries: Arc<FlashQueries>,
}

fn norm(value: Option<String>) -> Option<String> {
    value.filter(|v| !v.is_empty())
}

#[singleton]
impl SettingsService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get(&self) -> sqlx::Result<SettingsResponseDto> {
        let setting = self.current().await?;
        let policy = self.queries.background_policies_get().await?;
        Ok(response(setting, &policy))
    }

    pub async fn update(&self, input: UpdateSettingsDto) -> sqlx::Result<SettingsResponseDto> {
        let current = self.current().await?;
        let setting = Settings {
            id: current.id,
            server_ip: input.server_ip.or(current.server_ip),
            certificate_type: input
                .certificate_type
                .map(|value| value.as_str().to_owned())
                .unwrap_or(current.certificate_type),
            custom_cert_resolver: input.custom_cert_resolver.or(current.custom_cert_resolver),
            https: input.https.map(i64::from).unwrap_or(current.https),
            host: input.host.or(current.host),
            lets_encrypt_email: input.lets_encrypt_email.or(current.lets_encrypt_email),
            enable_docker_cleanup: input
                .enable_docker_cleanup
                .map(i64::from)
                .unwrap_or(current.enable_docker_cleanup),
            log_cleanup_cron: input.log_cleanup_cron.or(current.log_cleanup_cron),
            metrics_config: input.metrics_config.unwrap_or(current.metrics_config),
            created_at: current.created_at,
            updated_at: chrono::Utc::now().timestamp(),
        };
        validate(&setting)?;
        self.queries
            .settings_update(&SettingsUpdateParams {
                server_ip: setting.server_ip.clone().unwrap_or_default(),
                certificate_type: setting.certificate_type.clone(),
                custom_cert_resolver: setting.custom_cert_resolver.clone().unwrap_or_default(),
                https: setting.https,
                host: setting.host.clone().unwrap_or_default(),
                lets_encrypt_email: setting.lets_encrypt_email.clone().unwrap_or_default(),
                enable_docker_cleanup: setting.enable_docker_cleanup,
                log_cleanup_cron: setting.log_cleanup_cron.clone().unwrap_or_default(),
                metrics_config: setting.metrics_config.clone(),
                created_at: setting.created_at,
                updated_at: setting.updated_at,
                id: setting.id,
            })
            .await?;
        let mut policy = self.queries.background_policies_get().await?;
        if let Some(value) = input.panel_backup_cron {
            policy.panel_backup_cron = value;
        }
        if let Some(value) = input.log_retention_days {
            policy.log_retention_days = value;
        }
        if let Some(value) = input.panel_backup_enabled {
            policy.panel_backup_enabled = i64::from(value);
        }
        if let Some(value) = input.log_cleanup_enabled {
            policy.log_cleanup_enabled = i64::from(value);
        }
        validate_policy(&policy)?;
        self.queries
            .background_policies_update(&BackgroundPoliciesUpdateParams {
                panel_backup_cron: policy.panel_backup_cron.clone(),
                log_cleanup_cron: policy.log_cleanup_cron.clone(),
                log_retention_days: policy.log_retention_days,
                panel_backup_enabled: policy.panel_backup_enabled,
                log_cleanup_enabled: policy.log_cleanup_enabled,
            })
            .await?;
        Ok(response(setting, &policy))
    }

    async fn current(&self) -> sqlx::Result<Settings> {
        if let Ok(row) = self.queries.get_settings().await {
            return Ok(normalize(row));
        }
        let now = chrono::Utc::now().timestamp();
        let _ = self
            .queries
            .settings_create(&SettingsCreateParams {
                server_ip: String::new(),
                certificate_type: "NONE".into(),
                custom_cert_resolver: String::new(),
                https: 0,
                host: String::new(),
                lets_encrypt_email: String::new(),
                enable_docker_cleanup: 1,
                log_cleanup_cron: "0 0 * * *".into(),
                metrics_config: "{}".into(),
                created_at: now,
                updated_at: now,
            })
            .await?;
        let row = self.queries.get_settings().await?;
        Ok(normalize(row))
    }
}

fn normalize(row: Settings) -> Settings {
    Settings {
        server_ip: norm(row.server_ip),
        custom_cert_resolver: norm(row.custom_cert_resolver),
        host: norm(row.host),
        lets_encrypt_email: norm(row.lets_encrypt_email),
        log_cleanup_cron: norm(row.log_cleanup_cron),
        ..row
    }
}

fn validate(setting: &Settings) -> sqlx::Result<()> {
    if let Some(ip) = setting.server_ip.as_deref()
        && ip.parse::<std::net::IpAddr>().is_err()
    {
        return Err(sqlx::Error::Protocol(
            "server_ip must be a valid IP address".into(),
        ));
    }
    if let Some(host) = setting.host.as_deref()
        && (host.trim().is_empty() || host.chars().any(char::is_whitespace))
    {
        return Err(sqlx::Error::Protocol(
            "host must not contain whitespace".into(),
        ));
    }
    if setting.certificate_type == "LETSENCRYPT"
        && setting
            .lets_encrypt_email
            .as_deref()
            .is_none_or(|value| !value.contains('@'))
    {
        return Err(sqlx::Error::Protocol(
            "lets_encrypt_email is required for LETSENCRYPT".into(),
        ));
    }
    if setting.certificate_type == "CUSTOM"
        && setting
            .custom_cert_resolver
            .as_deref()
            .is_none_or(|value| value.trim().is_empty())
    {
        return Err(sqlx::Error::Protocol(
            "custom_cert_resolver is required for CUSTOM".into(),
        ));
    }
    if let Some(cron) = setting.log_cleanup_cron.as_deref()
        && cron.split_whitespace().count() != 5
    {
        return Err(sqlx::Error::Protocol(
            "log_cleanup_cron must contain five cron fields".into(),
        ));
    }
    if !setting.metrics_config.trim().is_empty() {
        serde_json::from_str::<serde_json::Value>(&setting.metrics_config).map_err(|error| {
            sqlx::Error::Protocol(format!("invalid metrics_config JSON: {error}"))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::flash_gen::models::Settings;

    fn setting() -> Settings {
        Settings {
            id: 1,
            server_ip: Some("127.0.0.1".into()),
            certificate_type: "NONE".into(),
            custom_cert_resolver: None,
            https: 0,
            host: Some("panel.example.com".into()),
            lets_encrypt_email: None,
            enable_docker_cleanup: 1,
            log_cleanup_cron: Some("0 0 * * *".into()),
            metrics_config: "{}".into(),
            created_at: 1,
            updated_at: 1,
        }
    }

    #[test]
    fn validates_product_settings_contract() {
        assert!(validate(&setting()).is_ok());
        let mut invalid_ip = setting();
        invalid_ip.server_ip = Some("not-ip".into());
        assert!(validate(&invalid_ip).is_err());
        let mut letsencrypt = setting();
        letsencrypt.certificate_type = "LETSENCRYPT".into();
        assert!(validate(&letsencrypt).is_err());
        let mut invalid_json = setting();
        invalid_json.metrics_config = "{".into();
        assert!(validate(&invalid_json).is_err());
    }
}

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

fn response(setting: Settings, policy: &BackgroundPoliciesGetRow) -> SettingsResponseDto {
    let mut dto: SettingsResponseDto = setting.into();
    dto.panel_backup_cron = policy.panel_backup_cron.clone();
    dto.log_cleanup_cron = Some(policy.log_cleanup_cron.clone());
    dto.log_retention_days = policy.log_retention_days;
    dto.panel_backup_enabled = policy.panel_backup_enabled != 0;
    dto.log_cleanup_enabled = policy.log_cleanup_enabled != 0;
    dto
}

fn validate_policy(policy: &BackgroundPoliciesGetRow) -> sqlx::Result<()> {
    if policy.panel_backup_cron.split_whitespace().count() != 5
        || policy.log_cleanup_cron.split_whitespace().count() != 5
    {
        return Err(sqlx::Error::Protocol(
            "background cron expressions must contain five fields".into(),
        ));
    }
    if !(1..=3650).contains(&policy.log_retention_days) {
        return Err(sqlx::Error::Protocol(
            "log_retention_days must be between 1 and 3650".into(),
        ));
    }
    Ok(())
}
