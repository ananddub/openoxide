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

#[derive(Debug, Clone)]
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
}
