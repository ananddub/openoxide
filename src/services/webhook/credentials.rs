use auto_di::singleton;
use std::sync::Arc;

use crate::db::queries::FlashQueries;

use super::GitProviderKind;

#[derive(Debug, Clone)]
pub enum WebhookCredential {
    Hmac(String),
    Token(String),
}

pub struct WebhookCredentialStore {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl WebhookCredentialStore {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn load(
        &self,
        provider: GitProviderKind,
        provider_id: i64,
    ) -> Result<WebhookCredential, String> {
        let secret = match provider {
            GitProviderKind::Github => self.queries.webhooks_credential_secret1(provider_id).await,
            GitProviderKind::Gitlab => self.queries.webhooks_credential_secret2(provider_id).await,
            GitProviderKind::Gitea => self.queries.webhooks_credential_secret3(provider_id).await,
            GitProviderKind::Bitbucket => self.queries.webhooks_credential_secret4(provider_id).await,
        }
        .map_err(|error| error.to_string())?
        .filter(|secret| !secret.is_empty())
        .ok_or_else(|| "webhook provider secret is not configured".to_string())?;

        Ok(match provider {
            GitProviderKind::Gitlab => WebhookCredential::Token(secret),
            _ => WebhookCredential::Hmac(secret),
        })
    }
}
