use crate::services::notification::NotificationProvider;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, poem_openapi::Object)]
pub struct CreateNotificationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub notification_type: NotificationProvider,
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

    pub webhook_url: Option<String>,
    pub channel: Option<String>,
    pub username: Option<String>,
    pub decoration: Option<i64>,

    pub bot_token: Option<String>,
    pub chat_id: Option<String>,
    pub message_thread_id: Option<String>,

    pub smtp_server: Option<String>,
    pub smtp_port: Option<i64>,
    pub password: Option<String>,
    pub from_address: Option<String>,
    pub to_addresses: Option<Vec<String>>,
    pub api_key: Option<String>,

    pub server_url: Option<String>,
    pub app_token: Option<String>,
    pub priority: Option<i64>,
    pub topic: Option<String>,
    pub access_token: Option<String>,
    pub endpoint: Option<String>,
    pub headers: Option<String>,

    pub user_key: Option<String>,
    pub api_token: Option<String>,
    pub retry: Option<i64>,
    pub expire: Option<i64>,
}
