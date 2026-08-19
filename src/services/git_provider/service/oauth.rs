use crate::{
    services::git_provider::{CreateProvider, GitProviderView, ProviderCredentials},
    utils::provider::oauth::{AuthorizationInfo, OAuthConfig},
};

use super::GitProviderService;

impl GitProviderService {
    pub async fn create_github_from_manifest(
        &self,
        code: &str,
        installation_id: Option<&str>,
    ) -> Result<GitProviderView, String> {
        if code.trim().is_empty() {
            return Err("GitHub manifest code is required".into());
        }
        let response = reqwest::Client::new()
            .post(format!(
                "https://api.github.com/app-manifests/{code}/conversions"
            ))
            .header(reqwest::header::USER_AGENT, "rustploy")
            .send()
            .await
            .map_err(|e| format!("GitHub App conversion failed: {e}"))?;
        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Invalid GitHub App conversion response: {e}"))?;
        if !status.is_success() {
            return Err(format!(
                "GitHub App conversion failed (status {status}): {}",
                body["message"].as_str().unwrap_or("unknown error")
            ));
        }
        let app_name = body["name"]
            .as_str()
            .filter(|v| !v.trim().is_empty())
            .ok_or("GitHub conversion response did not include app name")?;
        let created = self
            .create(CreateProvider {
                name: app_name.to_string(),
                shared: true,
                credentials: ProviderCredentials::Github {
                    app_name: Some(app_name.to_string()),
                    app_id: body["id"].as_i64(),
                    client_id: body["client_id"].as_str().map(str::to_string),
                    client_secret: body["client_secret"].as_str().map(str::to_string),
                    installation_id: installation_id.map(str::to_string),
                    private_key: body["pem"].as_str().map(str::to_string),
                },
            })
            .await
            .map_err(|e| format!("Failed to save GitHub provider: {e}"))?;
        if created.0.configured || installation_id.is_none() {
            return Ok(created.0);
        }
        self.get(created.0.id).await.map_err(|e| e.to_string())
    }

    pub async fn authorization(&self, id: i64) -> Result<AuthorizationInfo, String> {
        use crate::utils::provider::GitProviderType;
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        let state = self.oauth_state(id).await?;
        let provider_kind: Option<GitProviderType> = provider.provider_type.parse().ok();
        match provider_kind {
            Some(GitProviderType::Github) => {
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
            Some(GitProviderType::Gitlab) => Ok(self
                .oauth
                .authorization_url(&self.gitlab_oauth_config(id).await?, &state)),
            Some(GitProviderType::Gitea) => Ok(self
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
        use crate::utils::provider::GitProviderType;
        self.verify_oauth_state(id, state).await?;
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        let provider_kind: Option<GitProviderType> = provider.provider_type.parse().ok();
        match provider_kind {
            Some(GitProviderType::Github) => {
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
            Some(GitProviderType::Gitlab) => {
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
            Some(GitProviderType::Gitea) => {
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
        use crate::utils::provider::GitProviderType;
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        let provider_kind: Option<GitProviderType> = provider.provider_type.parse().ok();
        let changed = match provider_kind {
            Some(GitProviderType::Github) => self.github.disconnect(id).await,
            Some(GitProviderType::Gitlab) => self.gitlab.disconnect(id).await,
            Some(GitProviderType::Gitea) => self.gitea.disconnect(id).await,
            Some(GitProviderType::Bitbucket) => self.bitbucket.disconnect(id).await,
            None => return Err("Unsupported Git provider".into()),
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
