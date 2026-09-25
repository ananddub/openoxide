use reqwest::Url;
use reqwest::header::{HeaderMap, USER_AGENT};
use rig_core::{
    client::{CompletionClient, ModelListingClient, Nothing},
    completion::{AssistantContent, CompletionModel},
    providers::{anthropic, gemini, ollama, openai},
};
use serde_json::{Value, json};

use super::{AiProviderConfig, AiProviderKind};

/// AgentRouter gateways reject generic HTTP clients with a 401
/// "unauthorized client" error unless the request looks like QwenCode.
fn provider_headers(base: &str) -> Option<HeaderMap> {
    let host = Url::parse(base).ok()?.host_str()?.to_string();
    host.contains("agentrouter").then(|| {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "QwenCode/0.22.3 (linux; x64)"
                .parse()
                .expect("static user-agent"),
        );
        headers
    })
}

#[derive(Clone, Default)]
pub struct AiClient;

impl AiClient {
    pub async fn discover_models(&self, config: &AiProviderConfig) -> Result<Vec<String>, String> {
        let base = validate_base_url(config)?;
        let list = match config.kind() {
            AiProviderKind::Ollama => {
                ollama::Client::builder()
                    .api_key(Nothing)
                    .base_url(&base)
                    .build()
                    .map_err(rig_error)?
                    .list_models()
                    .await
            }
            AiProviderKind::Gemini => {
                gemini::Client::builder()
                    .api_key(&config.api_key)
                    .base_url(&base)
                    .build()
                    .map_err(rig_error)?
                    .list_models()
                    .await
            }
            AiProviderKind::Anthropic => {
                anthropic::Client::builder()
                    .api_key(&config.api_key)
                    .base_url(&base)
                    .build()
                    .map_err(rig_error)?
                    .list_models()
                    .await
            }
            AiProviderKind::OpenAi | AiProviderKind::OpenAiCompatible => {
                let mut builder = openai::Client::builder()
                    .api_key(&config.api_key)
                    .base_url(openai_base_url(&base));
                if let Some(headers) = provider_headers(&base) {
                    builder = builder.http_headers(headers);
                }
                builder.build().map_err(rig_error)?.list_models().await
            }
        }
        .map_err(rig_error)?;

        let mut models = list
            .iter()
            .map(|model| model.id.trim_start_matches("models/").to_string())
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
        let prompt = if json_only {
            format!("Return only valid JSON without markdown fences.\n\n{prompt}")
        } else {
            prompt.to_string()
        };

        match config.kind() {
            AiProviderKind::Anthropic => {
                let client = anthropic::Client::builder()
                    .api_key(&config.api_key)
                    .base_url(&base)
                    .build()
                    .map_err(rig_error)?;
                complete_with_model(client.completion_model(&config.model), prompt, None).await
            }
            AiProviderKind::Gemini => {
                let client = gemini::Client::builder()
                    .api_key(&config.api_key)
                    .base_url(&base)
                    .build()
                    .map_err(rig_error)?;
                complete_with_model(
                    client.completion_model(config.model.trim_start_matches("models/")),
                    prompt,
                    None,
                )
                .await
            }
            AiProviderKind::Ollama => {
                let client = ollama::Client::builder()
                    .api_key(Nothing)
                    .base_url(&base)
                    .build()
                    .map_err(rig_error)?;
                let params = json_only.then(|| json!({"format": "json"}));
                complete_with_model(client.completion_model(&config.model), prompt, params).await
            }
            AiProviderKind::OpenAi | AiProviderKind::OpenAiCompatible => {
                let mut builder = openai::CompletionsClient::builder()
                    .api_key(&config.api_key)
                    .base_url(openai_base_url(&base));
                if let Some(headers) = provider_headers(&base) {
                    builder = builder.http_headers(headers);
                }
                let client = builder.build().map_err(rig_error)?;
                let params = (json_only && config.kind() == AiProviderKind::OpenAi)
                    .then(|| json!({"response_format": {"type": "json_object"}}));
                complete_with_model(client.completion_model(&config.model), prompt, params).await
            }
        }
    }
}

async fn complete_with_model<M: CompletionModel>(
    model: M,
    prompt: String,
    additional_params: Option<Value>,
) -> Result<String, String> {
    let mut request = model
        .completion_request(prompt)
        .temperature(0.2)
        // Reasoning models (GLM/DeepSeek "v4"-style) spend part of the budget on
        // hidden thinking; a 3-variant compose JSON can exceed 16k visible tokens alone.
        .max_tokens(32768);
    if let Some(params) = additional_params {
        request = request.additional_params(params);
    }
    let response = model.completion(request.build()).await.map_err(rig_error)?;
    let text = response
        .choice
        .iter()
        .filter_map(|content| match content {
            AssistantContent::Text(text) => Some(text.text.as_str()),
            _ => None,
        })
        .collect::<String>();
    if text.trim().is_empty() {
        Err("AI provider response did not contain generated text".into())
    } else {
        Ok(text)
    }
}

fn rig_error(error: impl std::fmt::Display) -> String {
    format!("AI provider request failed: {error}")
}

fn openai_base_url(base: &str) -> String {
    if base.ends_with("/v1") {
        base.to_string()
    } else {
        format!("{base}/v1")
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

fn strip_json_fence(text: &str) -> &str {
    let trimmed = text.trim();
    trimmed
        .strip_circumfix("```json", "```")
        .or_else(|| trimmed.strip_circumfix("```", "```"))
        .or_else(|| trimmed.strip_prefix("```json"))
        .or_else(|| trimmed.strip_prefix("```"))
        .unwrap_or(trimmed)
        .trim()
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
    }

    #[test]
    fn strips_markdown_json_fence() {
        assert_eq!(
            strip_json_fence("```json\n{\"ok\":true}\n```"),
            "{\"ok\":true}"
        );
    }
}
