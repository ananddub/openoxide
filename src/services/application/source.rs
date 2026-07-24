use crate::api::dto::application::{
    PatchBitbucketSourceDto, PatchCustomGitSourceDto, PatchDockerSourceDto, PatchDropSourceDto,
    PatchGiteaSourceDto, PatchGithubSourceDto, PatchGitlabSourceDto,
};

use super::{ApplicationRecord, ApplicationService};

impl ApplicationService {
    pub async fn set_github_source(
        &self,
        id: i64,
        input: PatchGithubSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_github_source(
                id,
                Some(input.repository),
                Some(input.owner),
                input.branch,
                input.build_path.unwrap_or_else(|| "/".into()),
                input.github_provider_id,
                input.auto_deploy.unwrap_or(1),
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        input: PatchGitlabSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_gitlab_source(
                id,
                input.gitlab_project_id,
                Some(input.gitlab_repository),
                Some(input.gitlab_owner),
                input.gitlab_branch,
                input.gitlab_build_path.unwrap_or_else(|| "/".into()),
                input.gitlab_path_namespace,
                input.gitlab_provider_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        input: PatchGiteaSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_gitea_source(
                id,
                Some(input.gitea_repository),
                Some(input.gitea_owner),
                input.gitea_branch,
                input.gitea_build_path.unwrap_or_else(|| "/".into()),
                input.gitea_provider_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        input: PatchBitbucketSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_bitbucket_source(
                id,
                Some(input.bitbucket_repository),
                input.bitbucket_repository_slug,
                Some(input.bitbucket_owner),
                input.bitbucket_branch,
                input.bitbucket_build_path.unwrap_or_else(|| "/".into()),
                input.bitbucket_provider_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_docker_source(
        &self,
        id: i64,
        input: PatchDockerSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_docker_source(
                id,
                Some(input.docker_image),
                input.docker_username,
                input.docker_password,
                input.registry_url,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: PatchCustomGitSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_custom_git_source(
                id,
                Some(input.custom_git_url),
                input.custom_git_branch,
                input.custom_git_build_path.unwrap_or_else(|| "/".into()),
                input.custom_git_ssh_key_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_drop_source(
        &self,
        id: i64,
        input: PatchDropSourceDto,
    ) -> sqlx::Result<ApplicationRecord> {
        self.repo_app
            .set_drop_source(id, input.drop_build_path.unwrap_or_else(|| "/".into()))
            .await?;
        self.get_by_id(id).await
    }
}
