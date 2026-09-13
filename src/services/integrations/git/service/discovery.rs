use crate::utils::provider::discovery::{
    CollaboratorPermission, GitReferenceInfo, ProviderAccess, RepositoryInfo,
};

use super::{parse_config, BitbucketConfig, GitProviderService, GiteaConfig, GithubConfig,
    GitlabConfig};

impl GitProviderService {
    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<(), String> {
        self.discovery
            .test(&self.access(id, organization_id).await?)
            .await
    }

    pub async fn repositories(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<RepositoryInfo>, String> {
        self.discovery
            .repositories(&self.access(id, organization_id).await?)
            .await
    }

    pub async fn branches(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        organization_id: i64,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        self.discovery
            .branches(
                &self.access(id, organization_id).await?,
                owner,
                repository,
            )
            .await
    }

    pub async fn tags(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        organization_id: i64,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        self.discovery
            .tags(&self.access(id, organization_id).await?, owner, repository)
            .await
    }

    pub async fn collaborator_permission(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        username: &str,
        organization_id: i64,
    ) -> Result<CollaboratorPermission, String> {
        self.discovery
            .collaborator_permission(
                &self.access(id, organization_id).await?,
                owner,
                repository,
                username,
            )
            .await
    }

    pub(super) async fn access(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<ProviderAccess, String> {
        use crate::utils::provider::GitProviderType;
        let row = self.provider_row(id, organization_id).await?;
        let provider_kind: GitProviderType = row
            .provider_type
            .parse()
            .map_err(|_| format!("Unsupported Git provider: {}", row.provider_type))?;
        match provider_kind {
            GitProviderType::Github => {
                let item: GithubConfig = parse_config(&row.config_json)
                    .map_err(|error| error.to_string())?;
                Ok(ProviderAccess::GithubApp {
                    app_id: item.app_id.ok_or("GitHub App ID is required")?,
                    installation_id: item
                        .installation_id
                        .ok_or("GitHub installation ID is required")?,
                    private_key: item.private_key.ok_or("GitHub private key is required")?,
                })
            }
            GitProviderType::Gitlab => {
                let item: GitlabConfig = parse_config(&row.config_json)
                    .map_err(|error| error.to_string())?;
                Ok(ProviderAccess::Gitlab {
                    base_url: item
                        .url
                        .filter(|value| !value.trim().is_empty())
                        .ok_or("GitLab URL is required")?,
                    token: item
                        .access_token
                        .ok_or("GitLab access token is required")?,
                })
            }
            GitProviderType::Gitea => {
                let item: GiteaConfig =
                    parse_config(&row.config_json).map_err(|e| e.to_string())?;
                Ok(ProviderAccess::Gitea {
                    base_url: item
                        .url
                        .filter(|value| !value.trim().is_empty())
                        .ok_or("Gitea URL is required")?,
                    token: item.access_token.ok_or("Gitea access token is required")?,
                })
            }
            GitProviderType::Bitbucket => {
                let item: BitbucketConfig = parse_config(&row.config_json)
                    .map_err(|error| error.to_string())?;
                Ok(ProviderAccess::Bitbucket {
                    username: item.username,
                    app_password: item.app_password,
                    api_token: item.api_token,
                })
            }
        }
    }
}
