use super::webhook::post_webhook;
use crate::services::integrations::notification::{
    loader::LarkConfig, message::NotificationMessage,
};
use reqwest::Client;
use serde_json::json;

pub async fn send_lark(
    client: &Client,
    cfg: &LarkConfig,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let payload = json!({
        "msg_type": "text",
        "content": { "text": msg.to_plain_text() },
    });

    post_webhook(client, &cfg.webhook_url, &payload, "lark").await
}
