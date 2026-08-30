use getrandom::fill;

use crate::flash_gen::git_providers::{CreateGitProviderParams, UpdateGitProviderParams};
use crate::flash_gen::models::GitProviders;
use crate::flash_gen::service_git::CountServicesUsingGitProviderParams;
use crate::services::git_provider::{
    CreateProvider, GitProviderView, ProviderCredentials, UpdateProvider,
};

use super::{config_to_string, parse_config, BitbucketConfig, GitProviderService, GiteaConfig,
    GithubConfig, GitlabConfig};

impl GitProviderService {
    pub async fn create(&self, input: CreateProvider) -> sqlx::Result<(GitProviderView, String)> {
        let config_json = initial_config(&input.credentials)?;
        let provider = self
            .queries
            .create_git_provider(&CreateGitProviderParams {
                name: input.name,
                provider_type: provider_type(&input.credentials).as_str().to_string(),
                config_json,
                shared: i64::from(input.shared),
                webhook_secret: String::new(),
            })
            .await?;
        let secret = generate_secret().map_err(sqlx::Error::Protocol)?;
        let rotated = self
            .queries
            .git_providers_rotate_webhook_secret(&secret, provider.id)
            .await?;
        if rotated.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok((self.get(provider.id).await?, secret))
    }

    pub async fn update(&self, id: i64, input: UpdateProvider) -> sqlx::Result<GitProviderView> {
        let current = self.queries.get_git_provider_by_i_d(id).await?;
        if current.provider_type != provider_type(&input.credentials).as_str() {
            return Err(sqlx::Error::Protocol(
                "Git provider type cannot be changed".into(),
            ));
        }
        let config_json = merged_config(&current, &input.credentials)?;
        self.queries
            .update_git_provider(&UpdateGitProviderParams {
                name: input.name,
                provider_type: current.provider_type,
                config_json,
                shared: i64::from(input.shared.unwrap_or(current.shared != 0)),
                webhook_secret: current.webhook_secret.clone().unwrap_or_default(),
                id,
            })
            .await?;
        self.get(id).await
    }

    pub async fn rotate_secret(&self, id: i64) -> sqlx::Result<String> {
        self.get(id).await?;
        let secret = generate_secret().map_err(sqlx::Error::Protocol)?;
        let rotated = self
            .queries
            .git_providers_rotate_webhook_secret(&secret, id)
            .await?;
        if rotated.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(secret)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get(id).await?;
        let usage = self
            .queries
            .count_services_using_git_provider(&CountServicesUsingGitProviderParams {
                github_provider_id: id,
                gitlab_provider_id: id,
                gitea_provider_id: id,
                bitbucket_provider_id: id,
            })
            .await?;
        if usage > 0 {
            return Err(sqlx::Error::Protocol(
                "Git provider is assigned to an application or compose project".into(),
            ));
        }
        self.queries.delete_git_provider(id).await?;
        Ok(())
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

fn initial_config(credentials: &ProviderCredentials) -> sqlx::Result<String> {
    match credentials {
        ProviderCredentials::Github {
            app_name,
            app_id,
            client_id,
            client_secret,
            installation_id,
            private_key,
        } => config_to_string(&GithubConfig {
            app_name: app_name.clone(),
            app_id: *app_id,
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            installation_id: installation_id.clone(),
            private_key: private_key.clone(),
        }),
        ProviderCredentials::Gitlab {
            url,
            internal_url,
            application_id,
            redirect_uri,
            secret,
            access_token,
            refresh_token,
            group_name,
        } => config_to_string(&GitlabConfig {
            url: Some(url.clone()),
            internal_url: internal_url.clone(),
            application_id: application_id.clone(),
            redirect_uri: redirect_uri.clone(),
            secret: secret.clone(),
            access_token: access_token.clone(),
            refresh_token: refresh_token.clone(),
            group_name: group_name.clone(),
            expires_at: None,
        }),
        ProviderCredentials::Gitea {
            url,
            internal_url,
            redirect_uri,
            client_id,
            client_secret,
            access_token,
            refresh_token,
            scopes,
        } => config_to_string(&GiteaConfig {
            url: Some(url.clone()),
            internal_url: internal_url.clone(),
            redirect_uri: redirect_uri.clone(),
            client_id: client_id.clone(),
            client_secret: client_secret.clone(),
            access_token: access_token.clone(),
            refresh_token: refresh_token.clone(),
            expires_at: None,
            scopes: scopes.clone(),
            last_authenticated_at: None,
        }),
        ProviderCredentials::Bitbucket {
            username,
            email,
            app_password,
            api_token,
            workspace,
        } => config_to_string(&BitbucketConfig {
            username: username.clone(),
            email: email.clone(),
            app_password: app_password.clone(),
            api_token: api_token.clone(),
            workspace_name: workspace.clone(),
        }),
    }
}

fn merged_config(current: &GitProviders, credentials: &ProviderCredentials) -> sqlx::Result<String> {
    match credentials {
        ProviderCredentials::Github {
            app_name,
            app_id,
            client_id,
            client_secret,
            installation_id,
            private_key,
        } => {
            let mut item: GithubConfig = parse_config(&current.config_json)?;
            item.app_name = app_name.clone().or(item.app_name);
            item.app_id = app_id.or(item.app_id);
            item.client_id = client_id.clone().or(item.client_id);
            item.client_secret = client_secret.clone().or(item.client_secret);
            item.installation_id = installation_id.clone().or(item.installation_id);
            item.private_key = private_key.clone().or(item.private_key);
            config_to_string(&item)
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
            let mut item: GitlabConfig = parse_config(&current.config_json)?;
            item.url = Some(url.clone());
            item.internal_url = internal_url.clone().or(item.internal_url);
            item.application_id = application_id.clone().or(item.application_id);
            item.redirect_uri = redirect_uri.clone().or(item.redirect_uri);
            item.secret = secret.clone().or(item.secret);
            item.access_token = access_token.clone().or(item.access_token);
            item.refresh_token = refresh_token.clone().or(item.refresh_token);
            item.group_name = group_name.clone().or(item.group_name);
            config_to_string(&item)
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
            let mut item: GiteaConfig = parse_config(&current.config_json)?;
            item.url = Some(url.clone());
            item.internal_url = internal_url.clone().or(item.internal_url);
            item.redirect_uri = redirect_uri.clone().or(item.redirect_uri);
            item.client_id = client_id.clone().or(item.client_id);
            item.client_secret = client_secret.clone().or(item.client_secret);
            item.access_token = access_token.clone().or(item.access_token);
            item.refresh_token = refresh_token.clone().or(item.refresh_token);
            item.scopes = scopes.clone().or(item.scopes);
            config_to_string(&item)
        }
        ProviderCredentials::Bitbucket {
            username,
            email,
            app_password,
            api_token,
            workspace,
        } => {
            let mut item: BitbucketConfig = parse_config(&current.config_json)?;
            item.username = username.clone().or(item.username);
            item.email = email.clone().or(item.email);
            item.app_password = app_password.clone().or(item.app_password);
            item.api_token = api_token.clone().or(item.api_token);
            item.workspace_name = workspace.clone().or(item.workspace_name);
            config_to_string(&item)
        }
    }
}

fn generate_secret() -> Result<String, String> {
    let mut bytes = [0_u8; 32];
    fill(&mut bytes).map_err(|error| format!("could not generate webhook secret: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
