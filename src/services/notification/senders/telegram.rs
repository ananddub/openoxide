use crate::{
    db::models::notif_telegram::NotifTelegram, services::notification::message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_telegram(
    client: &Client,
    cfg: &NotifTelegram,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", cfg.bot_token);

    let mut payload = json!({
        "chat_id": cfg.chat_id,
        "text": msg.to_plain_text(),
        "disable_web_page_preview": true,
    });

    if let Some(thread) = cfg
        .message_thread_id
        .as_deref()
        .filter(|t| !t.is_empty())
        .and_then(|t| t.parse::<i64>().ok())
    {
        payload["message_thread_id"] = json!(thread);
    }

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("telegram request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("telegram returned status {}", response.status()));
    }

    Ok(())
}
