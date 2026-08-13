use os::string_enum;
use serde::{Deserialize, Serialize};

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, poem_openapi::Enum, ts_rs::TS)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    #[oai(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum NotificationProvider {
        default = Slack;

        Slack => "SLACK",
        Telegram => "TELEGRAM",
        Discord => "DISCORD",
        Email => "EMAIL",
        Resend => "RESEND",
        Gotify => "GOTIFY",
        Ntfy => "NTFY",
        Mattermost => "MATTERMOST",
        Pushover => "PUSHOVER",
        Custom => "CUSTOM",
        Lark => "LARK",
        Teams => "TEAMS",
    }
}
