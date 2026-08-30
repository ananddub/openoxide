mod discovery;
mod mutations;
mod oauth;
mod webhooks;

use auto_di::singleton;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    db::queries::FlashQueries,
    flash_gen::models::GitProviders,
    utils::provider::{discovery::ProviderDiscovery, oauth::OAuthClient},
};

use super::{GitProviderConfigView, GitProviderView};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct GithubConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct GitlabConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct GiteaConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authenticated_at: Option<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct BitbucketConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_name: Option<String>,
}

pub(super) fn parse_config<T: serde::de::DeserializeOwned>(config_json: &str) -> sqlx::Result<T> {
    serde_json::from_str(config_json)
        .map_err(|error| sqlx::Error::Protocol(format!("invalid git provider config: {error}")))
}

pub(super) fn config_to_string<T: Serialize>(config: &T) -> sqlx::Result<String> {
    serde_json::to_string(config)
        .map_err(|error| sqlx::Error::Protocol(format!("invalid git provider config: {error}")))
}

pub struct GitProviderService {
    pub(super) queries: Arc<FlashQueries>,
    pub(super) discovery: ProviderDiscovery,
    pub(super) oauth: OAuthClient,
}

#[singleton]
impl GitProviderService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self {
            queries,
            discovery: ProviderDiscovery::default(),
            oauth: OAuthClient::default(),
        }
    }

    pub async fn list(&self) -> sqlx::Result<Vec<GitProviderView>> {
        let rows = self.queries.list_git_providers().await?;
        let mut output = Vec::with_capacity(rows.len());
        for row in rows {
            output.push(self.view(row).await?);
        }
        Ok(output)
    }

    pub async fn get(&self, id: i64) -> sqlx::Result<GitProviderView> {
        let row = self.queries.get_git_provider_by_i_d(id).await?;
        self.view(row).await
    }

    pub(super) async fn view(&self, row: GitProviders) -> sqlx::Result<GitProviderView> {
        use crate::utils::provider::GitProviderType;
        let id = row.id;
        let provider_kind: Option<GitProviderType> = row.provider_type.parse().ok();
        let (configured, config) = match provider_kind {
            Some(GitProviderType::Github) => {
                let item: GithubConfig = parse_config(&row.config_json)?;
                let configured = item.app_id.is_some()
                    && item.installation_id.is_some()
                    && item.private_key.is_some();
                let config = GitProviderConfigView {
                    app_name: item.app_name,
                    app_id: item.app_id,
                    client_id: item.client_id,
                    installation_id: item.installation_id,
                    has_client_secret: item.client_secret.is_some(),
                    has_private_key: item.private_key.is_some(),
                    ..Default::default()
                };
                (configured, config)
            }
            Some(GitProviderType::Gitlab) => {
                let item: GitlabConfig = parse_config(&row.config_json)?;
                let configured = item.access_token.is_some();
                let config = GitProviderConfigView {
                    url: item.url,
                    internal_url: item.internal_url,
                    application_id: item.application_id,
                    redirect_uri: item.redirect_uri,
                    group_name: item.group_name,
                    has_client_secret: item.secret.is_some(),
                    has_access_token: item.access_token.is_some(),
                    ..Default::default()
                };
                (configured, config)
            }
            Some(GitProviderType::Gitea) => {
                let item: GiteaConfig = parse_config(&row.config_json)?;
                let configured = item.access_token.is_some();
                let config = GitProviderConfigView {
                    url: item.url,
                    internal_url: item.internal_url,
                    redirect_uri: item.redirect_uri,
                    client_id: item.client_id,
                    scopes: item.scopes,
                    has_client_secret: item.client_secret.is_some(),
                    has_access_token: item.access_token.is_some(),
                    ..Default::default()
                };
                (configured, config)
            }
            Some(GitProviderType::Bitbucket) => {
                let item: BitbucketConfig = parse_config(&row.config_json)?;
                let configured = item.api_token.is_some() || item.app_password.is_some();
                let config = GitProviderConfigView {
                    username: item.username,
                    email: item.email,
                    workspace: item.workspace_name,
                    has_app_password: item.app_password.is_some(),
                    has_api_token: item.api_token.is_some(),
                    ..Default::default()
                };
                (configured, config)
            }
            None => (false, GitProviderConfigView::default()),
        };
        Ok(GitProviderView {
            id,
            name: row.name,
            provider_type: row.provider_type,
            shared: row.shared != 0,
            configured,
            webhook_configured: self.queries.git_providers_has_webhook_secret(id).await? > 0,
            created_at: row.created_at,
            updated_at: row.updated_at,
            config,
        })
    }
}
