use crate::utils::provider::discovery::{
    CollaboratorPermission, GitReferenceInfo, ProviderAccess, RepositoryInfo,
};

use super::GitProviderService;

impl GitProviderService {
    pub async fn test_connection(&self, id: i64) -> Result<(), String> {
        self.discovery.test(&self.access(id).await?).await
    }

    pub async fn repositories(&self, id: i64) -> Result<Vec<RepositoryInfo>, String> {
        self.discovery.repositories(&self.access(id).await?).await
    }

    pub async fn branches(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        self.discovery
            .branches(&self.access(id).await?, owner, repository)
            .await
    }

    pub async fn tags(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        self.discovery
            .tags(&self.access(id).await?, owner, repository)
            .await
    }

    pub async fn collaborator_permission(
        &self,
        id: i64,
        owner: &str,
        repository: &str,
        username: &str,
    ) -> Result<CollaboratorPermission, String> {
        self.discovery
            .collaborator_permission(&self.access(id).await?, owner, repository, username)
            .await
    }

    pub(super) async fn access(&self, id: i64) -> Result<ProviderAccess, String> {
        let provider = self.get(id).await.map_err(|error| error.to_string())?;
        match provider.provider_type.as_str() {
            "GITHUB" => {
                let item = self.github_item(id).await?;
                Ok(ProviderAccess::GithubApp {
                    app_id: item.github_app_id.ok_or("GitHub App ID is required")?,
                    installation_id: item
                        .github_installation_id
                        .ok_or("GitHub installation ID is required")?,
                    private_key: item
                        .github_private_key
                        .ok_or("GitHub private key is required")?,
                })
            }
            "GITLAB" => {
                let item = self
                    .gitlab
                    .get_by_git_provider_id(id)
                    .await
                    .map_err(|error| error.to_string())?
                    .ok_or("GitLab configuration not found")?;
                Ok(ProviderAccess::Gitlab {
                    base_url: item.gitlab_url,
                    token: item.access_token.ok_or("GitLab access token is required")?,
                })
            }
            "GITEA" => {
                let item = self
                    .gitea
                    .get_by_git_provider_id(id)
                    .await
                    .map_err(|error| error.to_string())?
                    .ok_or("Gitea configuration not found")?;
                Ok(ProviderAccess::Gitea {
                    base_url: item.gitea_url,
                    token: item.access_token.ok_or("Gitea access token is required")?,
                })
            }
            "BITBUCKET" => {
                let item = self
                    .bitbucket
                    .get_by_git_provider_id(id)
                    .await
                    .map_err(|error| error.to_string())?
                    .ok_or("Bitbucket configuration not found")?;
                Ok(ProviderAccess::Bitbucket {
                    username: item.bitbucket_username,
                    app_password: item.app_password,
                    api_token: item.api_token,
                })
            }
            _ => Err("Unsupported Git provider".into()),
        }
    }
}
