//! Agentic assistant: turns a natural-language request into permission-checked actions by driving a rig tool-loop whose `invoke` tool calls the panel's own Connect endpoints.

pub mod catalog;
pub mod invoke;
pub mod runner;
pub mod session;
pub mod templates;

pub use invoke::UserContext;
pub use runner::{AgentEvent, AgentReply, ChoiceOption, Interaction, InteractionKind};

use rig_core::client::CompletionClient;
use rig_core::providers::{anthropic, gemini, ollama, openai};

use crate::utils::ai::{AiProviderConfig, AiProviderKind, provider_headers};

/// Build the per-provider completion model and run the assistant loop under the caller's permission context.
pub async fn run_assistant(
    config: &AiProviderConfig,
    ctx: &UserContext,
    history: &[(String, String)],
    pending_actions: &str,
    prompt: &str,
    tx: Option<tokio::sync::mpsc::Sender<runner::AgentEvent>>,
) -> Result<AgentReply, String> {
    let base = config.normalized_url();
    match config.kind() {
        AiProviderKind::Anthropic => {
            let client = anthropic::Client::builder()
                .api_key(&config.api_key)
                .base_url(&base)
                .build()
                .map_err(err)?;
            runner::run(
                client.completion_model(&config.model),
                ctx,
                history,
                pending_actions,
                prompt,
                tx.clone(),
            )
            .await
        }
        AiProviderKind::Gemini => {
            let client = gemini::Client::builder()
                .api_key(&config.api_key)
                .base_url(&config.gemini_base_url())
                .build()
                .map_err(err)?;
            runner::run(
                client.completion_model(config.model.trim_start_matches("models/")),
                ctx,
                history,
                pending_actions,
                prompt,
                tx.clone(),
            )
            .await
        }
        AiProviderKind::Ollama => {
            let client = ollama::Client::builder()
                .api_key(rig_core::client::Nothing)
                .base_url(&base)
                .build()
                .map_err(err)?;
            runner::run(
                client.completion_model(&config.model),
                ctx,
                history,
                pending_actions,
                prompt,
                tx.clone(),
            )
            .await
        }
        AiProviderKind::OpenAi | AiProviderKind::OpenAiCompatible => {
            let base_url = if base.ends_with("/v1") {
                base.clone()
            } else {
                format!("{base}/v1")
            };
            let mut builder = openai::CompletionsClient::builder()
                .api_key(&config.api_key)
                .base_url(base_url);
            // AgentRouter-style gateways 401 generic clients unless the request carries the QwenCode User-Agent.
            if let Some(headers) = provider_headers(&base) {
                builder = builder.http_headers(headers);
            }
            let client = builder.build().map_err(err)?;
            runner::run(
                client.completion_model(&config.model),
                ctx,
                history,
                pending_actions,
                prompt,
                tx.clone(),
            )
            .await
        }
    }
}

fn err(e: impl std::fmt::Display) -> String {
    format!("AI provider setup failed: {e}")
}
