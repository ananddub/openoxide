use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::settings::{SettingsResponseDto, UpdateSettingsDto},
    db::models::settings::Setting,
    repository::SettingRepository,
};

pub struct SettingsService {
    repository: Arc<SettingRepository>,
    policies: Arc<crate::repository::BackgroundPolicyRepository>,
}

#[singleton]
impl SettingsService {
    fn new(
        repository: Arc<SettingRepository>,
        policies: Arc<crate::repository::BackgroundPolicyRepository>,
    ) -> Self {
        Self {
            repository,
            policies,
        }
    }

    pub async fn get(&self) -> sqlx::Result<SettingsResponseDto> {
        let setting = self.current().await?;
        let policy = self.policies.get().await?;
        Ok(response(setting, policy))
    }

    pub async fn update(&self, input: UpdateSettingsDto) -> sqlx::Result<SettingsResponseDto> {
        let current = self.current().await?;
        let setting = Setting {
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
        self.repository
            .update(current.id.unwrap_or_default(), &setting)
            .await?;
        let mut policy = self.policies.get().await?;
        if let Some(value) = input.panel_backup_cron {
            policy.panel_backup_cron = value;
        }
        if let Some(value) = input.log_retention_days {
            policy.log_retention_days = value;
        }
        if let Some(value) = input.panel_backup_enabled {
            policy.panel_backup_enabled = value;
        }
        if let Some(value) = input.log_cleanup_enabled {
            policy.log_cleanup_enabled = value;
        }
        validate_policy(&policy)?;
        self.policies.update(&policy).await?;
        Ok(response(setting, policy))
    }

    async fn current(&self) -> sqlx::Result<Setting> {
        if let Some(setting) = self.repository.get_all().await?.into_iter().next() {
            return Ok(setting);
        }
        let now = chrono::Utc::now().timestamp();
        let setting = Setting {
            id: None,
            server_ip: None,
            certificate_type: "NONE".into(),
            custom_cert_resolver: None,
            https: 0,
            host: None,
            lets_encrypt_email: None,
            enable_docker_cleanup: 1,
            log_cleanup_cron: Some("0 0 * * *".into()),
            metrics_config: "{}".into(),
            created_at: now,
            updated_at: now,
        };
        let id = self.repository.create(&setting).await?;
        Ok(Setting {
            id: Some(id),
            ..setting
        })
    }
}

fn validate(setting: &Setting) -> sqlx::Result<()> {
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
    use crate::db::models::settings::Setting;

    fn setting() -> Setting {
        Setting {
            id: Some(1),
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

impl From<Setting> for SettingsResponseDto {
    fn from(value: Setting) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
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

fn response(
    setting: Setting,
    policy: crate::db::repository::background_policies::BackgroundPolicy,
) -> SettingsResponseDto {
    let mut dto: SettingsResponseDto = setting.into();
    dto.panel_backup_cron = policy.panel_backup_cron;
    dto.log_cleanup_cron = Some(policy.log_cleanup_cron);
    dto.log_retention_days = policy.log_retention_days;
    dto.panel_backup_enabled = policy.panel_backup_enabled;
    dto.log_cleanup_enabled = policy.log_cleanup_enabled;
    dto
}

fn validate_policy(
    policy: &crate::db::repository::background_policies::BackgroundPolicy,
) -> sqlx::Result<()> {
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
