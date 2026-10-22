use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiProviderKind {
    OpenAi,
    Anthropic,
    Gemini,
    Ollama,
    OpenAiCompatible,
}

impl AiProviderKind {
    pub fn detect(api_url: &str) -> Self {
        let url = api_url.to_ascii_lowercase();
        if url.contains("anthropic.com") {
            Self::Anthropic
        } else if url.contains("generativelanguage.googleapis.com") {
            Self::Gemini
        } else if url.contains("ollama") || url.contains(":11434") {
            Self::Ollama
        } else if url.contains("api.openai.com") {
            Self::OpenAi
        } else {
            Self::OpenAiCompatible
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AiProviderConfig {
    pub api_url: String,
    pub api_key: String,
    pub model: String,
}

impl AiProviderConfig {
    pub fn kind(&self) -> AiProviderKind {
        AiProviderKind::detect(&self.api_url)
    }

    pub fn normalized_url(&self) -> String {
        self.api_url.trim().trim_end_matches('/').to_string()
    }

    /// Base URL for the rig Gemini client. rig's Gemini provider prepends the
    /// full `/v1beta/...` path itself, so the base must be the bare origin —
    /// otherwise a configured `.../v1beta` URL becomes `.../v1beta/v1beta/...`
    /// and every request 404s.
    pub fn gemini_base_url(&self) -> String {
        let base = self.normalized_url();
        base.strip_suffix("/v1beta")
            .or_else(|| base.strip_suffix("/v1"))
            .unwrap_or(&base)
            .trim_end_matches('/')
            .to_string()
    }
}
