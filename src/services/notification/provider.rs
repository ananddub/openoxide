use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationProvider {
    Slack,
    Telegram,
    Discord,
    Email,
    Resend,
    Gotify,
    Ntfy,
    Mattermost,
    Pushover,
    Custom,
    Lark,
    Teams,
}

impl NotificationProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Slack => "SLACK",
            Self::Telegram => "TELEGRAM",
            Self::Discord => "DISCORD",
            Self::Email => "EMAIL",
            Self::Resend => "RESEND",
            Self::Gotify => "GOTIFY",
            Self::Ntfy => "NTFY",
            Self::Mattermost => "MATTERMOST",
            Self::Pushover => "PUSHOVER",
            Self::Custom => "CUSTOM",
            Self::Lark => "LARK",
            Self::Teams => "TEAMS",
        }
    }
}

impl FromStr for NotificationProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "SLACK" => Ok(Self::Slack),
            "TELEGRAM" => Ok(Self::Telegram),
            "DISCORD" => Ok(Self::Discord),
            "EMAIL" => Ok(Self::Email),
            "RESEND" => Ok(Self::Resend),
            "GOTIFY" => Ok(Self::Gotify),
            "NTFY" => Ok(Self::Ntfy),
            "MATTERMOST" => Ok(Self::Mattermost),
            "PUSHOVER" => Ok(Self::Pushover),
            "CUSTOM" => Ok(Self::Custom),
            "LARK" => Ok(Self::Lark),
            "TEAMS" => Ok(Self::Teams),
            other => Err(format!("Unknown notification provider: '{other}'")),
        }
    }
}
