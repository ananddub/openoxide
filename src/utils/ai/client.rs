use reqwest::{Client, Response, Url};
use serde_json::{Value, json};
use std::time::Duration;

use super::{AiProviderConfig, AiProviderKind};

#[derive(Clone)]
pub struct AiClient {
    http: Client,
}

impl Default for AiClient {
    fn default() -> Self {
        Self {
            http: Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(90))
                .build()
                .expect("AI HTTP client configuration is valid"),
        }
    }
}

impl AiClient {
    pub async fn discover_models(&self, config: &AiProviderConfig) -> Result<Vec<String>, String> {
        let base = validate_base_url(config)?;
        let response = match config.kind() {
            AiProviderKind::Ollama => self.http.get(format!("{base}/api/tags")).send().await,
            AiProviderKind::Gemini => {
                self.http
                    .get(format!("{base}/models"))
                    .query(&[("key", config.api_key.as_str())])
                    .send()
                    .await
            }
            AiProviderKind::Anthropic => {
                self.http
                    .get(join_api_path(&base, "models"))
                    .header("x-api-key", &config.api_key)
                    .header("anthropic-version", "2023-06-01")
                    .send()
                    .await
            }
            AiProviderKind::OpenAi | AiProviderKind::OpenAiCompatible => {
                self.http
                    .get(join_api_path(&base, "models"))
                    .bearer_auth(&config.api_key)
                    .send()
                    .await
            }
        }
        .map_err(|error| format!("AI provider request failed: {error}"))?;

        let value = response_json(response).await?;
        let values = value
            .get("data")
            .or_else(|| value.get("models"))
            .and_then(Value::as_array)
            .or_else(|| value.as_array())
            .ok_or_else(|| "AI provider returned an unsupported models response".to_string())?;

        let mut models = values
            .iter()
            .filter_map(|model| {
                model
                    .get("id")
                    .or_else(|| model.get("name"))
                    .or_else(|| model.get("model"))
                    .and_then(Value::as_str)
                    .map(|name| name.trim_start_matches("models/").to_string())
            })
            .collect::<Vec<_>>();
        models.sort();
        models.dedup();
        Ok(models)
    }

    pub async fn test_connection(&self, config: &AiProviderConfig) -> Result<(), String> {
        let response = self
            .complete(config, "Reply with exactly: ok", false)
            .await?;
        if response.trim().is_empty() {
            Err("AI provider returned an empty response".into())
        } else {
            Ok(())
        }
    }

    pub async fn complete_text(
        &self,
        config: &AiProviderConfig,
        prompt: &str,
    ) -> Result<String, String> {
        self.complete(config, prompt, false).await
    }

    pub async fn complete_json(
        &self,
        config: &AiProviderConfig,
        prompt: &str,
    ) -> Result<Value, String> {
        let text = self.complete(config, prompt, true).await?;
        serde_json::from_str(strip_json_fence(&text))
            .map_err(|error| format!("AI provider returned invalid JSON: {error}"))
    }

    async fn complete(
        &self,
        config: &AiProviderConfig,
        prompt: &str,
        json_only: bool,
    ) -> Result<String, String> {
        let base = validate_base_url(config)?;
        let response = match config.kind() {
            AiProviderKind::Anthropic => {
                self.http
                    .post(join_api_path(&base, "messages"))
                    .header("x-api-key", &config.api_key)
                    .header("anthropic-version", "2023-06-01")
                    .json(&json!({
                        "model": config.model,
                        "max_tokens": 8192,
                        "messages": [{"role": "user", "content": prompt}]
                    }))
                    .send()
                    .await
            }
            AiProviderKind::Gemini => {
                let model = config.model.trim_start_matches("models/");
                self.http
                    .post(format!("{base}/models/{model}:generateContent"))
                    .query(&[("key", config.api_key.as_str())])
                    .json(&json!({"contents": [{"parts": [{"text": prompt}]}]}))
                    .send()
                    .await
            }
            AiProviderKind::Ollama => {
                let mut body = json!({
                    "model": config.model,
                    "stream": false,
                    "messages": [{"role": "user", "content": prompt}]
                });
                if json_only {
                    body["format"] = Value::String("json".into());
                }
                self.http
                    .post(format!("{base}/api/chat"))
                    .json(&body)
                    .send()
                    .await
            }
            AiProviderKind::OpenAi | AiProviderKind::OpenAiCompatible => {
                let mut body = json!({
                    "model": config.model,
                    "messages": [{"role": "user", "content": prompt}],
                    "temperature": 0.2
                });
                if json_only && config.kind() == AiProviderKind::OpenAi {
                    body["response_format"] = json!({"type": "json_object"});
                }
                self.http
                    .post(join_api_path(&base, "chat/completions"))
                    .bearer_auth(&config.api_key)
                    .json(&body)
                    .send()
                    .await
            }
        }
        .map_err(|error| format!("AI provider request failed: {error}"))?;

        let value = response_json(response).await?;
        extract_text(config.kind(), &value)
    }
}

fn validate_base_url(config: &AiProviderConfig) -> Result<String, String> {
    let base = config.normalized_url();
    let parsed = Url::parse(&base).map_err(|error| format!("invalid AI API URL: {error}"))?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err("AI API URL must use http or https".into());
    }
    if config.kind() != AiProviderKind::Ollama && config.api_key.trim().is_empty() {
        return Err("API key is required for this AI provider".into());
    }
    Ok(base)
}

fn join_api_path(base: &str, path: &str) -> String {
    if base.ends_with("/v1") || base.ends_with("/v1beta") {
        format!("{base}/{path}")
    } else {
        format!("{base}/v1/{path}")
    }
}

async fn response_json(response: Response) -> Result<Value, String> {
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| format!("could not read AI provider response: {error}"))?;
    if !status.is_success() {
        let short = body.chars().take(2_000).collect::<String>();
        return Err(format!("AI provider returned {status}: {short}"));
    }
    serde_json::from_str(&body)
        .map_err(|error| format!("AI provider returned invalid response JSON: {error}"))
}

fn extract_text(kind: AiProviderKind, value: &Value) -> Result<String, String> {
    let text = match kind {
        AiProviderKind::Anthropic => value.pointer("/content/0/text").and_then(Value::as_str),
        AiProviderKind::Gemini => value
            .pointer("/candidates/0/content/parts/0/text")
            .and_then(Value::as_str),
        AiProviderKind::Ollama => value.pointer("/message/content").and_then(Value::as_str),
        AiProviderKind::OpenAi | AiProviderKind::OpenAiCompatible => value
            .pointer("/choices/0/message/content")
            .and_then(Value::as_str),
    };
    text.map(str::to_string)
        .ok_or_else(|| "AI provider response did not contain generated text".into())
}

fn strip_json_fence(text: &str) -> &str {
    let trimmed = text.trim();
    if let Some(without_open) = trimmed.strip_prefix("```json") {
        return without_open
            .strip_suffix("```")
            .unwrap_or(without_open)
            .trim();
    }
    if let Some(without_open) = trimmed.strip_prefix("```") {
        return without_open
            .strip_suffix("```")
            .unwrap_or(without_open)
            .trim();
    }
    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_provider_and_builds_versioned_url() {
        assert_eq!(
            AiProviderKind::detect("https://api.anthropic.com/v1"),
            AiProviderKind::Anthropic
        );
        assert_eq!(
            join_api_path("https://api.openai.com/v1", "models"),
            "https://api.openai.com/v1/models"
        );
    }

    #[test]
    fn strips_markdown_json_fence() {
        assert_eq!(
            strip_json_fence("```json\n{\"ok\":true}\n```"),
            "{\"ok\":true}"
        );
    }
}
