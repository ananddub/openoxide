use std::sync::Arc;

use auto_di::singleton;

use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    background_policies::{BackgroundPoliciesGetRow, BackgroundPoliciesUpdateParams},
    models::Settings,
    settings::{SettingsCreateParams, SettingsUpdateParams},
};

use super::types::{SettingsResponseDto, UpdateSettingsDto};

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
        let policy = self.policy().await?;
        Ok(response(setting, &policy))
    }

    pub async fn update(&self, input: UpdateSettingsDto) -> sqlx::Result<SettingsResponseDto> {
        let saved = self.persist(input).await?;
        // Reflect the panel's own domain into Traefik so the dashboard routes
        // on its configured host (). Best-effort: a Traefik/edge
        // hiccup must not fail the settings save.
        if let Ok(traefik) =
            auto_di::resolve::<crate::services::networking::traefik::TraefikService>().await
            && let Err(error) = traefik
                .update_panel_domain(
                    saved.host.as_deref(),
                    saved.https,
                    &saved.certificate_type,
                    saved.custom_cert_resolver.as_deref(),
                )
                .await
        {
            tracing::warn!(%error, "could not write panel Traefik router");
        }
        Ok(saved)
    }

    /// The persistent half of [`Self::update`]. Split out because the Traefik
    /// reflection above resolves the panel's DI graph (Agent, database pool),
    /// which database-backed tests must not boot just to check a saved row.
    async fn persist(&self, input: UpdateSettingsDto) -> sqlx::Result<SettingsResponseDto> {
        let current = self.current().await?;
        let log_cleanup_cron = input.log_cleanup_cron.clone();
        let setting = Settings {
            id: current.id,
            server_ip: patch(current.server_ip, input.server_ip),
            certificate_type: input
                .certificate_type
                .map(|value| value.as_str().to_owned())
                .unwrap_or(current.certificate_type),
            custom_cert_resolver: patch(current.custom_cert_resolver, input.custom_cert_resolver),
            https: input.https.map(i64::from).unwrap_or(current.https),
            host: patch(current.host, input.host),
            lets_encrypt_email: patch(current.lets_encrypt_email, input.lets_encrypt_email),
            enable_docker_cleanup: input
                .enable_docker_cleanup
                .map(i64::from)
                .unwrap_or(current.enable_docker_cleanup),
            log_cleanup_cron: patch(current.log_cleanup_cron, log_cleanup_cron),
            metrics_config: input.metrics_config.unwrap_or(current.metrics_config),
            created_at: current.created_at,
            updated_at: chrono::Utc::now().timestamp(),
        };
        // Validate both halves before either write; otherwise a rejected
        // update still persists the settings row that validated fine.
        validate(&setting)?;
        let mut policy = self.policy().await?;
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
        // The background scheduler reads the policy row, not settings, so the
        // form's cleanup cron has to land there too.
        if let Some(value) = setting.log_cleanup_cron.clone() {
            policy.log_cleanup_cron = value;
        }
        validate_policy(&policy)?;
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
        self.queries
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

    /// Read the singleton background-policies row, seeding defaults when the
    /// row does not exist yet (fresh installs never insert it).
    async fn policy(&self) -> sqlx::Result<BackgroundPoliciesGetRow> {
        match self.queries.background_policies_get().await {
            Ok(row) => Ok(row),
            Err(sqlx::Error::RowNotFound) => {
                self.queries.background_policies_seed().await?;
                self.queries.background_policies_get().await
            }
            Err(error) => Err(error),
        }
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

/// PATCH semantics for optional text fields: absent keeps the current value,
/// blank (empty/whitespace) clears it, anything else replaces it.
fn patch(current: Option<String>, incoming: Option<String>) -> Option<String> {
    match incoming {
        None => current,
        Some(value) => {
            let value = value.trim().to_owned();
            if value.is_empty() { None } else { Some(value) }
        }
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

#[cfg(test)]
mod tests {
    use super::{SettingsService, UpdateSettingsDto, validate};
    use crate::flash_gen::models::Settings;
    use crate::services::platform::settings::SettingsCertificateTypeDto;
    use crate::test_support::fixture;

    async fn service(tag: &str) -> (SettingsService, sqlx::SqlitePool) {
        let (queries, pool) = fixture(tag).await;
        (SettingsService { queries }, pool)
    }

    fn empty_update() -> UpdateSettingsDto {
        UpdateSettingsDto {
            server_ip: None,
            certificate_type: None,
            custom_cert_resolver: None,
            https: None,
            host: None,
            lets_encrypt_email: None,
            enable_docker_cleanup: None,
            log_cleanup_cron: None,
            metrics_config: None,
            panel_backup_cron: None,
            log_retention_days: None,
            panel_backup_enabled: None,
            log_cleanup_enabled: None,
        }
    }

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

    #[tokio::test]
    async fn get_seeds_defaults_on_a_fresh_database() {
        let (service, _pool) = service("settings-seed").await;

        let first = service.get().await.unwrap();

        assert_eq!(first.certificate_type, "NONE");
        assert!(!first.https);
        assert!(first.enable_docker_cleanup);
        assert_eq!(first.host, None, "an unset host must normalize to None");
        assert_eq!(first.log_cleanup_cron.as_deref(), Some("0 0 * * *"));
        assert_eq!(first.panel_backup_cron, "0 3 * * *");
        assert_eq!(first.log_retention_days, 30);
        assert!(first.panel_backup_enabled);
        assert!(first.log_cleanup_enabled);

        let second = service.get().await.unwrap();
        assert_eq!(second.id, first.id, "settings must stay a singleton");
    }

    /// `persist` is what `update` writes before its best-effort Traefik
    /// reflection; the reflection needs the panel's live DI/Agent graph.
    #[tokio::test]
    async fn update_persists_settings_and_policy_flags() {
        let (service, _pool) = service("settings-update").await;

        let updated = service
            .persist(UpdateSettingsDto {
                server_ip: Some("10.0.0.5".into()),
                certificate_type: Some(SettingsCertificateTypeDto::Letsencrypt),
                lets_encrypt_email: Some("ops@example.com".into()),
                https: Some(true),
                host: Some("panel.example.com".into()),
                metrics_config: Some(r#"{"enabled":true}"#.into()),
                panel_backup_cron: Some("30 2 * * *".into()),
                log_retention_days: Some(7),
                panel_backup_enabled: Some(false),
                log_cleanup_enabled: Some(false),
                ..empty_update()
            })
            .await
            .unwrap();

        assert_eq!(updated.host.as_deref(), Some("panel.example.com"));
        assert!(updated.https);
        assert_eq!(updated.certificate_type, "LETSENCRYPT");
        assert_eq!(updated.panel_backup_cron, "30 2 * * *");
        assert_eq!(updated.log_retention_days, 7);
        assert!(!updated.panel_backup_enabled);
        assert!(!updated.log_cleanup_enabled);

        let read = service.get().await.unwrap();
        assert_eq!(read.server_ip.as_deref(), Some("10.0.0.5"));
        assert_eq!(read.lets_encrypt_email.as_deref(), Some("ops@example.com"));
        assert_eq!(read.metrics_config, r#"{"enabled":true}"#);
        assert_eq!(read.host.as_deref(), Some("panel.example.com"));
    }

    #[tokio::test]
    async fn blank_text_clears_and_absent_text_keeps() {
        let (service, _pool) = service("settings-patch").await;
        service
            .persist(UpdateSettingsDto {
                server_ip: Some("10.0.0.5".into()),
                host: Some("panel.example.com".into()),
                ..empty_update()
            })
            .await
            .unwrap();

        let patched = service
            .persist(UpdateSettingsDto {
                server_ip: None,
                host: Some("   ".into()),
                ..empty_update()
            })
            .await
            .unwrap();

        assert_eq!(
            patched.server_ip.as_deref(),
            Some("10.0.0.5"),
            "an absent server_ip must keep its value"
        );
        assert_eq!(patched.host, None, "a blank host must clear it");
    }

    #[tokio::test]
    async fn update_rejects_invalid_values_without_persisting() {
        let (service, _pool) = service("settings-reject").await;

        for update in [
            UpdateSettingsDto {
                server_ip: Some("not-an-ip".into()),
                ..empty_update()
            },
            UpdateSettingsDto {
                certificate_type: Some(SettingsCertificateTypeDto::Letsencrypt),
                ..empty_update()
            },
            UpdateSettingsDto {
                metrics_config: Some("{".into()),
                ..empty_update()
            },
            UpdateSettingsDto {
                log_cleanup_cron: Some("not a cron".into()),
                ..empty_update()
            },
            UpdateSettingsDto {
                log_retention_days: Some(0),
                ..empty_update()
            },
            UpdateSettingsDto {
                panel_backup_cron: Some("* *".into()),
                ..empty_update()
            },
        ] {
            let error = service.update(update).await.unwrap_err();
            assert!(
                matches!(error, sqlx::Error::Protocol(_)),
                "expected a validation error, got {error}"
            );
        }

        let read = service.get().await.unwrap();
        assert_eq!(read.server_ip, None, "a rejected update must not persist");
        assert_eq!(read.log_cleanup_cron.as_deref(), Some("0 0 * * *"));
        assert_eq!(read.log_retention_days, 30);
    }

    /// Both halves are validated before either is written, so an update
    /// rejected for one invalid field persists nothing.
    #[tokio::test]
    async fn a_rejected_update_does_not_partially_persist() {
        let (service, _pool) = service("settings-atomic").await;

        let error = service
            .persist(UpdateSettingsDto {
                host: Some("partial.example.com".into()),
                log_retention_days: Some(0),
                ..empty_update()
            })
            .await
            .unwrap_err();
        assert!(matches!(error, sqlx::Error::Protocol(_)), "{error}");

        let read = service.get().await.unwrap();
        assert_eq!(
            read.host, None,
            "the rejected update kept its already-written settings half"
        );
    }

    /// The background scheduler reads `background_policies.log_cleanup_cron`,
    /// so a saved cleanup cron must land in that row too.
    #[tokio::test]
    async fn saving_the_log_cleanup_cron_reaches_the_scheduler_policy() {
        let (service, _pool) = service("settings-cron").await;

        let response = service
            .persist(UpdateSettingsDto {
                log_cleanup_cron: Some("15 4 * * *".into()),
                ..empty_update()
            })
            .await
            .unwrap();

        let policy = service.queries.background_policies_get().await.unwrap();
        assert_eq!(
            policy.log_cleanup_cron, "15 4 * * *",
            "the scheduler still reads the seeded cleanup cron"
        );
        assert_eq!(response.log_cleanup_cron.as_deref(), Some("15 4 * * *"));
    }
}
