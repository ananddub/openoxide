mod discovery;
mod mutations;
mod oauth;
mod webhooks;

use auto_di::singleton;
use std::sync::Arc;

use crate::{
    db::models::{git_providers::GitProvider, github_providers::GithubProvider},
    repository::{
        BitbucketProviderRepository, GitProviderRepository, GiteaProviderRepository,
        GithubProviderRepository, GitlabProviderRepository,
    },
    utils::provider::{discovery::ProviderDiscovery, oauth::OAuthClient},
};

use super::{GitProviderConfigView, GitProviderView};

pub struct GitProviderService {
    pub(super) providers: Arc<GitProviderRepository>,
    pub(super) github: Arc<GithubProviderRepository>,
    pub(super) gitlab: Arc<GitlabProviderRepository>,
    pub(super) gitea: Arc<GiteaProviderRepository>,
    pub(super) bitbucket: Arc<BitbucketProviderRepository>,
    pub(super) discovery: ProviderDiscovery,
    pub(super) oauth: OAuthClient,
}

#[singleton]
impl GitProviderService {
    fn new(
        providers: Arc<GitProviderRepository>,
        github: Arc<GithubProviderRepository>,
        gitlab: Arc<GitlabProviderRepository>,
        gitea: Arc<GiteaProviderRepository>,
        bitbucket: Arc<BitbucketProviderRepository>,
    ) -> Self {
        Self {
            providers,
            github,
            gitlab,
            gitea,
            bitbucket,
            discovery: ProviderDiscovery::default(),
            oauth: OAuthClient::default(),
        }
    }

    pub async fn list(&self) -> sqlx::Result<Vec<GitProviderView>> {
        let rows = self.providers.get_all().await?;
        let mut output = Vec::with_capacity(rows.len());
        for row in rows {
            output.push(self.view(row).await?);
        }
        Ok(output)
    }

    pub async fn get(&self, id: i64) -> sqlx::Result<GitProviderView> {
        let row = self
            .providers
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        self.view(row).await
    }

    pub(super) async fn view(&self, row: GitProvider) -> sqlx::Result<GitProviderView> {
        let id = row
            .id
            .as_deref()
            .and_then(|id| id.parse().ok())
            .unwrap_or_default();
        use crate::utils::provider::GitProviderType;
        let provider_kind: Option<GitProviderType> = row.provider_type.parse().ok();
        let (configured, config) = match provider_kind {
            Some(GitProviderType::Github) => {
                let item = self.github.get_by_git_provider_id(id).await?;
                let configured = item.as_ref().is_some_and(|item| {
                    item.github_app_id.is_some()
                        && item.github_installation_id.is_some()
                        && item.github_private_key.is_some()
                });
                let config = item
                    .map(|item| GitProviderConfigView {
                        app_name: item.github_app_name,
                        app_id: item.github_app_id,
                        client_id: item.github_client_id,
                        installation_id: item.github_installation_id,
                        has_client_secret: item.github_client_secret.is_some(),
                        has_private_key: item.github_private_key.is_some(),
                        ..Default::default()
                    })
                    .unwrap_or_default();
                (configured, config)
            }
            Some(GitProviderType::Gitlab) => {
                let item = self.gitlab.get_by_git_provider_id(id).await?;
                let configured = item
                    .as_ref()
                    .is_some_and(|item| item.access_token.is_some());
                let config = item
                    .map(|item| GitProviderConfigView {
                        url: Some(item.gitlab_url),
                        internal_url: item.gitlab_internal_url,
                        application_id: item.application_id,
                        redirect_uri: item.redirect_uri,
                        group_name: item.group_name,
                        has_client_secret: item.secret.is_some(),
                        has_access_token: item.access_token.is_some(),
                        ..Default::default()
                    })
                    .unwrap_or_default();
                (configured, config)
            }
            Some(GitProviderType::Gitea) => {
                let item = self.gitea.get_by_git_provider_id(id).await?;
                let configured = item
                    .as_ref()
                    .is_some_and(|item| item.access_token.is_some());
                let config = item
                    .map(|item| GitProviderConfigView {
                        url: Some(item.gitea_url),
                        internal_url: item.gitea_internal_url,
                        redirect_uri: item.redirect_uri,
                        client_id: item.client_id,
                        scopes: item.scopes,
                        has_client_secret: item.client_secret.is_some(),
                        has_access_token: item.access_token.is_some(),
                        ..Default::default()
                    })
                    .unwrap_or_default();
                (configured, config)
            }
            Some(GitProviderType::Bitbucket) => {
                let item = self.bitbucket.get_by_git_provider_id(id).await?;
                let configured = item
                    .as_ref()
                    .is_some_and(|item| item.api_token.is_some() || item.app_password.is_some());
                let config = item
                    .map(|item| GitProviderConfigView {
                        username: item.bitbucket_username,
                        email: item.bitbucket_email,
                        workspace: item.bitbucket_workspace_name,
                        has_app_password: item.app_password.is_some(),
                        has_api_token: item.api_token.is_some(),
                        ..Default::default()
                    })
                    .unwrap_or_default();
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
            webhook_configured: self.providers.has_webhook_secret(id).await?,
            created_at: row.created_at,
            updated_at: row.updated_at,
            config,
        })
    }

    pub(super) async fn github_item(&self, id: i64) -> Result<GithubProvider, String> {
        self.github
            .get_by_git_provider_id(id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("GitHub configuration not found".into())
    }
}
