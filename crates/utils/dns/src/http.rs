use reqwest::{Client, RequestBuilder, Response};

use crate::error::DnsError;

/// A provider's HTTP surface: one base URL plus the shared client, so a test can point a
/// client at a local mock server without touching the production default.
///
/// The client comes from the caller. This crate never installs a rustls crypto provider:
/// the panel binary does that once at startup (`src/main.rs`) against a `reqwest` built
/// with `rustls-no-provider`, and a second install attempt would abort the process.
#[derive(Debug, Clone)]
pub struct Http {
    client: Client,
    base_url: String,
}

impl Http {
    pub fn new(client: &Client, base_url: impl Into<String>) -> Self {
        Self {
            client: client.clone(),
            base_url: base_url.into(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(self.url(path))
    }

    pub fn request(&self, method: reqwest::Method, path: &str) -> RequestBuilder {
        self.client.request(method, self.url(path))
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.client.post(self.url(path))
    }

    pub fn put(&self, path: &str) -> RequestBuilder {
        self.client.put(self.url(path))
    }

    pub fn patch(&self, path: &str) -> RequestBuilder {
        self.client.patch(self.url(path))
    }

    pub fn delete(&self, path: &str) -> RequestBuilder {
        self.client.delete(self.url(path))
    }

    /// Reads the body as JSON and fails on a non-2xx status, keeping whatever the provider
    /// said about why. A body that is not JSON still surfaces as an error with its text, so
    /// an HTML maintenance page never looks like an empty result.
    pub async fn json(
        &self,
        response: Response,
        provider: &str,
        operation: &str,
    ) -> Result<serde_json::Value, DnsError> {
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            return Err(DnsError::provider(describe_failure(
                provider, operation, status, &text,
            )));
        }
        if text.trim().is_empty() {
            return Ok(serde_json::Value::Null);
        }
        serde_json::from_str(&text).map_err(|_| {
            DnsError::provider(format!(
                "{provider}: {operation} returned a non-JSON response ({status}): {}",
                clip(&text)
            ))
        })
    }
}

fn describe_failure(
    provider: &str,
    operation: &str,
    status: reqwest::StatusCode,
    text: &str,
) -> String {
    let reason = error_reason(text);
    if reason.is_empty() {
        format!("{provider}: {operation} failed (status {status})")
    } else {
        format!("{provider}: {operation} failed (status {status}): {reason}")
    }
}

/// Pulls the human-readable reason out of the several envelope shapes these APIs use.
/// Providers with a richer envelope (Cloudflare's `errors[]`, OVH's `message`) add their own
/// reading on top; this covers the common ones so a failure is never silent.
pub(crate) fn error_reason(text: &str) -> String {
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) else {
        return clip(text);
    };
    for key in ["message", "error", "reason"] {
        if let Some(reason) = parsed.get(key).and_then(serde_json::Value::as_str)
            && !reason.trim().is_empty()
        {
            return reason.to_string();
        }
    }
    if let Some(errors) = parsed.get("errors").and_then(serde_json::Value::as_array) {
        let messages: Vec<&str> = errors
            .iter()
            .filter_map(|entry| {
                entry
                    .get("message")
                    .or_else(|| entry.as_str().map(|_| entry))
                    .and_then(serde_json::Value::as_str)
            })
            .collect();
        if !messages.is_empty() {
            return messages.join(", ");
        }
    }
    clip(text)
}

/// Keeps a raw body out of an error string past a readable size.
fn clip(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.chars().count() > 300 {
        format!("{}…", trimmed.chars().take(300).collect::<String>())
    } else {
        trimmed.to_string()
    }
}
