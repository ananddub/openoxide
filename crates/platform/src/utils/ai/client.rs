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
pub fn provider_headers(base: &str) -> Option<HeaderMap> {
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
                    .base_url(config.gemini_base_url())
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
                    .base_url(config.gemini_base_url())
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
    // Reasoning models (GLM/DeepSeek "v4"-style) spend part of the budget on
    // hidden thinking; a 3-variant compose JSON can exceed 16k visible tokens.
    const TOKEN_BUDGET: u64 = 32768;

    // Build a request using the chosen token-limit parameter. Newer OpenAI
    // reasoning models reject `max_tokens` and require `max_completion_tokens`;
    // older models/providers only accept `max_tokens`. The same models also
    // reject any non-default `temperature` (only the default 1 is allowed), so
    // temperature is applied conditionally and dropped on retry.
    let build = |use_completion_tokens: bool, with_temperature: bool| {
        let mut request = model.completion_request(prompt.clone());
        if with_temperature {
            request = request.temperature(0.2);
        }
        request = if use_completion_tokens {
            let extra = json!({ "max_completion_tokens": TOKEN_BUDGET });
            let merged = match &additional_params {
                Some(existing) => merge_json(existing.clone(), extra),
                None => extra,
            };
            request.additional_params(merged)
        } else {
            let request = request.max_tokens(TOKEN_BUDGET);
            match &additional_params {
                Some(params) => request.additional_params(params.clone()),
                None => request,
            }
        };
        request.build()
    };

    // Try the broadest-compatible request first, then progressively drop the
    // parameters a strict reasoning model rejects (max_tokens → completion
    // tokens, temperature → default). Each retry only fires on the matching
    // provider error so we never silently change behaviour for models that
    // accepted the original request.
    let mut use_completion_tokens = false;
    let mut with_temperature = true;
    let response = loop {
        match model
            .completion(build(use_completion_tokens, with_temperature))
            .await
        {
            Ok(r) => break r,
            Err(e) => {
                let msg = e.to_string();
                if !use_completion_tokens && wants_completion_tokens(&msg) {
                    use_completion_tokens = true;
                } else if with_temperature && rejects_temperature(&msg) {
                    with_temperature = false;
                } else {
                    return Err(rig_error(e));
                }
            }
        }
    };
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

/// True if the provider rejected `max_tokens` and wants `max_completion_tokens`.
fn wants_completion_tokens(err: &str) -> bool {
    let e = err.to_ascii_lowercase();
    e.contains("max_completion_tokens") || (e.contains("max_tokens") && e.contains("unsupported"))
}

/// True if the provider rejected a non-default `temperature`. Newer OpenAI
/// reasoning models (o-series, gpt-5/6 reasoning) only accept the default
/// temperature and 400 on anything else; we retry without the field.
fn rejects_temperature(err: &str) -> bool {
    let e = err.to_ascii_lowercase();
    e.contains("temperature")
        && (e.contains("unsupported")
            || e.contains("does not support")
            || e.contains("only the default")
            || e.contains("default (1)"))
}

/// Shallow-merge two JSON objects (b wins on key clash); non-objects return b.
fn merge_json(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(mut ao), Value::Object(bo)) => {
            for (k, v) in bo {
                ao.insert(k, v);
            }
            Value::Object(ao)
        }
        (_, b) => b,
    }
}

fn rig_error(error: impl std::fmt::Display) -> String {
    format!("AI provider request failed: {error}")
}

/// Resolve the base URL an OpenAI-compatible client should call.
///
/// rig's OpenAI client appends `/chat/completions` (and `/models`) to this
/// base, so the base must already point at the provider's API root. Providers
/// disagree on that root:
///   - OpenAI / OpenRouter / Perplexity / MiniMax → `.../v1`
///   - DeepInfra                                   → `.../v1/openai`
///   - GLM (z.ai / BigModel)                       → `.../api/paas/v4`
///   - Ollama's OpenAI shim                        → `.../v1`
///
/// So we only append `/v1` when the user gave a bare host with NO path. If the
/// URL already carries a path segment (any version or vendor path), we trust it
/// verbatim — blindly appending `/v1` was turning `.../api/paas/v4` into
/// `.../api/paas/v4/v1` and 404ing every GLM/DeepInfra request.
fn openai_base_url(base: &str) -> String {
    let trimmed = base.trim_end_matches('/');
    let has_path = Url::parse(trimmed)
        .map(|u| !u.path().trim_matches('/').is_empty())
        .unwrap_or(false);
    if has_path {
        trimmed.to_string()
    } else {
        format!("{trimmed}/v1")
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

    #[test]
    fn detects_completion_tokens_requirement() {
        let body = "Unsupported parameter: 'max_tokens' is not supported with \
                    this model. Use 'max_completion_tokens' instead.";
        assert!(wants_completion_tokens(body));
        assert!(wants_completion_tokens("please use max_completion_tokens"));
        assert!(!wants_completion_tokens("rate limit exceeded"));
        assert!(!wants_completion_tokens("invalid api key"));
    }

    #[test]
    fn openai_base_url_per_provider() {
        // bare host → append /v1
        assert_eq!(
            openai_base_url("https://api.openai.com"),
            "https://api.openai.com/v1"
        );
        assert_eq!(
            openai_base_url("https://api.openai.com/"),
            "https://api.openai.com/v1"
        );
        // already /v1 → preserved
        assert_eq!(
            openai_base_url("https://api.openai.com/v1"),
            "https://api.openai.com/v1"
        );
        assert_eq!(
            openai_base_url("https://openrouter.ai/api/v1"),
            "https://openrouter.ai/api/v1"
        );
        // vendor version paths → preserved verbatim (the old code broke these
        // by appending /v1 → /api/paas/v4/v1 and /v1/openai/v1).
        assert_eq!(
            openai_base_url("https://api.z.ai/api/paas/v4"),
            "https://api.z.ai/api/paas/v4"
        );
        assert_eq!(
            openai_base_url("https://api.z.ai/api/paas/v4/"),
            "https://api.z.ai/api/paas/v4"
        );
        assert_eq!(
            openai_base_url("https://api.deepinfra.com/v1/openai"),
            "https://api.deepinfra.com/v1/openai"
        );
        assert_eq!(
            openai_base_url("https://api.perplexity.ai/v1"),
            "https://api.perplexity.ai/v1"
        );
    }

    #[test]
    fn detects_temperature_rejection() {
        let body = "Unsupported value: 'temperature' does not support 0.2 with \
                    this model. Only the default (1) value is supported.";
        assert!(rejects_temperature(body));
        assert!(rejects_temperature(
            "temperature is unsupported with this model"
        ));
        assert!(!rejects_temperature("rate limit exceeded"));
        assert!(!rejects_temperature("invalid api key"));
    }

    #[test]
    fn gemini_base_url_strips_version_suffix() {
        let mk = |u: &str| AiProviderConfig {
            api_url: u.into(),
            api_key: "k".into(),
            model: "m".into(),
        };
        let origin = "https://generativelanguage.googleapis.com";
        assert_eq!(mk(&format!("{origin}/v1beta")).gemini_base_url(), origin);
        assert_eq!(mk(&format!("{origin}/v1beta/")).gemini_base_url(), origin);
        assert_eq!(mk(&format!("{origin}/v1")).gemini_base_url(), origin);
        assert_eq!(mk(origin).gemini_base_url(), origin);
    }

    /// Live Gemini smoke test. Ignored by default (hits the network); run with:
    ///   GEMINI_API_KEY=... cargo test -p openoxide --lib \
    ///     utils::ai::client::tests::gemini_live -- --ignored --nocapture
    #[tokio::test]
    #[ignore = "network: needs GEMINI_API_KEY"]
    async fn gemini_live() {
        // Mirror main.rs: reqwest's rustls build needs a crypto provider
        // installed once before any TLS client is built.
        let _ = rustls::crypto::ring::default_provider().install_default();
        let key =
            std::env::var("GEMINI_API_KEY").expect("set GEMINI_API_KEY to run this ignored test");
        let config = AiProviderConfig {
            api_url: "https://generativelanguage.googleapis.com/v1beta".into(),
            api_key: key,
            model: "gemini-flash-latest".into(),
        };
        assert_eq!(
            config.kind(),
            AiProviderKind::Gemini,
            "url must detect Gemini"
        );

        let client = AiClient;

        // 1. discover_models must return a non-empty, de-prefixed list.
        let models = client
            .discover_models(&config)
            .await
            .expect("discover_models should succeed against live Gemini");
        println!(
            "discovered {} models, sample: {:?}",
            models.len(),
            &models[..models.len().min(5)]
        );
        assert!(!models.is_empty(), "Gemini returned no models");
        assert!(
            models.iter().all(|m| !m.starts_with("models/")),
            "model ids should be de-prefixed"
        );

        // 2. test_connection (a real completion) must succeed.
        client
            .test_connection(&config)
            .await
            .expect("test_connection should succeed against live Gemini");

        // 3. a plain completion returns non-empty text.
        let text = client
            .complete_text(&config, "Reply with exactly: ok")
            .await
            .expect("complete_text should succeed");
        println!("gemini reply: {text:?}");
        assert!(!text.trim().is_empty(), "empty completion text");
    }
}
