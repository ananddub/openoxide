use connectrpc::{ConnectError, ErrorCode};
use serde_json::json;

use crate::api::connect::generated::openoxide::api::v1::{
    Notification as ProtoNotification, NotificationBinding as ProtoNotificationBinding,
    NotificationDeliveryAttempt as ProtoNotificationDeliveryAttempt,
    NotificationProviderType as ProtoNotificationProviderType,
    NotificationResourceType as ProtoNotificationResourceType,
};
use crate::flash_gen::models::NotificationDeliveryAttempts;
use crate::flash_gen::models::NotificationResourceBindings;
use crate::flash_gen::models::Notifications as Notification;
use crate::services::integrations::notification::NotificationProvider;

pub fn verify_scope(authenticated: i64, requested: i64) -> Result<(), ConnectError> {
    if authenticated == requested {
        Ok(())
    } else {
        Err(ConnectError::new(ErrorCode::PermissionDenied, "organization does not match authenticated scope"))
    }
}

pub fn internal(error: impl std::fmt::Display) -> ConnectError {
    ConnectError::new(ErrorCode::Internal, error.to_string())
}

// ── Provider kind mapping ─────────────────────────────────────────

/// Rows carry the provider as its SCREAMING_SNAKE name; an unrecognized
/// stored value falls back to `Custom`, matching the previous DTO behavior.
fn provider_of_row(stored: &str) -> NotificationProvider {
    stored.parse().unwrap_or(NotificationProvider::Custom)
}

fn provider_to_proto(provider: NotificationProvider) -> ::buffa::EnumValue<ProtoNotificationProviderType> {
    ::buffa::EnumValue::Known(match provider {
        NotificationProvider::Slack => ProtoNotificationProviderType::Slack,
        NotificationProvider::Telegram => ProtoNotificationProviderType::Telegram,
        NotificationProvider::Discord => ProtoNotificationProviderType::Discord,
        NotificationProvider::Email => ProtoNotificationProviderType::Email,
        NotificationProvider::Resend => ProtoNotificationProviderType::Resend,
        NotificationProvider::Gotify => ProtoNotificationProviderType::Gotify,
        NotificationProvider::Ntfy => ProtoNotificationProviderType::Ntfy,
        NotificationProvider::Mattermost => ProtoNotificationProviderType::Mattermost,
        NotificationProvider::Pushover => ProtoNotificationProviderType::Pushover,
        NotificationProvider::Custom => ProtoNotificationProviderType::Custom,
        NotificationProvider::Lark => ProtoNotificationProviderType::Lark,
        NotificationProvider::Teams => ProtoNotificationProviderType::Teams,
    })
}

// ── Row → proto mappings ──────────────────────────────────────────

/// Notification rows never echo provider credentials; config is write-only,
/// so nothing from `config_json` is exposed here.
pub fn notif_to_proto(value: Notification) -> ProtoNotification {
    let provider = provider_of_row(&value.notification_type);
    ProtoNotification {
        id: value.id,
        name: value.name,
        notification_type: provider_to_proto(provider),
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
        ..Default::default()
    }
}

pub fn delivery_to_proto(value: NotificationDeliveryAttempts) -> ProtoNotificationDeliveryAttempt {
    ProtoNotificationDeliveryAttempt {
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
        ..Default::default()
    }
}

pub fn binding_to_proto(value: NotificationResourceBindings) -> ProtoNotificationBinding {
    ProtoNotificationBinding {
        id: value.id,
        notification_id: value.notification_id,
        organization_id: value.organization_id,
        resource_type: ::buffa::EnumValue::Known(match value.resource_type.as_str() {
            "SERVER" => ProtoNotificationResourceType::Server,
            "APPLICATION" => ProtoNotificationResourceType::Application,
            "COMPOSE" => ProtoNotificationResourceType::Compose,
            "DATABASE" => ProtoNotificationResourceType::Database,
            _ => ProtoNotificationResourceType::Unspecified,
        }),
        resource_id: value.resource_id,
        created_at: value.created_at,
        ..Default::default()
    }
}

/// Stored spelling of the binding resource kind (SCREAMING_SNAKE, matching
/// the string previously produced by `NotificationResourceType::as_str`).
pub fn proto_resource_type_name(
    value: ::buffa::EnumValue<ProtoNotificationResourceType>,
) -> Result<String, ConnectError> {
    Ok(match value {
        ::buffa::EnumValue::Known(ProtoNotificationResourceType::Server) => "SERVER",
        ::buffa::EnumValue::Known(ProtoNotificationResourceType::Application) => "APPLICATION",
        ::buffa::EnumValue::Known(ProtoNotificationResourceType::Compose) => "COMPOSE",
        ::buffa::EnumValue::Known(ProtoNotificationResourceType::Database) => "DATABASE",
        _ => {
            return Err(ConnectError::invalid_argument(
                "resource_type must be a defined notification resource",
            ))
        }
    }
    .to_owned())
}

// ── PATCH config merge ────────────────────────────────────────────

/// Applies the optional channel-config fields of an update request onto the
/// stored `config_json` object in place. Absent fields keep their stored
/// values; the provider kind itself is immutable here. The `endpoint` wire
/// field is stored under the config key `url`, as documented in
/// `.flash/queries/notifications.sql`.
pub fn apply_provider_patch(
    config_json: &mut String,
    fields: Vec<(&'static str, serde_json::Value)>,
    to_addresses: Vec<String>,
) -> Result<(), ConnectError> {
    if fields.is_empty() && to_addresses.is_empty() {
        return Ok(());
    }
    let mut config: serde_json::Value = if config_json.is_empty() {
        json!({})
    } else {
        serde_json::from_str(config_json).map_err(|e| internal(e.to_string()))?
    };
    let object = config
        .as_object_mut()
        .ok_or_else(|| internal("stored notification config is not a JSON object"))?;
    for (key, value) in fields {
        object.insert(key.to_owned(), value);
    }
    if !to_addresses.is_empty() {
        object.insert("to_addresses".to_owned(), json!(to_addresses));
    }
    *config_json = serde_json::to_string(&config).map_err(internal)?;
    Ok(())
}
