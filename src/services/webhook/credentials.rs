use auto_di::singleton;
use std::sync::Arc;

use crate::repository::WebhookRepository;

use super::GitProviderKind;

#[derive(Debug, Clone)]
pub enum WebhookCredential {
    Hmac(String),
    Token(String),
}

pub struct WebhookCredentialStore {
    repository: Arc<WebhookRepository>,
}

#[singleton]
impl WebhookCredentialStore {
    fn new(repository: Arc<WebhookRepository>) -> Self {
        Self { repository }
    }

    pub async fn load(
        &self,
        provider: GitProviderKind,
        provider_id: i64,
    ) -> Result<WebhookCredential, String> {
        let secret = self
            .repository
            .credential_secret(provider, provider_id)
            .await
            .map_err(|error| error.to_string())?
            .filter(|secret| !secret.is_empty())
            .ok_or_else(|| "webhook provider secret is not configured".to_string())?;

        Ok(match provider {
            GitProviderKind::Gitlab => WebhookCredential::Token(secret),
            _ => WebhookCredential::Hmac(secret),
        })
    }
}
