use crate::flash_gen::models::Notifications as Notification;
use serde::{Deserialize, Serialize};

/// Channel configuration extracted from a notification's `config_json`.
///
/// Key layout per channel is documented in `.flash/queries/notifications.sql`.
#[derive(Debug, Clone)]
pub enum ChannelConfig {
    Slack(SlackConfig),
    Telegram(TelegramConfig),
    Discord(DiscordConfig),
    Email(EmailConfig),
    Resend(ResendConfig),
    Gotify(GotifyConfig),
    Ntfy(NtfyConfig),
    Mattermost(MattermostConfig),
    Teams(TeamsConfig),
    Lark(LarkConfig),
    Pushover(PushoverConfig),
    Custom(CustomConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackConfig {
    pub webhook_url: String,
    #[serde(default)]
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
    #[serde(default)]
    pub message_thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub webhook_url: String,
    #[serde(default)]
    pub decoration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: i64,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    pub from_address: String,
    #[serde(default)]
    pub to_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendConfig {
    pub api_key: String,
    pub from_address: String,
    #[serde(default)]
    pub to_addresses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GotifyConfig {
    pub server_url: String,
    pub app_token: String,
    #[serde(default)]
    pub priority: i64,
    #[serde(default)]
    pub decoration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtfyConfig {
    pub server_url: String,
    pub topic: String,
    #[serde(default)]
    pub access_token: Option<String>,
    #[serde(default)]
    pub priority: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MattermostConfig {
    pub webhook_url: String,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamsConfig {
    pub webhook_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LarkConfig {
    pub webhook_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushoverConfig {
    pub api_token: String,
    pub user_key: String,
    #[serde(default)]
    pub priority: i64,
    #[serde(default)]
    pub retry: Option<i64>,
    #[serde(default)]
    pub expire: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomConfig {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub headers: Option<String>,
    #[serde(default)]
    pub payload_template: Option<String>,
}

/// Parses per-channel configuration out of `notifications.config_json`.
///
/// Configs used to live in twelve per-channel tables; since the unified
/// schema they are embedded in the notification row itself, so loading is
/// a pure `serde_json` decode with no database round-trip.
pub struct NotificationConfigLoader;

impl NotificationConfigLoader {
    fn parse_config<T: serde::de::DeserializeOwned>(
        notification: &Notification,
    ) -> Result<T, String> {
        serde_json::from_str(&notification.config_json)
            .map_err(|e| format!("invalid config for {} notification: {e}", notification.notification_type))
    }

    /// Loads the channel config matching the notification's `notification_type`.
    pub fn load(&self, notification: &Notification) -> Result<ChannelConfig, String> {
        use super::provider::NotificationProvider;

        let provider: NotificationProvider = notification
            .notification_type
            .parse()
            .map_err(|e| format!("unknown notification type: {e}"))?;

        Ok(match provider {
            NotificationProvider::Slack => {
                ChannelConfig::Slack(Self::parse_config(notification)?)
            }
            NotificationProvider::Telegram => {
                ChannelConfig::Telegram(Self::parse_config(notification)?)
            }
            NotificationProvider::Discord => {
                ChannelConfig::Discord(Self::parse_config(notification)?)
            }
            NotificationProvider::Email => ChannelConfig::Email(Self::parse_config(notification)?),
            NotificationProvider::Resend => {
                ChannelConfig::Resend(Self::parse_config(notification)?)
            }
            NotificationProvider::Gotify => {
                ChannelConfig::Gotify(Self::parse_config(notification)?)
            }
            NotificationProvider::Ntfy => ChannelConfig::Ntfy(Self::parse_config(notification)?),
            NotificationProvider::Mattermost => {
                ChannelConfig::Mattermost(Self::parse_config(notification)?)
            }
            NotificationProvider::Teams => {
                ChannelConfig::Teams(Self::parse_config(notification)?)
            }
            NotificationProvider::Lark => ChannelConfig::Lark(Self::parse_config(notification)?),
            NotificationProvider::Pushover => {
                ChannelConfig::Pushover(Self::parse_config(notification)?)
            }
            NotificationProvider::Custom => {
                ChannelConfig::Custom(Self::parse_config(notification)?)
            }
        })
    }

    pub fn load_slack(&self, n: &Notification) -> Result<SlackConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_telegram(&self, n: &Notification) -> Result<TelegramConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_discord(&self, n: &Notification) -> Result<DiscordConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_email(&self, n: &Notification) -> Result<EmailConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_resend(&self, n: &Notification) -> Result<ResendConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_gotify(&self, n: &Notification) -> Result<GotifyConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_ntfy(&self, n: &Notification) -> Result<NtfyConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_mattermost(&self, n: &Notification) -> Result<MattermostConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_teams(&self, n: &Notification) -> Result<TeamsConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_lark(&self, n: &Notification) -> Result<LarkConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_pushover(&self, n: &Notification) -> Result<PushoverConfig, String> {
        Self::parse_config(n)
    }

    pub fn load_custom(&self, n: &Notification) -> Result<CustomConfig, String> {
        Self::parse_config(n)
    }
}
