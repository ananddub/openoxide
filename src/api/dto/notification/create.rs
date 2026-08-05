use poem_openapi::{Object, Union};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct NotificationSettingsDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub organization_id: i64,
    #[serde(default)]
    pub on_app_deploy: bool,
    #[serde(default)]
    pub on_app_build_error: bool,
    #[serde(default)]
    pub on_database_backup: bool,
    #[serde(default)]
    pub on_volume_backup: bool,
    #[serde(default)]
    pub on_panel_restart: bool,
    #[serde(default)]
    pub on_docker_cleanup: bool,
    #[serde(default)]
    pub on_server_threshold: bool,
    #[serde(default)]
    pub on_panel_backup: bool,
    #[serde(default)]
    pub on_schedule_success: bool,
    #[serde(default)]
    pub on_schedule_failure: bool,
}

#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct SlackNotificationDto {
    #[validate(url)]
    pub webhook_url: String,
    pub channel: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct TelegramNotificationDto {
    #[validate(length(min = 1))]
    pub bot_token: String,
    #[validate(length(min = 1))]
    pub chat_id: String,
    pub message_thread_id: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct DiscordNotificationDto {
    #[validate(url)]
    pub webhook_url: String,
    #[serde(default)]
    pub decoration: bool,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct EmailNotificationDto {
    #[validate(length(min = 1))]
    pub smtp_server: String,
    #[validate(range(min = 1, max = 65535))]
    pub smtp_port: i64,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(email)]
    pub from_address: String,
    #[validate(length(min = 1))]
    pub to_addresses: Vec<String>,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct ResendNotificationDto {
    #[validate(length(min = 1))]
    pub api_key: String,
    #[validate(email)]
    pub from_address: String,
    #[validate(length(min = 1))]
    pub to_addresses: Vec<String>,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct GotifyNotificationDto {
    #[validate(url)]
    pub server_url: String,
    #[validate(length(min = 1))]
    pub app_token: String,
    #[validate(range(min = 0, max = 10))]
    pub priority: i64,
    #[serde(default)]
    pub decoration: bool,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct NtfyNotificationDto {
    #[validate(url)]
    pub server_url: String,
    #[validate(length(min = 1))]
    pub topic: String,
    pub access_token: Option<String>,
    #[validate(range(min = 1, max = 5))]
    pub priority: i64,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct MattermostNotificationDto {
    #[validate(url)]
    pub webhook_url: String,
    pub channel: Option<String>,
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct CustomNotificationDto {
    #[validate(url)]
    pub endpoint: String,
    pub headers: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct WebhookNotificationDto {
    #[validate(url)]
    pub webhook_url: String,
}
#[derive(Debug, Clone, Deserialize, Validate, Object)]
pub struct PushoverNotificationDto {
    #[validate(length(min = 1))]
    pub user_key: String,
    #[validate(length(min = 1))]
    pub api_token: String,
    #[validate(range(min = -2, max = 2))]
    pub priority: i64,
    #[validate(range(min = 30))]
    pub retry: Option<i64>,
    #[validate(range(min = 1, max = 10800))]
    pub expire: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Union)]
#[serde(
    tag = "notification_type",
    content = "config",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
#[oai(discriminator_name = "notification_type")]
pub enum NotificationConfigDto {
    Slack(SlackNotificationDto),
    Telegram(TelegramNotificationDto),
    Discord(DiscordNotificationDto),
    Email(EmailNotificationDto),
    Resend(ResendNotificationDto),
    Gotify(GotifyNotificationDto),
    Ntfy(NtfyNotificationDto),
    Mattermost(MattermostNotificationDto),
    Custom(CustomNotificationDto),
    Lark(WebhookNotificationDto),
    Pushover(PushoverNotificationDto),
    Teams(WebhookNotificationDto),
}

#[derive(Debug, Clone, Deserialize, Object)]
pub struct CreateNotificationDto {
    #[oai(flatten)]
    pub settings: NotificationSettingsDto,
    #[oai(flatten)]
    pub provider: NotificationConfigDto,
}

impl Validate for CreateNotificationDto {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        self.settings.validate()?;
        match &self.provider {
            NotificationConfigDto::Slack(v) => v.validate(),
            NotificationConfigDto::Telegram(v) => v.validate(),
            NotificationConfigDto::Discord(v) => v.validate(),
            NotificationConfigDto::Email(v) => v.validate(),
            NotificationConfigDto::Resend(v) => v.validate(),
            NotificationConfigDto::Gotify(v) => v.validate(),
            NotificationConfigDto::Ntfy(v) => v.validate(),
            NotificationConfigDto::Mattermost(v) => v.validate(),
            NotificationConfigDto::Custom(v) => v.validate(),
            NotificationConfigDto::Lark(v) => v.validate(),
            NotificationConfigDto::Pushover(v) => {
                v.validate()?;
                if v.priority == 2 && (v.retry.is_none() || v.expire.is_none()) {
                    let mut e = validator::ValidationErrors::new();
                    e.add(
                        "config",
                        validator::ValidationError::new("emergency_requires_retry_and_expire"),
                    );
                    Err(e)
                } else {
                    Ok(())
                }
            }
            NotificationConfigDto::Teams(v) => v.validate(),
        }
    }
}
