use crate::{
    db::models::notif_custom::NotifCustom,
    services::notification::{guard::validate_webhook_url, message::NotificationMessage},
};
use reqwest::Client;
use serde_json::{Value, json};

pub async fn send_custom(
    client: &Client,
    cfg: &NotifCustom,
    msg: &NotificationMessage,
) -> Result<(), String> {
    validate_webhook_url(&cfg.endpoint)?;

    let payload = json!({
        "title": msg.title,
        "body": msg.body,
        "level": msg.level.as_str(),
        "timestamp": msg.timestamp,
        "url": msg.url,
        "fields": msg.fields.iter()
            .map(|(k, v)| json!({"key": k, "value": v}))
            .collect::<Vec<_>>(),
    });

    let mut request = client.post(&cfg.endpoint).json(&payload);

    if let Some(raw) = cfg.headers.as_deref().filter(|h| !h.trim().is_empty()) {
        match serde_json::from_str::<Value>(raw) {
            Ok(Value::Object(map)) => {
                for (key, value) in map {
                    if let Some(value) = value.as_str() {
                        request = request.header(key, value);
                    }
                }
            }
            _ => tracing::warn!("custom notification headers are not a JSON object, ignoring"),
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("custom webhook request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!(
            "custom webhook returned status {}",
            response.status()
        ));
    }

    Ok(())
}
