use std::path::Path;

use crate::api::connect::generated::openoxide::api::v1::{
    SetBitbucketSourceRequest, SetCustomGitSourceRequest, SetDockerSourceRequest,
    SetDropSourceRequest, SetGiteaSourceRequest, SetGithubSourceRequest, SetGitlabSourceRequest,
};
use crate::core::cache::CacheKey;
use crate::services::observability::monitoring::service::MonitoringService;
use crate::utils::exec::CommandExecutor;
use crate::utils::os::OsCli;

use super::{ApplicationRecord, ApplicationService};

fn current_trigger(git: Option<&crate::flash_gen::models::ServiceGit>) -> String {
    git.map(|git| git.trigger_type.clone())
        .unwrap_or_else(|| "PUSH".into())
}

fn current_auto_deploy(git: Option<&crate::flash_gen::models::ServiceGit>) -> i64 {
    git.and_then(|git| git.auto_deploy).unwrap_or(1)
}

fn current_watch_paths(git: Option<&crate::flash_gen::models::ServiceGit>) -> String {
    git.and_then(|git| git.watch_paths.clone())
        .unwrap_or_default()
}

fn current_enable_submodules(git: Option<&crate::flash_gen::models::ServiceGit>) -> i64 {
    git.map(|git| git.enable_submodules).unwrap_or(0)
}

impl ApplicationService {
    // The old repository bound provider/ssh-key ids as nullable columns; the
    // generated setters take non-Option i64 (0 would violate the foreign keys),
    // and the old single-row UPDATE also cleared the docker/drop columns that
    // now live in services/service_application — so these stay raw SQL.
    pub async fn set_github_source(
        &self,
        id: i64,
        input: SetGithubSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        sqlx::query(
            "INSERT INTO service_git (service_id, source_type, trigger_type, repository, owner, branch, github_provider_id, auto_deploy, watch_paths, enable_submodules) \
             VALUES (?, 'GITHUB', ?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(service_id) DO UPDATE SET \
             source_type = 'GITHUB', trigger_type = excluded.trigger_type, \
             repository = excluded.repository, owner = excluded.owner, branch = excluded.branch, \
             github_provider_id = excluded.github_provider_id, auto_deploy = excluded.auto_deploy, \
             watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules, \
             gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL, gitlab_build_path = NULL, gitlab_path_namespace = NULL, gitlab_provider_id = NULL, \
             gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL, gitea_provider_id = NULL, \
             bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL, bitbucket_build_path = NULL, bitbucket_provider_id = NULL, \
             custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL, custom_git_ssh_key_id = NULL;",
        )
        .bind(id)
        .bind(current_trigger(git.as_ref()))
        .bind(&input.repository)
        .bind(&input.owner)
        .bind(&input.branch)
        .bind(input.provider_id)
        .bind(i64::from(input.auto_deploy.unwrap_or(true)))
        .bind(current_watch_paths(git.as_ref()))
        .bind(current_enable_submodules(git.as_ref()))
        .execute(&self.queries.pool)
        .await?;
        sqlx::query("UPDATE services SET docker_image = NULL WHERE id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET build_path = ?, drop_build_path = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL WHERE service_id = ?;",
        )
        .bind(input.build_path.unwrap_or_else(|| "/".into()))
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        input: SetGitlabSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        sqlx::query(
            "INSERT INTO service_git (service_id, source_type, trigger_type, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch, gitlab_build_path, gitlab_path_namespace, gitlab_provider_id, auto_deploy, watch_paths, enable_submodules) \
             VALUES (?, 'GITLAB', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(service_id) DO UPDATE SET \
             source_type = 'GITLAB', trigger_type = excluded.trigger_type, \
             gitlab_project_id = excluded.gitlab_project_id, gitlab_repository = excluded.gitlab_repository, \
             gitlab_owner = excluded.gitlab_owner, gitlab_branch = excluded.gitlab_branch, \
             gitlab_build_path = excluded.gitlab_build_path, gitlab_path_namespace = excluded.gitlab_path_namespace, \
             gitlab_provider_id = excluded.gitlab_provider_id, auto_deploy = excluded.auto_deploy, \
             watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules, \
             repository = NULL, owner = NULL, branch = NULL, github_provider_id = NULL, \
             gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL, gitea_provider_id = NULL, \
             bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL, bitbucket_build_path = NULL, bitbucket_provider_id = NULL, \
             custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL, custom_git_ssh_key_id = NULL;",
        )
        .bind(id)
        .bind(current_trigger(git.as_ref()))
        .bind(input.project_id)
        .bind(&input.repository)
        .bind(&input.owner)
        .bind(&input.branch)
        .bind(input.build_path.unwrap_or_else(|| "/".into()))
        .bind(&input.path_namespace)
        .bind(input.provider_id)
        .bind(current_auto_deploy(git.as_ref()))
        .bind(current_watch_paths(git.as_ref()))
        .bind(current_enable_submodules(git.as_ref()))
        .execute(&self.queries.pool)
        .await?;
        sqlx::query("UPDATE services SET docker_image = NULL WHERE id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET drop_build_path = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL WHERE service_id = ?;",
        )
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        input: SetGiteaSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        sqlx::query(
            "INSERT INTO service_git (service_id, source_type, trigger_type, gitea_repository, gitea_owner, gitea_branch, gitea_build_path, gitea_provider_id, auto_deploy, watch_paths, enable_submodules) \
             VALUES (?, 'GITEA', ?, ?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(service_id) DO UPDATE SET \
             source_type = 'GITEA', trigger_type = excluded.trigger_type, \
             gitea_repository = excluded.gitea_repository, gitea_owner = excluded.gitea_owner, \
             gitea_branch = excluded.gitea_branch, gitea_build_path = excluded.gitea_build_path, \
             gitea_provider_id = excluded.gitea_provider_id, auto_deploy = excluded.auto_deploy, \
             watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules, \
             repository = NULL, owner = NULL, branch = NULL, github_provider_id = NULL, \
             gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL, gitlab_build_path = NULL, gitlab_path_namespace = NULL, gitlab_provider_id = NULL, \
             bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL, bitbucket_build_path = NULL, bitbucket_provider_id = NULL, \
             custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL, custom_git_ssh_key_id = NULL;",
        )
        .bind(id)
        .bind(current_trigger(git.as_ref()))
        .bind(&input.repository)
        .bind(&input.owner)
        .bind(&input.branch)
        .bind(input.build_path.unwrap_or_else(|| "/".into()))
        .bind(input.provider_id)
        .bind(current_auto_deploy(git.as_ref()))
        .bind(current_watch_paths(git.as_ref()))
        .bind(current_enable_submodules(git.as_ref()))
        .execute(&self.queries.pool)
        .await?;
        sqlx::query("UPDATE services SET docker_image = NULL WHERE id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET drop_build_path = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL WHERE service_id = ?;",
        )
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        input: SetBitbucketSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        sqlx::query(
            "INSERT INTO service_git (service_id, source_type, trigger_type, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner, bitbucket_branch, bitbucket_build_path, bitbucket_provider_id, auto_deploy, watch_paths, enable_submodules) \
             VALUES (?, 'BITBUCKET', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(service_id) DO UPDATE SET \
             source_type = 'BITBUCKET', trigger_type = excluded.trigger_type, \
             bitbucket_repository = excluded.bitbucket_repository, bitbucket_repository_slug = excluded.bitbucket_repository_slug, \
             bitbucket_owner = excluded.bitbucket_owner, bitbucket_branch = excluded.bitbucket_branch, \
             bitbucket_build_path = excluded.bitbucket_build_path, bitbucket_provider_id = excluded.bitbucket_provider_id, \
             auto_deploy = excluded.auto_deploy, watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules, \
             repository = NULL, owner = NULL, branch = NULL, github_provider_id = NULL, \
             gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL, gitlab_build_path = NULL, gitlab_path_namespace = NULL, gitlab_provider_id = NULL, \
             gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL, gitea_provider_id = NULL, \
             custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL, custom_git_ssh_key_id = NULL;",
        )
        .bind(id)
        .bind(current_trigger(git.as_ref()))
        .bind(&input.repository)
        .bind(&input.repository_slug)
        .bind(&input.owner)
        .bind(&input.branch)
        .bind(input.build_path.unwrap_or_else(|| "/".into()))
        .bind(input.provider_id)
        .bind(current_auto_deploy(git.as_ref()))
        .bind(current_watch_paths(git.as_ref()))
        .bind(current_enable_submodules(git.as_ref()))
        .execute(&self.queries.pool)
        .await?;
        sqlx::query("UPDATE services SET docker_image = NULL WHERE id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET drop_build_path = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL WHERE service_id = ?;",
        )
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_docker_source(
        &self,
        id: i64,
        input: SetDockerSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        self.queries.set_docker_source(id).await?;
        sqlx::query("UPDATE services SET docker_image = ? WHERE id = ?;")
            .bind(&input.image)
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET docker_username = ?, docker_password = ?, registry_url = ?, drop_build_path = NULL WHERE service_id = ?;",
        )
        .bind(&input.username)
        .bind(&input.password)
        .bind(&input.registry_url)
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        input: SetCustomGitSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        sqlx::query(
            "INSERT INTO service_git (service_id, source_type, trigger_type, custom_git_url, custom_git_branch, custom_git_build_path, custom_git_ssh_key_id, auto_deploy, watch_paths, enable_submodules) \
             VALUES (?, 'CUSTOM', ?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(service_id) DO UPDATE SET \
             source_type = 'CUSTOM', trigger_type = excluded.trigger_type, \
             custom_git_url = excluded.custom_git_url, custom_git_branch = excluded.custom_git_branch, \
             custom_git_build_path = excluded.custom_git_build_path, custom_git_ssh_key_id = excluded.custom_git_ssh_key_id, \
             auto_deploy = excluded.auto_deploy, watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules, \
             repository = NULL, owner = NULL, branch = NULL, github_provider_id = NULL, \
             gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL, gitlab_build_path = NULL, gitlab_path_namespace = NULL, gitlab_provider_id = NULL, \
             gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL, gitea_provider_id = NULL, \
             bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL, bitbucket_build_path = NULL, bitbucket_provider_id = NULL;",
        )
        .bind(id)
        .bind(current_trigger(git.as_ref()))
        .bind(&input.url)
        .bind(&input.branch)
        .bind(input.build_path.unwrap_or_else(|| "/".into()))
        .bind(input.ssh_key_id)
        .bind(current_auto_deploy(git.as_ref()))
        .bind(current_watch_paths(git.as_ref()))
        .bind(current_enable_submodules(git.as_ref()))
        .execute(&self.queries.pool)
        .await?;
        sqlx::query("UPDATE services SET docker_image = NULL WHERE id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET drop_build_path = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL WHERE service_id = ?;",
        )
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }

    pub async fn set_drop_source(
        &self,
        id: i64,
        input: SetDropSourceRequest,
    ) -> sqlx::Result<ApplicationRecord> {
        self.get_by_id(id).await?;
        sqlx::query("DELETE FROM service_git WHERE service_id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query("UPDATE services SET docker_image = NULL WHERE id = ?;")
            .bind(id)
            .execute(&self.queries.pool)
            .await?;
        sqlx::query(
            "UPDATE service_application SET drop_build_path = ?, docker_username = NULL, docker_password = NULL, registry_url = NULL WHERE service_id = ?;",
        )
        .bind(input.build_path.unwrap_or_else(|| "/".into()))
        .bind(id)
        .execute(&self.queries.pool)
        .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id).await
    }
}

/// Installs a drop-upload ZIP for an application: transfers the archive to
/// the server's Agent, sanitizes it (path traversal / symlink stripping),
/// wipes the previous code directory, and extracts the new sources into it.
/// The panel and the local Agent share the same host paths, so the Agent's
/// own base path resolves to the directory the builder later reads.
pub async fn install_drop_zip(
    app_name: &str,
    server_id: Option<i64>,
    zip_path: &Path,
) -> Result<(), String> {
    let server_id = crate::services::compute::compose::remote::agent_server_id(server_id).await?;
    let monitoring = auto_di::resolve::<MonitoringService>()
        .await
        .map_err(|e| format!("DI error: {e}"))?;

    let code_dir = crate::utils::paths::openoxide_paths().application_code(app_name);
    let uuid = uuid::Uuid::new_v4().to_string();
    let remote_zip = format!("/tmp/openoxide-drop-{uuid}.zip");
    let clean_zip = format!("/tmp/openoxide-drop-clean-{uuid}.zip");

    monitoring
        .upload_file(server_id, zip_path, &remote_zip)
        .await
        .map_err(|e| format!("Agent transfer failed: {e}"))?;
    let cmd = CommandExecutor::Agent(monitoring.agent_executor(server_id));
    let os = OsCli::new(&cmd);

    let outcome = async {
        os.archive(&remote_zip)
            .zip()
            .sanitize()
            .to(&clean_zip)
            .run()
            .await
            .map_err(|e| format!("ZIP validation failed: {e}"))?;
        // Replace the previous source tree wholesale — a stale file must not
        // survive into the new build.
        os.dir(&code_dir)
            .delete()
            .run()
            .await
            .map_err(|e| format!("failed to clear previous sources: {e}"))?;
        os.dir(&code_dir)
            .create()
            .run()
            .await
            .map_err(|e| format!("failed to create source directory: {e}"))?;
        os.archive(&clean_zip)
            .zip()
            .extract_to(&code_dir)
            .overwrite()
            .run()
            .await
            .map_err(|e| format!("failed to unzip on server: {e}"))?;
        Ok::<(), String>(())
    }
    .await;

    let _ = os.file(&remote_zip).delete().run().await;
    let _ = os.file(&clean_zip).delete().run().await;
    outcome
}
