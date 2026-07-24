use super::{ComposeRecord, ComposeService};
use crate::api::dto::compose::{
    PatchComposeBitbucketSourceDto, PatchComposeCustomGitSourceDto, PatchComposeGiteaSourceDto,
    PatchComposeGithubSourceDto, PatchComposeGitlabSourceDto, PatchComposeRawSourceDto,
};
use upload::{TempFileGuard, sanitize_zip, save_multipart_to_file};

impl ComposeService {
    // ── Git provider sources ──────────────────────────────────────────────────

    pub async fn set_github_source(
        &self,
        id: i64,
        input: PatchComposeGithubSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose
            .set_github_source(
                id,
                Some(input.repository),
                Some(input.owner),
                Some(input.branch),
                input.github_provider_id,
                input.auto_deploy.unwrap_or(1),
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        input: PatchComposeGitlabSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose
            .set_gitlab_source(
                id,
                input.gitlab_project_id,
                Some(input.gitlab_repository),
                Some(input.gitlab_owner),
                Some(input.gitlab_branch),
                input.gitlab_path_namespace,
                input.gitlab_provider_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        input: PatchComposeGiteaSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose
            .set_gitea_source(
                id,
                Some(input.gitea_repository),
                Some(input.gitea_owner),
                Some(input.gitea_branch),
                input.gitea_provider_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        input: PatchComposeBitbucketSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose
            .set_bitbucket_source(
                id,
                Some(input.bitbucket_repository),
                input.bitbucket_repository_slug,
                Some(input.bitbucket_owner),
                Some(input.bitbucket_branch),
                input.bitbucket_provider_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: PatchComposeCustomGitSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose
            .set_custom_git_source(
                id,
                Some(input.custom_git_url),
                Some(input.custom_git_branch),
                input.custom_git_ssh_key_id,
            )
            .await?;
        self.get_by_id(id).await
    }

    // ── Raw (inline) source ───────────────────────────────────────────────────

    pub async fn set_raw_source(
        &self,
        id: i64,
        input: PatchComposeRawSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        self.repo_compose
            .set_raw_source(id, input.compose_file)
            .await?;
        self.get_by_id(id).await
    }

    // ── Drop / upload source ──────────────────────────────────────────────────

    pub async fn upload_drop_source(
        &self,
        id: i64,
        multipart: axum::extract::Multipart,
    ) -> Result<ComposeRecord, String> {
        // 1. Fetch project
        let project = self
            .repo_compose
            .get_by_id(id)
            .await
            .map_err(|e| format!("Database error: {e}"))?
            .ok_or_else(|| "Compose project not found".to_string())?;

        // 2. Set up temp files with auto-cleanup on drop
        let uuid = uuid::Uuid::new_v4().to_string();
        let mut cleanup = TempFileGuard::new();

        let upload_zip = std::env::temp_dir().join(format!("upload-{uuid}.zip"));
        let clean_zip = std::env::temp_dir().join(format!("clean-{uuid}.zip"));
        cleanup.track(&upload_zip);
        cleanup.track(&clean_zip);

        // 3. Save upload + sanitize
        save_multipart_to_file(multipart, &upload_zip).await?;
        sanitize_zip(&upload_zip, &clean_zip).await?;

        // 4. Deploy — local or remote
        let dest_path = crate::utils::paths::rustploy_paths().compose_source(&project.app_name);
        match project.server_id {
            None => {
                local::deploy_zip_locally(&clean_zip, &dest_path).await?;
            }
            Some(server_id) => {
                let key_path = std::env::temp_dir().join(format!("key-{uuid}.pem"));
                cleanup.track(&key_path);
                remote::deploy_zip_to_remote(
                    &self.db, &clean_zip, &dest_path, server_id, &key_path, &uuid,
                )
                .await?;
            }
        }

        // 5. Mark source type in DB
        self.repo_compose
            .set_drop_source(id)
            .await
            .map_err(|e| format!("Database error updating source: {e}"))?;

        self.get_by_id(id)
            .await
            .map_err(|e| format!("Database error fetching project: {e}"))
    }
}

pub mod local;
pub mod remote;
pub mod upload;
