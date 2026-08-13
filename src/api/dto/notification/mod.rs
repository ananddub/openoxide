pub mod common;
pub mod create;

pub use common::{NotificationResponseDto, PatchNotificationDto};
pub use create::CreateNotificationDto;

#[derive(Debug, Clone, serde::Deserialize, poem_openapi::Object)]
pub struct CreateNotificationBindingDto {
    pub notification_id: i64,
    pub resource_type: NotificationResourceType,
    pub resource_id: i64,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object, ts_rs::TS)]
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

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct NotificationResourceBindingDto {
    pub id: i64,
    pub notification_id: i64,
    pub organization_id: i64,
    pub resource_type: String,
    pub resource_id: i64,
    pub created_at: i64,
}

impl From<crate::db::repository::notification_delivery::NotificationDeliveryAttempt>
    for NotificationDeliveryAttemptDto
{
    fn from(
        value: crate::db::repository::notification_delivery::NotificationDeliveryAttempt,
    ) -> Self {
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

impl From<crate::db::repository::notification_delivery::NotificationResourceBinding>
    for NotificationResourceBindingDto
{
    fn from(
        value: crate::db::repository::notification_delivery::NotificationResourceBinding,
    ) -> Self {
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
