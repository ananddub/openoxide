use super::{guard::validate_webhook_url, message::NotificationMessage};
use crate::db::models::{
    notif_custom::NotifCustom, notif_discord::NotifDiscord, notif_gotify::NotifGotify,
    notif_lark::NotifLark, notif_mattermost::NotifMattermost, notif_ntfy::NotifNtfy,
    notif_pushover::NotifPushover, notif_resend::NotifResend, notif_slack::NotifSlack,
    notif_teams::NotifTeam, notif_telegram::NotifTelegram,
};
use reqwest::Client;
use serde_json::{Value, json};

/// Parses a JSON array of strings, as stored in `notif_email.to_addresses` and
/// `notif_resend.to_addresses`. Falls back to comma-separated parsing so a
/// hand-edited row doesn't silently drop every recipient.
pub fn parse_addresses(raw: &str) -> Vec<String> {
    if let Ok(Value::Array(items)) = serde_json::from_str::<Value>(raw) {
        return items
            .into_iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .filter(|s| !s.trim().is_empty())
            .collect();
    }

    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// POSTs a JSON body to a user-supplied webhook URL, after SSRF validation.
async fn post_webhook(
    client: &Client,
    url: &str,
    payload: &Value,
    provider: &str,
) -> Result<(), String> {
    validate_webhook_url(url)?;

    let response = client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(|e| format!("{provider} request failed: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        // Body is read for the operator-facing log only. It is never returned to
        // the API caller, which would turn a webhook into an SSRF exfil channel.
        let body = response.text().await.unwrap_or_default();
        tracing::debug!(provider, status = status.as_u16(), body = %body, "notification provider returned an error");
        return Err(format!("{provider} returned status {status}"));
    }

    Ok(())
}

pub async fn send_slack(
    client: &Client,
    cfg: &NotifSlack,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut blocks = vec![
        json!({
            "type": "header",
            "text": {"type": "plain_text", "text": msg.subject(), "emoji": true}
        }),
        json!({
            "type": "section",
            "text": {"type": "mrkdwn", "text": msg.body}
        }),
    ];

    if !msg.fields.is_empty() {
        blocks.push(json!({
            "type": "section",
            "fields": msg.fields.iter().map(|(k, v)| json!({
                "type": "mrkdwn",
                "text": format!("*{k}*\n{v}")
            })).collect::<Vec<_>>()
        }));
    }

    if let Some(url) = &msg.url {
        blocks.push(json!({
            "type": "section",
            "text": {"type": "mrkdwn", "text": format!("<{url}|Open in Rustploy>")}
        }));
    }

    let mut payload = json!({ "text": msg.subject(), "blocks": blocks });
    if let Some(channel) = cfg.channel.as_deref().filter(|c| !c.is_empty()) {
        payload["channel"] = json!(channel);
    }

    post_webhook(client, &cfg.webhook_url, &payload, "slack").await
}

pub async fn send_discord(
    client: &Client,
    cfg: &NotifDiscord,
    msg: &NotificationMessage,
) -> Result<(), String> {
    // decoration = 0 means the user opted out of rich embeds
    let payload = if cfg.decoration == 0 {
        json!({ "content": msg.to_plain_text() })
    } else {
        let mut embed = json!({
            "title": msg.subject(),
            "description": msg.body,
            "color": msg.level.discord_color(),
            "timestamp": chrono::DateTime::from_timestamp(msg.timestamp, 0)
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
        });

        if !msg.fields.is_empty() {
            embed["fields"] = json!(
                msg.fields
                    .iter()
                    .map(|(k, v)| json!({"name": k, "value": v, "inline": true}))
                    .collect::<Vec<_>>()
            );
        }

        if let Some(url) = &msg.url {
            embed["url"] = json!(url);
        }

        json!({ "embeds": [embed] })
    };

    post_webhook(client, &cfg.webhook_url, &payload, "discord").await
}

pub async fn send_telegram(
    client: &Client,
    cfg: &NotifTelegram,
    msg: &NotificationMessage,
) -> Result<(), String> {
    // Bot token is a secret, so the URL is built by us rather than user-supplied.
    // It always points at api.telegram.org, hence no SSRF check.
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

pub async fn send_gotify(
    client: &Client,
    cfg: &NotifGotify,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let base = cfg.server_url.trim_end_matches('/');
    let url = format!("{base}/message");
    validate_webhook_url(&url)?;

    // priority 0 in config means "derive from severity"
    let priority = if cfg.priority > 0 {
        cfg.priority
    } else {
        msg.level.gotify_priority()
    };

    let payload = json!({
        "title": msg.subject(),
        "message": msg.to_plain_text(),
        "priority": priority,
    });

    let response = client
        .post(&url)
        .header("X-Gotify-Key", &cfg.app_token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("gotify request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("gotify returned status {}", response.status()));
    }

    Ok(())
}

pub async fn send_ntfy(
    client: &Client,
    cfg: &NotifNtfy,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let base = cfg.server_url.trim_end_matches('/');
    let url = format!("{base}/{}", cfg.topic);
    validate_webhook_url(&url)?;

    let priority = if cfg.priority > 0 {
        cfg.priority
    } else {
        msg.level.ntfy_priority()
    };

    let mut request = client
        .post(&url)
        .header("Title", msg.subject())
        .header("Priority", priority.to_string())
        .body(msg.to_plain_text());

    if let Some(token) = cfg.access_token.as_deref().filter(|t| !t.is_empty()) {
        request = request.bearer_auth(token);
    }

    if let Some(link) = &msg.url {
        request = request.header("Click", link);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("ntfy request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("ntfy returned status {}", response.status()));
    }

    Ok(())
}

pub async fn send_mattermost(
    client: &Client,
    cfg: &NotifMattermost,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let mut payload = json!({ "text": msg.to_plain_text() });

    if let Some(channel) = cfg.channel.as_deref().filter(|c| !c.is_empty()) {
        payload["channel"] = json!(channel);
    }
    if let Some(username) = cfg.username.as_deref().filter(|u| !u.is_empty()) {
        payload["username"] = json!(username);
    }

    post_webhook(client, &cfg.webhook_url, &payload, "mattermost").await
}

pub async fn send_teams(
    client: &Client,
    cfg: &NotifTeam,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let payload = json!({
        "@type": "MessageCard",
        "@context": "https://schema.org/extensions",
        "summary": msg.title,
        "themeColor": msg.level.teams_color(),
        "title": msg.subject(),
        "text": msg.body,
        "sections": [{
            "facts": msg.fields.iter()
                .map(|(k, v)| json!({"name": k, "value": v}))
                .collect::<Vec<_>>()
        }],
    });

    post_webhook(client, &cfg.webhook_url, &payload, "teams").await
}

pub async fn send_lark(
    client: &Client,
    cfg: &NotifLark,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let payload = json!({
        "msg_type": "text",
        "content": { "text": msg.to_plain_text() },
    });

    post_webhook(client, &cfg.webhook_url, &payload, "lark").await
}

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

    // Emergency priority (2) requires retry/expire or the API rejects the call.
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

pub async fn send_resend(
    client: &Client,
    cfg: &NotifResend,
    msg: &NotificationMessage,
) -> Result<(), String> {
    let to = parse_addresses(&cfg.to_addresses);
    if to.is_empty() {
        return Err("resend notification has no recipients".to_string());
    }

    let payload = json!({
        "from": cfg.from_address,
        "to": to,
        "subject": msg.subject(),
        "text": msg.to_plain_text(),
    });

    let response = client
        .post("https://api.resend.com/emails")
        .bearer_auth(&cfg.api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("resend request failed: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("resend returned status {}", response.status()));
    }

    Ok(())
}

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

    // headers column holds a JSON object of extra headers
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
