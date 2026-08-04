use crate::{
    db::models::notifications::Notification, services::notification::NotificationProvider,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, poem_openapi::Object)]
pub struct PatchNotificationDto {
    pub name: Option<String>,
    pub on_app_deploy: Option<bool>,
    pub on_app_build_error: Option<bool>,
    pub on_database_backup: Option<bool>,
    pub on_volume_backup: Option<bool>,
    pub on_panel_restart: Option<bool>,
    pub on_docker_cleanup: Option<bool>,
    pub on_server_threshold: Option<bool>,
    pub on_panel_backup: Option<bool>,

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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct NotificationResponseDto {
    pub id: i64,
    pub name: String,
    pub notification_type: NotificationProvider,
    pub on_app_deploy: bool,
    pub on_app_build_error: bool,
    pub on_database_backup: bool,
    pub on_volume_backup: bool,
    pub on_panel_restart: bool,
    pub on_docker_cleanup: bool,
    pub on_server_threshold: bool,
    pub on_panel_backup: bool,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Notification> for NotificationResponseDto {
    fn from(value: Notification) -> Self {
        let provider: NotificationProvider = value
            .notification_type
            .parse()
            .unwrap_or(NotificationProvider::Custom);

        Self {
            id: value.id.unwrap_or_default(),
            name: value.name,
            notification_type: provider,
            on_app_deploy: value.on_app_deploy != 0,
            on_app_build_error: value.on_app_build_error != 0,
            on_database_backup: value.on_database_backup != 0,
            on_volume_backup: value.on_volume_backup != 0,
            on_panel_restart: value.on_panel_restart != 0,
            on_docker_cleanup: value.on_docker_cleanup != 0,
            on_server_threshold: value.on_server_threshold != 0,
            on_panel_backup: value.on_panel_backup != 0,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
