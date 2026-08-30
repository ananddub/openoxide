use std::path::Path;

use serde::Deserialize;

use super::{ComposeRecord, ComposeService};
use crate::core::cache::CacheKey;
use crate::flash_gen::service_git::{
    SetBitbucketSourceParams, SetCustomGitSourceParams, SetGiteaSourceParams,
    SetGithubSourceParams, SetGitlabSourceParams,
};
use crate::services::observability::monitoring::service::MonitoringService;
use crate::utils::exec::CommandExecutor;
use crate::utils::os::OsCli;

impl ComposeService {
    // ── Git provider sources ──────────────────────────────────────────────────

    pub async fn set_github_source(
        &self,
        id: i64,
        input: PatchComposeGithubSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        // The retired repository preserved trigger/watch/submodule settings
        // while resetting auto_deploy to the request value (default 1).
        self.queries
            .set_github_source(&SetGithubSourceParams {
                service_id: id,
                trigger_type: current_trigger(git.as_ref()),
                repository: input.repository,
                owner: input.owner,
                branch: input.branch,
                github_provider_id: input.github_provider_id.unwrap_or(0),
                auto_deploy: input.auto_deploy.unwrap_or(1),
                watch_paths: current_watch_paths(git.as_ref()),
                enable_submodules: current_enable_submodules(git.as_ref()),
            })
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        input: PatchComposeGitlabSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        self.queries
            .set_gitlab_source(&SetGitlabSourceParams {
                service_id: id,
                trigger_type: current_trigger(git.as_ref()),
                gitlab_project_id: input.gitlab_project_id.unwrap_or(0),
                gitlab_repository: input.gitlab_repository,
                gitlab_owner: input.gitlab_owner,
                gitlab_branch: input.gitlab_branch,
                gitlab_build_path: git
                    .as_ref()
                    .and_then(|git| git.gitlab_build_path.clone())
                    .unwrap_or_default(),
                gitlab_path_namespace: input.gitlab_path_namespace.unwrap_or_default(),
                gitlab_provider_id: input.gitlab_provider_id.unwrap_or(0),
                auto_deploy: current_auto_deploy(git.as_ref()),
                watch_paths: current_watch_paths(git.as_ref()),
                enable_submodules: current_enable_submodules(git.as_ref()),
            })
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        input: PatchComposeGiteaSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        self.queries
            .set_gitea_source(&SetGiteaSourceParams {
                service_id: id,
                trigger_type: current_trigger(git.as_ref()),
                gitea_repository: input.gitea_repository,
                gitea_owner: input.gitea_owner,
                gitea_branch: input.gitea_branch,
                gitea_build_path: git
                    .as_ref()
                    .and_then(|git| git.gitea_build_path.clone())
                    .unwrap_or_default(),
                gitea_provider_id: input.gitea_provider_id.unwrap_or(0),
                auto_deploy: current_auto_deploy(git.as_ref()),
                watch_paths: current_watch_paths(git.as_ref()),
                enable_submodules: current_enable_submodules(git.as_ref()),
            })
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        input: PatchComposeBitbucketSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        self.queries
            .set_bitbucket_source(&SetBitbucketSourceParams {
                service_id: id,
                trigger_type: current_trigger(git.as_ref()),
                bitbucket_repository: input.bitbucket_repository,
                bitbucket_repository_slug: input.bitbucket_repository_slug.unwrap_or_default(),
                bitbucket_owner: input.bitbucket_owner,
                bitbucket_branch: input.bitbucket_branch,
                bitbucket_build_path: git
                    .as_ref()
                    .and_then(|git| git.bitbucket_build_path.clone())
                    .unwrap_or_default(),
                bitbucket_provider_id: input.bitbucket_provider_id.unwrap_or(0),
                auto_deploy: current_auto_deploy(git.as_ref()),
                watch_paths: current_watch_paths(git.as_ref()),
                enable_submodules: current_enable_submodules(git.as_ref()),
            })
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: PatchComposeCustomGitSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        self.queries
            .set_custom_git_source(&SetCustomGitSourceParams {
                service_id: id,
                trigger_type: current_trigger(git.as_ref()),
                custom_git_url: input.custom_git_url,
                custom_git_branch: input.custom_git_branch,
                custom_git_build_path: git
                    .as_ref()
                    .and_then(|git| git.custom_git_build_path.clone())
                    .unwrap_or_default(),
                custom_git_ssh_key_id: input.custom_git_ssh_key_id.unwrap_or(0),
                auto_deploy: current_auto_deploy(git.as_ref()),
                watch_paths: current_watch_paths(git.as_ref()),
                enable_submodules: current_enable_submodules(git.as_ref()),
            })
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }

    // ── Raw (inline) source ───────────────────────────────────────────────────

    pub async fn set_raw_source(
        &self,
        id: i64,
        input: PatchComposeRawSourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        // Old set_raw_source cleared the git columns and stored the inline
        // file; in the unified schema that is a deleted service_git row.
        self.queries.compose_set_raw_source(id).await?;
        self.queries
            .compose_set_raw_source_content(&input.compose_file, id)
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.get_by_id(id).await
    }
}

/// Pass-through helpers keeping the retired repositories' semantics: the
/// trigger/watch/submodule/auto-deploy settings survive source switches
/// (bind params are non-Option, so NULLs collapse to their defaults).
fn current_trigger(git: Option<&crate::flash_gen::models::ServiceGit>) -> String {
    git.map(|git| git.trigger_type.clone())
        .unwrap_or_else(|| "PUSH".into())
}

fn current_auto_deploy(git: Option<&crate::flash_gen::models::ServiceGit>) -> i64 {
    git.and_then(|git| git.auto_deploy).unwrap_or(0)
}

fn current_watch_paths(git: Option<&crate::flash_gen::models::ServiceGit>) -> String {
    git.and_then(|git| git.watch_paths.clone())
        .unwrap_or_default()
}

fn current_enable_submodules(git: Option<&crate::flash_gen::models::ServiceGit>) -> i64 {
    git.map(|git| git.enable_submodules)
        .unwrap_or(0)
}

/// Stream a ZIP to the server agent, then extract it through the same agent.
pub async fn deploy_zip_to_remote(
    zip_path: &Path,
    dest_path: &str,
    server_id: i64,
    uuid: &str,
) -> Result<(), String> {
    let monitoring = auto_di::resolve::<MonitoringService>()
        .await
        .map_err(|e| format!("DI error: {e}"))?;
    let remote_zip = format!("/tmp/upload-{uuid}.zip");
    let clean_zip = format!("/tmp/clean-{uuid}.zip");
    monitoring
        .upload_file(server_id, zip_path, &remote_zip)
        .await
        .map_err(|e| format!("Agent transfer failed: {e}"))?;
    let cmd = CommandExecutor::Agent(monitoring.agent_executor(server_id));

    let os = OsCli::new(&cmd);
    os.archive(&remote_zip)
        .zip()
        .sanitize()
        .to(&clean_zip)
        .run()
        .await
        .map_err(|e| format!("ZIP validation failed: {e}"))?;
    os.dir(dest_path)
        .create()
        .run()
        .await
        .map_err(|e| format!("Failed to create remote directory: {e}"))?;

    os.archive(&clean_zip)
        .zip()
        .extract_to(dest_path)
        .overwrite()
        .run()
        .await
        .map_err(|e| format!("Failed to unzip on remote: {e}"))?;

    let _ = os.file(&remote_zip).delete().run().await;
    let _ = os.file(&clean_zip).delete().run().await;

    Ok(())
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Deserialize)]
pub struct PatchComposeGithubSourceDto {
    pub repository: String,
    pub owner: String,
    pub branch: String,
    pub github_provider_id: Option<i64>,
    pub auto_deploy: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchComposeGitlabSourceDto {
    pub gitlab_project_id: Option<i64>,
    pub gitlab_repository: String,
    pub gitlab_owner: String,
    pub gitlab_branch: String,
    pub gitlab_path_namespace: Option<String>,
    pub gitlab_provider_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchComposeGiteaSourceDto {
    pub gitea_repository: String,
    pub gitea_owner: String,
    pub gitea_branch: String,
    pub gitea_provider_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchComposeBitbucketSourceDto {
    pub bitbucket_repository: String,
    pub bitbucket_repository_slug: Option<String>,
    pub bitbucket_owner: String,
    pub bitbucket_branch: String,
    pub bitbucket_provider_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchComposeCustomGitSourceDto {
    pub custom_git_url: String,
    pub custom_git_branch: String,
    pub custom_git_ssh_key_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchComposeRawSourceDto {
    pub compose_file: String,
}
