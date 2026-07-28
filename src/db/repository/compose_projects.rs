use crate::db::models::compose_projects::ComposeProject;
use crate::utils::builder::compose::adapter::mapper::ComposeRow;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ComposeProjectRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ComposeProjectRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<ComposeProject>, sqlx::Error> {
        sqlx::query_as!(
            ComposeProject,
            r#"SELECT id AS "id?: String", name AS "name: String", app_name AS "app_name: String", description AS "description?: String", env_var AS "env_var?: String", compose_file AS "compose_file: String", refresh_token AS "refresh_token?: String", source_type AS "source_type: String", compose_type AS "compose_type: String", compose_status AS "compose_status: String", trigger_type AS "trigger_type: String", repository AS "repository?: String", owner AS "owner?: String", branch AS "branch?: String", auto_deploy AS "auto_deploy: i64", gitlab_project_id AS "gitlab_project_id?: i64", gitlab_repository AS "gitlab_repository?: String", gitlab_owner AS "gitlab_owner?: String", gitlab_branch AS "gitlab_branch?: String", gitlab_path_namespace AS "gitlab_path_namespace?: String", bitbucket_repository AS "bitbucket_repository?: String", bitbucket_repository_slug AS "bitbucket_repository_slug?: String", bitbucket_owner AS "bitbucket_owner?: String", bitbucket_branch AS "bitbucket_branch?: String", gitea_repository AS "gitea_repository?: String", gitea_owner AS "gitea_owner?: String", gitea_branch AS "gitea_branch?: String", custom_git_url AS "custom_git_url?: String", custom_git_branch AS "custom_git_branch?: String", custom_git_ssh_key_id AS "custom_git_ssh_key_id?: i64", command AS "command: String", enable_submodules AS "enable_submodules: i64", compose_path AS "compose_path: String", suffix AS "suffix: String", randomize AS "randomize: i64", isolated_deployment AS "isolated_deployment: i64", isolated_deployments_volume AS "isolated_deployments_volume: i64", watch_paths AS "watch_paths?: String", service_networks AS "service_networks: String", environment_id AS "environment_id: i64", server_id AS "server_id?: i64", github_provider_id AS "github_provider_id?: i64", gitlab_provider_id AS "gitlab_provider_id?: i64", gitea_provider_id AS "gitea_provider_id?: i64", bitbucket_provider_id AS "bitbucket_provider_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM compose_projects"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<ComposeProject>, sqlx::Error> {
        sqlx::query_as!(
            ComposeProject,
            r#"SELECT id AS "id?: String", name AS "name: String", app_name AS "app_name: String", description AS "description?: String", env_var AS "env_var?: String", compose_file AS "compose_file: String", refresh_token AS "refresh_token?: String", source_type AS "source_type: String", compose_type AS "compose_type: String", compose_status AS "compose_status: String", trigger_type AS "trigger_type: String", repository AS "repository?: String", owner AS "owner?: String", branch AS "branch?: String", auto_deploy AS "auto_deploy: i64", gitlab_project_id AS "gitlab_project_id?: i64", gitlab_repository AS "gitlab_repository?: String", gitlab_owner AS "gitlab_owner?: String", gitlab_branch AS "gitlab_branch?: String", gitlab_path_namespace AS "gitlab_path_namespace?: String", bitbucket_repository AS "bitbucket_repository?: String", bitbucket_repository_slug AS "bitbucket_repository_slug?: String", bitbucket_owner AS "bitbucket_owner?: String", bitbucket_branch AS "bitbucket_branch?: String", gitea_repository AS "gitea_repository?: String", gitea_owner AS "gitea_owner?: String", gitea_branch AS "gitea_branch?: String", custom_git_url AS "custom_git_url?: String", custom_git_branch AS "custom_git_branch?: String", custom_git_ssh_key_id AS "custom_git_ssh_key_id?: i64", command AS "command: String", enable_submodules AS "enable_submodules: i64", compose_path AS "compose_path: String", suffix AS "suffix: String", randomize AS "randomize: i64", isolated_deployment AS "isolated_deployment: i64", isolated_deployments_volume AS "isolated_deployments_volume: i64", watch_paths AS "watch_paths?: String", service_networks AS "service_networks: String", environment_id AS "environment_id: i64", server_id AS "server_id?: i64", github_provider_id AS "github_provider_id?: i64", gitlab_provider_id AS "gitlab_provider_id?: i64", gitea_provider_id AS "gitea_provider_id?: i64", bitbucket_provider_id AS "bitbucket_provider_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM compose_projects WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &ComposeProject) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO compose_projects (name, app_name, description, env_var, compose_file, refresh_token, source_type, compose_type, compose_status, trigger_type, repository, owner, branch, auto_deploy, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch, gitlab_path_namespace, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner, bitbucket_branch, gitea_repository, gitea_owner, gitea_branch, custom_git_url, custom_git_branch, custom_git_ssh_key_id, command, enable_submodules, compose_path, suffix, randomize, isolated_deployment, isolated_deployments_volume, watch_paths, service_networks, environment_id, server_id, github_provider_id, gitlab_provider_id, gitea_provider_id, bitbucket_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.app_name,
            &item.description,
            &item.env_var,
            &item.compose_file,
            &item.refresh_token,
            &item.source_type,
            &item.compose_type,
            &item.compose_status,
            &item.trigger_type,
            &item.repository,
            &item.owner,
            &item.branch,
            item.auto_deploy,
            item.gitlab_project_id,
            &item.gitlab_repository,
            &item.gitlab_owner,
            &item.gitlab_branch,
            &item.gitlab_path_namespace,
            &item.bitbucket_repository,
            &item.bitbucket_repository_slug,
            &item.bitbucket_owner,
            &item.bitbucket_branch,
            &item.gitea_repository,
            &item.gitea_owner,
            &item.gitea_branch,
            &item.custom_git_url,
            &item.custom_git_branch,
            item.custom_git_ssh_key_id,
            &item.command,
            item.enable_submodules,
            &item.compose_path,
            &item.suffix,
            item.randomize,
            item.isolated_deployment,
            item.isolated_deployments_volume,
            &item.watch_paths,
            &item.service_networks,
            item.environment_id,
            item.server_id,
            item.github_provider_id,
            item.gitlab_provider_id,
            item.gitea_provider_id,
            item.bitbucket_provider_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &ComposeProject) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET name = ?, app_name = ?, description = ?, env_var = ?, compose_file = ?, refresh_token = ?, source_type = ?, compose_type = ?, compose_status = ?, trigger_type = ?, repository = ?, owner = ?, branch = ?, auto_deploy = ?, gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?, gitlab_path_namespace = ?, bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?, bitbucket_branch = ?, gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, custom_git_url = ?, custom_git_branch = ?, custom_git_ssh_key_id = ?, command = ?, enable_submodules = ?, compose_path = ?, suffix = ?, randomize = ?, isolated_deployment = ?, isolated_deployments_volume = ?, watch_paths = ?, service_networks = ?, environment_id = ?, server_id = ?, github_provider_id = ?, gitlab_provider_id = ?, gitea_provider_id = ?, bitbucket_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.app_name,
            &item.description,
            &item.env_var,
            &item.compose_file,
            &item.refresh_token,
            &item.source_type,
            &item.compose_type,
            &item.compose_status,
            &item.trigger_type,
            &item.repository,
            &item.owner,
            &item.branch,
            item.auto_deploy,
            item.gitlab_project_id,
            &item.gitlab_repository,
            &item.gitlab_owner,
            &item.gitlab_branch,
            &item.gitlab_path_namespace,
            &item.bitbucket_repository,
            &item.bitbucket_repository_slug,
            &item.bitbucket_owner,
            &item.bitbucket_branch,
            &item.gitea_repository,
            &item.gitea_owner,
            &item.gitea_branch,
            &item.custom_git_url,
            &item.custom_git_branch,
            item.custom_git_ssh_key_id,
            &item.command,
            item.enable_submodules,
            &item.compose_path,
            &item.suffix,
            item.randomize,
            item.isolated_deployment,
            item.isolated_deployments_volume,
            &item.watch_paths,
            &item.service_networks,
            item.environment_id,
            item.server_id,
            item.github_provider_id,
            item.gitlab_provider_id,
            item.gitea_provider_id,
            item.bitbucket_provider_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM compose_projects WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> Result<Vec<ComposeProject>, sqlx::Error> {
        sqlx::query_as!(
            ComposeProject,
            r#"SELECT id AS "id?: String", name AS "name: String", app_name AS "app_name: String", description AS "description?: String", env_var AS "env_var?: String", compose_file AS "compose_file: String", refresh_token AS "refresh_token?: String", source_type AS "source_type: String", compose_type AS "compose_type: String", compose_status AS "compose_status: String", trigger_type AS "trigger_type: String", repository AS "repository?: String", owner AS "owner?: String", branch AS "branch?: String", auto_deploy AS "auto_deploy: i64", gitlab_project_id AS "gitlab_project_id?: i64", gitlab_repository AS "gitlab_repository?: String", gitlab_owner AS "gitlab_owner?: String", gitlab_branch AS "gitlab_branch?: String", gitlab_path_namespace AS "gitlab_path_namespace?: String", bitbucket_repository AS "bitbucket_repository?: String", bitbucket_repository_slug AS "bitbucket_repository_slug?: String", bitbucket_owner AS "bitbucket_owner?: String", bitbucket_branch AS "bitbucket_branch?: String", gitea_repository AS "gitea_repository?: String", gitea_owner AS "gitea_owner?: String", gitea_branch AS "gitea_branch?: String", custom_git_url AS "custom_git_url?: String", custom_git_branch AS "custom_git_branch?: String", custom_git_ssh_key_id AS "custom_git_ssh_key_id?: i64", command AS "command: String", enable_submodules AS "enable_submodules: i64", compose_path AS "compose_path: String", suffix AS "suffix: String", randomize AS "randomize: i64", isolated_deployment AS "isolated_deployment: i64", isolated_deployments_volume AS "isolated_deployments_volume: i64", watch_paths AS "watch_paths?: String", service_networks AS "service_networks: String", environment_id AS "environment_id: i64", server_id AS "server_id?: i64", github_provider_id AS "github_provider_id?: i64", gitlab_provider_id AS "gitlab_provider_id?: i64", gitea_provider_id AS "gitea_provider_id?: i64", bitbucket_provider_id AS "bitbucket_provider_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM compose_projects WHERE environment_id = ? ORDER BY created_at DESC, id DESC"#,
            environment_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_simple(
        &self,
        name: String,
        app_name: String,
        description: Option<String>,
        environment_id: i64,
        server_id: Option<i64>,
        source_type: String,
        compose_type: String,
        compose_file: String,
    ) -> Result<ComposeProject, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO compose_projects (name, app_name, description, environment_id, server_id, source_type, compose_type, compose_file)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            name,
            app_name,
            description,
            environment_id,
            server_id,
            source_type,
            compose_type,
            compose_file
        )
        .execute(self.pool.as_ref())
        .await?;
        let id = res.last_insert_rowid();
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        env_var: Option<String>,
        compose_file: String,
        compose_type: String,
        trigger_type: String,
        command: String,
        enable_submodules: Option<i64>,
        compose_path: String,
        suffix: Option<String>,
        randomize: Option<i64>,
        isolated_deployment: Option<i64>,
        isolated_deployments_volume: Option<i64>,
        watch_paths: Option<String>,
        service_networks: String,
        server_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET
               name = ?, description = ?, env_var = ?, compose_file = ?, compose_type = ?,
               trigger_type = ?, command = ?, enable_submodules = COALESCE(?, enable_submodules),
               compose_path = ?, suffix = COALESCE(?, suffix), randomize = COALESCE(?, randomize),
               isolated_deployment = COALESCE(?, isolated_deployment),
               isolated_deployments_volume = COALESCE(?, isolated_deployments_volume),
               watch_paths = COALESCE(?, watch_paths), service_networks = ?, server_id = ?
               WHERE id = ?"#,
            name,
            description,
            env_var,
            compose_file,
            compose_type,
            trigger_type,
            command,
            enable_submodules,
            compose_path,
            suffix,
            randomize,
            isolated_deployment,
            isolated_deployments_volume,
            watch_paths,
            service_networks,
            server_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn update_status(
        &self,
        id: i64,
        status: &str,
    ) -> Result<ComposeProject, sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET compose_status = ? WHERE id = ?"#,
            status,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn get_deployment_context(
        &self,
        id: i64,
    ) -> Result<(i64, i64, Option<i64>), sqlx::Error> {
        sqlx::query_as::<_, (i64, i64, Option<i64>)>(
            r#"SELECT c.environment_id, e.project_id, c.server_id
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               WHERE c.id = ?"#,
        )
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn set_github_source(
        &self,
        id: i64,
        repository: Option<String>,
        owner: Option<String>,
        branch: Option<String>,
        github_provider_id: Option<i64>,
        auto_deploy: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GITHUB',
               repository = ?, owner = ?, branch = ?, github_provider_id = ?, auto_deploy = ?,
               gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
               gitlab_path_namespace = NULL, gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            repository,
            owner,
            branch,
            github_provider_id,
            auto_deploy,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_gitlab_source(
        &self,
        id: i64,
        gitlab_project_id: Option<i64>,
        gitlab_repository: Option<String>,
        gitlab_owner: Option<String>,
        gitlab_branch: Option<String>,
        gitlab_path_namespace: Option<String>,
        gitlab_provider_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GITLAB',
               gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?,
               gitlab_path_namespace = ?, gitlab_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitea_repository = NULL, gitea_owner = NULL,
               gitea_branch = NULL, bitbucket_repository = NULL, bitbucket_repository_slug = NULL,
               bitbucket_owner = NULL, bitbucket_branch = NULL, custom_git_url = NULL,
               custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
           gitlab_project_id,
           gitlab_repository,
           gitlab_owner,
           gitlab_branch,
           gitlab_path_namespace,
           gitlab_provider_id,
           id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_gitea_source(
        &self,
        id: i64,
        gitea_repository: Option<String>,
        gitea_owner: Option<String>,
        gitea_branch: Option<String>,
        gitea_provider_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GITEA',
               gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
           gitea_repository,
           gitea_owner,
           gitea_branch,
           gitea_provider_id,
           id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_bitbucket_source(
        &self,
        id: i64,
        bitbucket_repository: Option<String>,
        bitbucket_repository_slug: Option<String>,
        bitbucket_owner: Option<String>,
        bitbucket_branch: Option<String>,
        bitbucket_provider_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'BITBUCKET',
               bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?,
               bitbucket_branch = ?, bitbucket_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
           bitbucket_repository,
           bitbucket_repository_slug,
           bitbucket_owner,
           bitbucket_branch,
           bitbucket_provider_id,
           id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_custom_git_source(
        &self,
        id: i64,
        custom_git_url: Option<String>,
        custom_git_branch: Option<String>,
        custom_git_ssh_key_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'GIT',
               custom_git_url = ?, custom_git_branch = ?, custom_git_ssh_key_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL
               WHERE id = ?"#,
           custom_git_url,
           custom_git_branch,
           custom_git_ssh_key_id,
           id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_raw_source(&self, id: i64, compose_file: String) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'RAW', compose_file = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            compose_file,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_drop_source(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE compose_projects SET source_type = 'DROP',
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
               bitbucket_branch = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn get_spec_row(&self, compose_id: i64) -> Result<ComposeRow, sqlx::Error> {
        sqlx::query_as::<_, ComposeRow>(
            r#"SELECT
               c.app_name, c.source_type, c.compose_type, c.compose_file, c.env_var,
               c.service_networks,
               c.repository, c.owner, c.branch, c.gitlab_repository, c.gitlab_owner,
               c.gitlab_branch, c.gitea_repository, c.gitea_branch, c.bitbucket_repository,
               c.bitbucket_owner, c.bitbucket_branch, c.custom_git_url, c.custom_git_branch,
               c.custom_git_ssh_key_id, c.enable_submodules, c.compose_path, e.env_var AS environment_env,
               p.env_var AS project_env,
               ghp.github_private_key AS github_token,
               glp.access_token AS gitlab_token,
               bbp.api_token AS bitbucket_token,
               gtp.access_token AS gitea_token,
               sk.private_key AS ssh_private_key
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               JOIN projects p ON p.id = e.project_id
               LEFT JOIN github_providers ghp ON ghp.id = c.github_provider_id
               LEFT JOIN gitlab_providers glp ON glp.id = c.gitlab_provider_id
               LEFT JOIN bitbucket_providers bbp ON bbp.id = c.bitbucket_provider_id
               LEFT JOIN gitea_providers gtp ON gtp.id = c.gitea_provider_id
               LEFT JOIN ssh_keys sk ON sk.id = c.custom_git_ssh_key_id
               WHERE c.id = ?"#,
        )
        .bind(compose_id)
        .fetch_one(self.pool.as_ref())
        .await
    }
}
