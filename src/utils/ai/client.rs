use reqwest::Url;
use rig_core::{
    client::{CompletionClient, ModelListingClient, Nothing},
    completion::{AssistantContent, CompletionModel},
    providers::{anthropic, gemini, ollama, openai},
};
use serde_json::{Value, json};

use super::{AiProviderConfig, AiProviderKind};

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
                openai::Client::builder()
                    .api_key(&config.api_key)
                    .base_url(openai_base_url(&base))
                    .build()
                    .map_err(rig_error)?
                    .list_models()
                    .await
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
                let client = openai::CompletionsClient::builder()
                    .api_key(&config.api_key)
                    .base_url(openai_base_url(&base))
                    .build()
                    .map_err(rig_error)?;
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
        .max_tokens(8192);
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

fn join_api_path(base: &str, path: &str) -> String {
    if base.ends_with("/v1") || base.ends_with("/v1beta") {
        format!("{base}/{path}")
    } else {
        format!("{base}/v1/{path}")
    }
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
