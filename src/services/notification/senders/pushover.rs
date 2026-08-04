use crate::{
    db::models::notif_pushover::NotifPushover, services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_pushover(
    client: &Client,
    cfg: &NotifPushover,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut payload = json!({
        "token": cfg.api_token,
        "user": cfg.user_key,
        "title": msg.subject(),
        "message": msg.to_plain_text(),
        "priority": cfg.priority,
    });

    if cfg.priority >= 2 {
        payload["retry"] = json!(cfg.retry.unwrap_or(60).max(30));
        payload["expire"] = json!(cfg.expire.unwrap_or(3600).min(10_800));
    }

    if let Some(url) = &msg.url {
        payload["url"] = json!(url);
    }

    let response = client
        .post("https://api.pushover.net/1/messages.json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("pushover request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("pushover returned status {}", response.status()));
    }

    Ok(())
}
