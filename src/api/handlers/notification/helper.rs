use connectrpc::{ConnectError, ErrorCode};
use serde::{Deserialize, Serialize};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::Notifications as Notification;
use crate::services::notification::NotificationProvider;

pub fn notif_to_resource(dto: NotificationResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "notification".into(),
        payload,
        ..Default::default()
    }
}

pub fn delivery_to_resource(dto: NotificationDeliveryAttemptDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "notification_delivery".into(),
        payload,
        ..Default::default()
    }
}

pub fn binding_to_resource(dto: NotificationResourceBindingDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "notification_binding".into(),
        payload,
        ..Default::default()
    }
}

pub fn provider_of(dto: &CreateNotificationDto) -> NotificationProvider {
    use self::NotificationConfigDto::*;
    match &dto.provider {
        Slack(_) => NotificationProvider::Slack,
        Telegram(_) => NotificationProvider::Telegram,
        Discord(_) => NotificationProvider::Discord,
        Email(_) => NotificationProvider::Email,
        Resend(_) => NotificationProvider::Resend,
        Gotify(_) => NotificationProvider::Gotify,
        Ntfy(_) => NotificationProvider::Ntfy,
        Mattermost(_) => NotificationProvider::Mattermost,
        Custom(_) => NotificationProvider::Custom,
        Lark(_) => NotificationProvider::Lark,
        Pushover(_) => NotificationProvider::Pushover,
        Teams(_) => NotificationProvider::Teams,
    }
}

pub fn verify_scope(authenticated: i64, requested: i64) -> Result<(), ConnectError> {
    if authenticated == requested {
        Ok(())
    } else {
        Err(ConnectError::new(ErrorCode::PermissionDenied, "organization does not match authenticated scope"))
    }
}

// ── Notification payload DTOs ─────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationSettingsDto {
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

#[derive(Debug, Clone, Deserialize)]
pub struct SlackNotificationDto {
    pub webhook_url: String,
    pub channel: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TelegramNotificationDto {
    pub bot_token: String,
    pub chat_id: String,
    pub message_thread_id: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DiscordNotificationDto {
    pub webhook_url: String,
    #[serde(default)]
    pub decoration: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EmailNotificationDto {
    pub smtp_server: String,
    pub smtp_port: i64,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub to_addresses: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ResendNotificationDto {
    pub api_key: String,
    pub from_address: String,
    pub to_addresses: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GotifyNotificationDto {
    pub server_url: String,
    pub app_token: String,
    pub priority: i64,
    #[serde(default)]
    pub decoration: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NtfyNotificationDto {
    pub server_url: String,
    pub topic: String,
    pub access_token: Option<String>,
    pub priority: i64,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MattermostNotificationDto {
    pub webhook_url: String,
    pub channel: Option<String>,
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CustomNotificationDto {
    pub endpoint: String,
    pub headers: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookNotificationDto {
    pub webhook_url: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PushoverNotificationDto {
    pub user_key: String,
    pub api_token: String,
    pub priority: i64,
    pub retry: Option<i64>,
    pub expire: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(
    tag = "notification_type",
    content = "config",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNotificationDto {
    pub settings: NotificationSettingsDto,
    pub provider: NotificationConfigDto,
}

#[derive(Debug, Clone, Deserialize)]
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
    pub on_schedule_success: Option<bool>,
    pub on_schedule_failure: Option<bool>,

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

#[derive(Debug, Clone, Serialize)]
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
    pub on_schedule_success: bool,
    pub on_schedule_failure: bool,
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
            id: value.id,
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
            on_schedule_success: value.on_schedule_success != 0,
            on_schedule_failure: value.on_schedule_failure != 0,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNotificationBindingDto {
    pub notification_id: i64,
    pub resource_type: NotificationResourceType,
    pub resource_id: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationResourceType {
    Server,
    Application,
    Compose,
    Database,
}

impl NotificationResourceType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Server => "SERVER",
            Self::Application => "APPLICATION",
            Self::Compose => "COMPOSE",
            Self::Database => "DATABASE",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationDeliveryAttemptDto {
    pub id: i64,
    pub notification_id: i64,
    pub organization_id: i64,
    pub trigger_name: String,
    pub correlation_id: String,
    pub status: String,
    pub attempt: i64,
    pub title: String,
    pub body: String,
    pub error: Option<String>,
    pub created_at: i64,
    pub finished_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationResourceBindingDto {
    pub id: i64,
    pub notification_id: i64,
    pub organization_id: i64,
    pub resource_type: String,
    pub resource_id: i64,
    pub created_at: i64,
}

impl From<crate::flash_gen::models::NotificationDeliveryAttempts> for NotificationDeliveryAttemptDto {
    fn from(value: crate::flash_gen::models::NotificationDeliveryAttempts) -> Self {
        Self {
            id: value.id,
            notification_id: value.notification_id,
            organization_id: value.organization_id,
            trigger_name: value.trigger_name,
            correlation_id: value.correlation_id,
            status: value.status,
            attempt: value.attempt,
            title: value.title,
            body: value.body,
            error: value.error,
            created_at: value.created_at,
            finished_at: value.finished_at,
        }
    }
}

impl From<crate::flash_gen::models::NotificationResourceBindings> for NotificationResourceBindingDto {
    fn from(value: crate::flash_gen::models::NotificationResourceBindings) -> Self {
        Self {
            id: value.id,
            notification_id: value.notification_id,
            organization_id: value.organization_id,
            resource_type: value.resource_type,
            resource_id: value.resource_id,
            created_at: value.created_at,
        }
    }
}
