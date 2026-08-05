use crate::{
    services::git_provider::GitProviderView,
    utils::provider::oauth::{AuthorizationInfo, OAuthConfig},
};

use super::GitProviderService;

impl GitProviderService {
    pub async fn authorization(&self, id: i64) -> Result<AuthorizationInfo, String> {
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        let state = self.oauth_state(id).await?;
        match provider.provider_type.as_str() {
            "GITHUB" => {
                let app_name = self
                    .github_item(id)
                    .await?
                    .github_app_name
                    .filter(|value| !value.trim().is_empty())
                    .ok_or("GitHub App name is required")?;
                Ok(AuthorizationInfo {
                    url: format!(
                        "https://github.com/apps/{}/installations/new?state={}",
                        crate::utils::provider::oauth::encode(&app_name),
                        crate::utils::provider::oauth::encode(&state)
                    ),
                    state,
                })
            }
            "GITLAB" => Ok(self
                .oauth
                .authorization_url(&self.gitlab_oauth_config(id).await?, &state)),
            "GITEA" => Ok(self
                .oauth
                .authorization_url(&self.gitea_oauth_config(id).await?, &state)),
            _ => Err("This provider does not support browser authorization".into()),
        }
    }

    pub async fn complete_authorization(
        &self,
        id: i64,
        state: &str,
        code: Option<&str>,
        installation_id: Option<&str>,
    ) -> Result<GitProviderView, String> {
        self.verify_oauth_state(id, state).await?;
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        match provider.provider_type.as_str() {
            "GITHUB" => {
                let installation_id = installation_id
                    .filter(|value| !value.trim().is_empty())
                    .ok_or("GitHub installation ID is required")?;
                if !self
                    .github
                    .set_installation_id(id, installation_id)
                    .await
                    .map_err(|error| error.to_string())?
                {
                    return Err("GitHub configuration not found".into());
                }
            }
            "GITLAB" => {
                let tokens = self
                    .oauth
                    .exchange_code(
                        &self.gitlab_oauth_config(id).await?,
                        code.ok_or("OAuth authorization code is required")?,
                    )
                    .await?;
                self.gitlab
                    .set_oauth_tokens(
                        id,
                        &tokens.access_token,
                        tokens.refresh_token.as_deref(),
                        tokens.expires_at,
                    )
                    .await
                    .map_err(|error| error.to_string())?;
            }
            "GITEA" => {
                let tokens = self
                    .oauth
                    .exchange_code(
                        &self.gitea_oauth_config(id).await?,
                        code.ok_or("OAuth authorization code is required")?,
                    )
                    .await?;
                self.gitea
                    .set_oauth_tokens(
                        id,
                        &tokens.access_token,
                        tokens.refresh_token.as_deref(),
                        tokens.expires_at,
                        tokens.scopes.as_deref(),
                    )
                    .await
                    .map_err(|error| error.to_string())?;
            }
            _ => return Err("This provider does not support browser authorization".into()),
        }
        self.get(id).await.map_err(|error| error.to_string())
    }

    pub async fn disconnect(&self, id: i64) -> Result<(), String> {
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        let changed = match provider.provider_type.as_str() {
            "GITHUB" => self.github.disconnect(id).await,
            "GITLAB" => self.gitlab.disconnect(id).await,
            "GITEA" => self.gitea.disconnect(id).await,
            "BITBUCKET" => self.bitbucket.disconnect(id).await,
            _ => return Err("Unsupported Git provider".into()),
        }
        .map_err(|error| error.to_string())?;
        changed
            .then_some(())
            .ok_or("Provider configuration not found".into())
    }

    async fn gitlab_oauth_config(&self, id: i64) -> Result<OAuthConfig, String> {
        let item = self
            .gitlab
            .get_by_git_provider_id(id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("GitLab configuration not found")?;
        Ok(OAuthConfig {
            base_url: item.gitlab_url,
            client_id: item
                .application_id
                .ok_or("GitLab application ID is required")?,
            client_secret: item.secret.ok_or("GitLab application secret is required")?,
            redirect_uri: item.redirect_uri.ok_or("GitLab redirect URI is required")?,
            scopes: "read_user read_repository api".into(),
        })
    }

    async fn gitea_oauth_config(&self, id: i64) -> Result<OAuthConfig, String> {
        let item = self
            .gitea
            .get_by_git_provider_id(id)
            .await
            .map_err(|error| error.to_string())?
            .ok_or("Gitea configuration not found")?;
        Ok(OAuthConfig {
            base_url: item.gitea_url,
            client_id: item.client_id.ok_or("Gitea client ID is required")?,
            client_secret: item
                .client_secret
                .ok_or("Gitea client secret is required")?,
            redirect_uri: item.redirect_uri.ok_or("Gitea redirect URI is required")?,
            scopes: item.scopes.unwrap_or_else(|| "read:user repo".into()),
        })
    }

    async fn oauth_state(&self, id: i64) -> Result<String, String> {
        let payload = format!("{id}.{}", chrono::Utc::now().timestamp() + 600);
        let signature = crate::services::webhook::sign_hmac_sha256(
            self.webhook_secret(id).await?.as_bytes(),
            payload.as_bytes(),
        );
        Ok(format!("{payload}.{signature}"))
    }

    async fn verify_oauth_state(&self, id: i64, state: &str) -> Result<(), String> {
        let (payload, signature) = state.rsplit_once('.').ok_or("Invalid OAuth state")?;
        let (state_id, expires) = payload.split_once('.').ok_or("Invalid OAuth state")?;
        if state_id.parse::<i64>().ok() != Some(id)
            || expires.parse::<i64>().unwrap_or_default() < chrono::Utc::now().timestamp()
        {
            return Err("OAuth state is invalid or expired".into());
        }
        crate::services::webhook::verify_hmac_sha256(
            self.webhook_secret(id).await?.as_bytes(),
            payload.as_bytes(),
            signature,
        )
        .then_some(())
        .ok_or("Invalid OAuth state signature".into())
    }
}
