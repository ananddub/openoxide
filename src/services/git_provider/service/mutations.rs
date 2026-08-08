use getrandom::fill;

use crate::db::models::{
    bitbucket_providers::BitbucketProvider, git_providers::GitProvider,
    gitea_providers::GiteaProvider, github_providers::GithubProvider,
    gitlab_providers::GitlabProvider,
};

use crate::services::git_provider::{
    CreateProvider, GitProviderView, ProviderCredentials, UpdateProvider,
};

use super::GitProviderService;

impl GitProviderService {
    pub async fn create(&self, input: CreateProvider) -> sqlx::Result<(GitProviderView, String)> {
        let now = chrono::Utc::now().timestamp();
        let provider_id = self
            .providers
            .create(&GitProvider {
                id: None,
                name: input.name,
                provider_type: provider_type(&input.credentials).to_string(),
                shared: i64::from(input.shared),
                created_at: now,
                updated_at: now,
            })
            .await?;
        if let Err(error) = self.create_child(provider_id, input.credentials, now).await {
            let _ = self.providers.delete(provider_id).await;
            return Err(error);
        }
        let secret = generate_secret().map_err(sqlx::Error::Protocol)?;
        self.providers
            .rotate_webhook_secret(provider_id, &secret)
            .await?;
        Ok((self.get(provider_id).await?, secret))
    }

    pub async fn update(&self, id: i64, input: UpdateProvider) -> sqlx::Result<GitProviderView> {
        let current = self
            .providers
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if current.provider_type != provider_type(&input.credentials).as_str() {
            return Err(sqlx::Error::Protocol(
                "Git provider type cannot be changed".into(),
            ));
        }
        self.update_child(id, input.credentials).await?;
        self.providers
            .update(
                id,
                &GitProvider {
                    id: current.id,
                    name: input.name,
                    provider_type: current.provider_type,
                    shared: i64::from(input.shared.unwrap_or(current.shared != 0)),
                    created_at: current.created_at,
                    updated_at: chrono::Utc::now().timestamp(),
                },
            )
            .await?;
        self.get(id).await
    }

    pub async fn rotate_secret(&self, id: i64) -> sqlx::Result<String> {
        self.get(id).await?;
        let secret = generate_secret().map_err(sqlx::Error::Protocol)?;
        self.providers.rotate_webhook_secret(id, &secret).await?;
        Ok(secret)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get(id).await?;
        if !self.providers.delete_if_unused(id).await? {
            return Err(sqlx::Error::Protocol(
                "Git provider is assigned to an application or compose project".into(),
            ));
        }
        Ok(())
    }

    async fn create_child(
        &self,
        id: i64,
        credentials: ProviderCredentials,
        now: i64,
    ) -> sqlx::Result<()> {
        match credentials {
            ProviderCredentials::Github {
                app_name,
                app_id,
                client_id,
                client_secret,
                installation_id,
                private_key,
            } => self
                .github
                .create(&GithubProvider {
                    id: None,
                    github_app_name: app_name,
                    github_app_id: app_id,
                    github_client_id: client_id,
                    github_client_secret: client_secret,
                    github_installation_id: installation_id,
                    github_private_key: private_key,
                    github_webhook_secret: None,
                    git_provider_id: id,
                    created_at: now,
                    updated_at: now,
                })
                .await
                .map(|_| ()),
            ProviderCredentials::Gitlab {
                url,
                internal_url,
                application_id,
                redirect_uri,
                secret,
                access_token,
                refresh_token,
                group_name,
            } => self
                .gitlab
                .create(&GitlabProvider {
                    id: None,
                    gitlab_url: url,
                    gitlab_internal_url: internal_url,
                    application_id,
                    redirect_uri,
                    secret,
                    access_token,
                    refresh_token,
                    group_name,
                    expires_at: None,
                    git_provider_id: id,
                    created_at: now,
                    updated_at: now,
                })
                .await
                .map(|_| ()),
            ProviderCredentials::Gitea {
                url,
                internal_url,
                redirect_uri,
                client_id,
                client_secret,
                access_token,
                refresh_token,
                scopes,
            } => self
                .gitea
                .create(&GiteaProvider {
                    id: None,
                    gitea_url: url,
                    gitea_internal_url: internal_url,
                    redirect_uri,
                    client_id,
                    client_secret,
                    access_token,
                    refresh_token,
                    expires_at: None,
                    scopes,
                    last_authenticated_at: None,
                    git_provider_id: id,
                    created_at: now,
                    updated_at: now,
                })
                .await
                .map(|_| ()),
            ProviderCredentials::Bitbucket {
                username,
                email,
                app_password,
                api_token,
                workspace,
            } => self
                .bitbucket
                .create(&BitbucketProvider {
                    id: None,
                    bitbucket_username: username,
                    bitbucket_email: email,
                    app_password,
                    api_token,
                    bitbucket_workspace_name: workspace,
                    git_provider_id: id,
                    created_at: now,
                    updated_at: now,
                })
                .await
                .map(|_| ()),
        }
    }

    async fn update_child(&self, id: i64, credentials: ProviderCredentials) -> sqlx::Result<()> {
        let now = chrono::Utc::now().timestamp();
        match credentials {
            ProviderCredentials::Github {
                app_name,
                app_id,
                client_id,
                client_secret,
                installation_id,
                private_key,
            } => {
                let mut item = self
                    .github
                    .get_by_git_provider_id(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                let child_id = item.id.ok_or(sqlx::Error::RowNotFound)?;
                item.github_app_name = app_name.or(item.github_app_name);
                item.github_app_id = app_id.or(item.github_app_id);
                item.github_client_id = client_id.or(item.github_client_id);
                item.github_client_secret = client_secret.or(item.github_client_secret);
                item.github_installation_id = installation_id.or(item.github_installation_id);
                item.github_private_key = private_key.or(item.github_private_key);
                item.updated_at = now;
                self.github.update(child_id, &item).await
            }
            ProviderCredentials::Gitlab {
                url,
                internal_url,
                application_id,
                redirect_uri,
                secret,
                access_token,
                refresh_token,
                group_name,
            } => {
                let mut item = self
                    .gitlab
                    .get_by_git_provider_id(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                let child_id = item.id.ok_or(sqlx::Error::RowNotFound)?;
                item.gitlab_url = url;
                item.gitlab_internal_url = internal_url.or(item.gitlab_internal_url);
                item.application_id = application_id.or(item.application_id);
                item.redirect_uri = redirect_uri.or(item.redirect_uri);
                item.secret = secret.or(item.secret);
                item.access_token = access_token.or(item.access_token);
                item.refresh_token = refresh_token.or(item.refresh_token);
                item.group_name = group_name.or(item.group_name);
                item.updated_at = now;
                self.gitlab.update(child_id, &item).await
            }
            ProviderCredentials::Gitea {
                url,
                internal_url,
                redirect_uri,
                client_id,
                client_secret,
                access_token,
                refresh_token,
                scopes,
            } => {
                let mut item = self
                    .gitea
                    .get_by_git_provider_id(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                let child_id = item.id.ok_or(sqlx::Error::RowNotFound)?;
                item.gitea_url = url;
                item.gitea_internal_url = internal_url.or(item.gitea_internal_url);
                item.redirect_uri = redirect_uri.or(item.redirect_uri);
                item.client_id = client_id.or(item.client_id);
                item.client_secret = client_secret.or(item.client_secret);
                item.access_token = access_token.or(item.access_token);
                item.refresh_token = refresh_token.or(item.refresh_token);
                item.scopes = scopes.or(item.scopes);
                item.updated_at = now;
                self.gitea.update(child_id, &item).await
            }
            ProviderCredentials::Bitbucket {
                username,
                email,
                app_password,
                api_token,
                workspace,
            } => {
                let mut item = self
                    .bitbucket
                    .get_by_git_provider_id(id)
                    .await?
                    .ok_or(sqlx::Error::RowNotFound)?;
                let child_id = item.id.ok_or(sqlx::Error::RowNotFound)?;
                item.bitbucket_username = username.or(item.bitbucket_username);
                item.bitbucket_email = email.or(item.bitbucket_email);
                item.app_password = app_password.or(item.app_password);
                item.api_token = api_token.or(item.api_token);
                item.bitbucket_workspace_name = workspace.or(item.bitbucket_workspace_name);
                item.updated_at = now;
                self.bitbucket.update(child_id, &item).await
            }
        }
    }
}

fn provider_type(credentials: &ProviderCredentials) -> crate::utils::provider::GitProviderType {
    use crate::utils::provider::GitProviderType;
    match credentials {
        ProviderCredentials::Github { .. } => GitProviderType::Github,
        ProviderCredentials::Gitlab { .. } => GitProviderType::Gitlab,
        ProviderCredentials::Gitea { .. } => GitProviderType::Gitea,
        ProviderCredentials::Bitbucket { .. } => GitProviderType::Bitbucket,
    }
}

fn generate_secret() -> Result<String, String> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|error| format!("could not generate webhook secret: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
