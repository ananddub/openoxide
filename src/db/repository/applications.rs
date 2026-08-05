use crate::db::models::applications::Application;
use crate::utils::builder::application::adapter::mapper::AppRow;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ApplicationRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ApplicationRepository {
    pub async fn update_networks(
        &self,
        id: i64,
        network_ids: &str,
        detach_rustploy_network: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE applications SET network_ids = ?, detach_rustploy_network = ? WHERE id = ?",
            network_ids,
            detach_rustploy_network,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn move_to_environment_in_same_organization(
        &self,
        id: i64,
        target_environment_id: i64,
    ) -> Result<Option<Application>, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE applications
               SET environment_id = ?, updated_at = strftime('%s', 'now')
               WHERE id = ?
                 AND EXISTS (
                     SELECT 1
                     FROM environments source_environment
                     INNER JOIN projects source_project ON source_project.id = source_environment.project_id
                     INNER JOIN environments target_environment ON target_environment.id = ?
                     INNER JOIN projects target_project ON target_project.id = target_environment.project_id
                     WHERE source_environment.id = applications.environment_id
                       AND source_project.organization_id = target_project.organization_id
                 )"#,
            target_environment_id,
            id,
            target_environment_id
        )
        .execute(self.pool.as_ref())
        .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_by_id(id).await
    }

    pub async fn rotate_refresh_token(&self, id: i64, token: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE applications SET refresh_token = ? WHERE id = ?",
            token,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn update_preview_config(
        &self,
        id: i64,
        preview_env: Option<&str>,
        preview_build_args: Option<&str>,
        preview_build_secrets: Option<&str>,
        preview_labels: Option<&str>,
        preview_wildcard: Option<&str>,
        preview_port: i64,
        preview_https: bool,
        preview_path: &str,
        preview_certificate_type: &str,
        preview_custom_cert_resolver: Option<&str>,
        preview_limit: i64,
        active: bool,
        require_collaborator_permissions: bool,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE applications SET preview_env = ?, preview_build_args = ?, preview_build_secrets = ?, preview_labels = ?, preview_wildcard = ?, preview_port = ?, preview_https = ?, preview_path = ?, preview_certificate_type = ?, preview_custom_cert_resolver = ?, preview_limit = ?, is_preview_deployments_active = ?, preview_require_collaborator_permissions = ? WHERE id = ?"#,
            preview_env,
            preview_build_args,
            preview_build_secrets,
            preview_labels,
            preview_wildcard,
            preview_port,
            preview_https,
            preview_path,
            preview_certificate_type,
            preview_custom_cert_resolver,
            preview_limit,
            active,
            require_collaborator_permissions,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Application>, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"SELECT id AS "id?: String", name AS "name: String", app_name AS "app_name: String", description AS "description?: String", source_type AS "source_type: String", build_type AS "build_type: String", app_status AS "app_status: String", trigger_type AS "trigger_type: String", build_args AS "build_args?: String", build_secrets AS "build_secrets?: String", dockerfile AS "dockerfile?: String", docker_context_path AS "docker_context_path?: String", docker_build_stage AS "docker_build_stage?: String", publish_directory AS "publish_directory?: String", is_static_spa AS "is_static_spa?: i64", create_env_file AS "create_env_file: i64", railpack_version AS "railpack_version?: String", heroku_version AS "heroku_version?: String", command AS "command?: String", args AS "args?: String", env_var AS "env_var?: String", build_path AS "build_path?: String", clean_cache AS "clean_cache: i64", drop_build_path AS "drop_build_path?: String", enable_submodules AS "enable_submodules: i64", watch_paths AS "watch_paths?: String", refresh_token AS "refresh_token?: String", icon AS "icon?: String", network_ids AS "network_ids: String", detach_rustploy_network AS "detach_rustploy_network: i64", memory_reservation AS "memory_reservation?: String", memory_limit AS "memory_limit?: String", cpu_reservation AS "cpu_reservation?: String", cpu_limit AS "cpu_limit?: String", replicas AS "replicas: i64", health_check_swarm AS "health_check_swarm?: String", restart_policy_swarm AS "restart_policy_swarm?: String", placement_swarm AS "placement_swarm?: String", update_config_swarm AS "update_config_swarm?: String", rollback_config_swarm AS "rollback_config_swarm?: String", mode_swarm AS "mode_swarm?: String", labels_swarm AS "labels_swarm?: String", network_swarm AS "network_swarm?: String", endpoint_spec_swarm AS "endpoint_spec_swarm?: String", ulimits_swarm AS "ulimits_swarm?: String", stop_grace_period_swarm AS "stop_grace_period_swarm?: i64", repository AS "repository?: String", owner AS "owner?: String", branch AS "branch?: String", auto_deploy AS "auto_deploy?: i64", gitlab_project_id AS "gitlab_project_id?: i64", gitlab_repository AS "gitlab_repository?: String", gitlab_owner AS "gitlab_owner?: String", gitlab_branch AS "gitlab_branch?: String", gitlab_build_path AS "gitlab_build_path?: String", gitlab_path_namespace AS "gitlab_path_namespace?: String", gitea_repository AS "gitea_repository?: String", gitea_owner AS "gitea_owner?: String", gitea_branch AS "gitea_branch?: String", gitea_build_path AS "gitea_build_path?: String", bitbucket_repository AS "bitbucket_repository?: String", bitbucket_repository_slug AS "bitbucket_repository_slug?: String", bitbucket_owner AS "bitbucket_owner?: String", bitbucket_branch AS "bitbucket_branch?: String", bitbucket_build_path AS "bitbucket_build_path?: String", docker_image AS "docker_image?: String", docker_username AS "docker_username?: String", docker_password AS "docker_password?: String", registry_url AS "registry_url?: String", custom_git_url AS "custom_git_url?: String", custom_git_branch AS "custom_git_branch?: String", custom_git_build_path AS "custom_git_build_path?: String", custom_git_ssh_key_id AS "custom_git_ssh_key_id?: i64", preview_env AS "preview_env?: String", preview_build_args AS "preview_build_args?: String", preview_build_secrets AS "preview_build_secrets?: String", preview_labels AS "preview_labels?: String", preview_wildcard AS "preview_wildcard?: String", preview_port AS "preview_port?: i64", preview_https AS "preview_https: i64", preview_path AS "preview_path?: String", preview_certificate_type AS "preview_certificate_type: String", preview_custom_cert_resolver AS "preview_custom_cert_resolver?: String", preview_limit AS "preview_limit?: i64", is_preview_deployments_active AS "is_preview_deployments_active: i64", preview_require_collaborator_permissions AS "preview_require_collaborator_permissions: i64", rollback_active AS "rollback_active: i64", environment_id AS "environment_id: i64", server_id AS "server_id?: i64", build_server_id AS "build_server_id?: i64", registry_id AS "registry_id?: i64", rollback_registry_id AS "rollback_registry_id?: i64", build_registry_id AS "build_registry_id?: i64", github_provider_id AS "github_provider_id?: i64", gitlab_provider_id AS "gitlab_provider_id?: i64", gitea_provider_id AS "gitea_provider_id?: i64", bitbucket_provider_id AS "bitbucket_provider_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM applications"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Application>, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"SELECT id AS "id?: String", name AS "name: String", app_name AS "app_name: String", description AS "description?: String", source_type AS "source_type: String", build_type AS "build_type: String", app_status AS "app_status: String", trigger_type AS "trigger_type: String", build_args AS "build_args?: String", build_secrets AS "build_secrets?: String", dockerfile AS "dockerfile?: String", docker_context_path AS "docker_context_path?: String", docker_build_stage AS "docker_build_stage?: String", publish_directory AS "publish_directory?: String", is_static_spa AS "is_static_spa?: i64", create_env_file AS "create_env_file: i64", railpack_version AS "railpack_version?: String", heroku_version AS "heroku_version?: String", command AS "command?: String", args AS "args?: String", env_var AS "env_var?: String", build_path AS "build_path?: String", clean_cache AS "clean_cache: i64", drop_build_path AS "drop_build_path?: String", enable_submodules AS "enable_submodules: i64", watch_paths AS "watch_paths?: String", refresh_token AS "refresh_token?: String", icon AS "icon?: String", network_ids AS "network_ids: String", detach_rustploy_network AS "detach_rustploy_network: i64", memory_reservation AS "memory_reservation?: String", memory_limit AS "memory_limit?: String", cpu_reservation AS "cpu_reservation?: String", cpu_limit AS "cpu_limit?: String", replicas AS "replicas: i64", health_check_swarm AS "health_check_swarm?: String", restart_policy_swarm AS "restart_policy_swarm?: String", placement_swarm AS "placement_swarm?: String", update_config_swarm AS "update_config_swarm?: String", rollback_config_swarm AS "rollback_config_swarm?: String", mode_swarm AS "mode_swarm?: String", labels_swarm AS "labels_swarm?: String", network_swarm AS "network_swarm?: String", endpoint_spec_swarm AS "endpoint_spec_swarm?: String", ulimits_swarm AS "ulimits_swarm?: String", stop_grace_period_swarm AS "stop_grace_period_swarm?: i64", repository AS "repository?: String", owner AS "owner?: String", branch AS "branch?: String", auto_deploy AS "auto_deploy?: i64", gitlab_project_id AS "gitlab_project_id?: i64", gitlab_repository AS "gitlab_repository?: String", gitlab_owner AS "gitlab_owner?: String", gitlab_branch AS "gitlab_branch?: String", gitlab_build_path AS "gitlab_build_path?: String", gitlab_path_namespace AS "gitlab_path_namespace?: String", gitea_repository AS "gitea_repository?: String", gitea_owner AS "gitea_owner?: String", gitea_branch AS "gitea_branch?: String", gitea_build_path AS "gitea_build_path?: String", bitbucket_repository AS "bitbucket_repository?: String", bitbucket_repository_slug AS "bitbucket_repository_slug?: String", bitbucket_owner AS "bitbucket_owner?: String", bitbucket_branch AS "bitbucket_branch?: String", bitbucket_build_path AS "bitbucket_build_path?: String", docker_image AS "docker_image?: String", docker_username AS "docker_username?: String", docker_password AS "docker_password?: String", registry_url AS "registry_url?: String", custom_git_url AS "custom_git_url?: String", custom_git_branch AS "custom_git_branch?: String", custom_git_build_path AS "custom_git_build_path?: String", custom_git_ssh_key_id AS "custom_git_ssh_key_id?: i64", preview_env AS "preview_env?: String", preview_build_args AS "preview_build_args?: String", preview_build_secrets AS "preview_build_secrets?: String", preview_labels AS "preview_labels?: String", preview_wildcard AS "preview_wildcard?: String", preview_port AS "preview_port?: i64", preview_https AS "preview_https: i64", preview_path AS "preview_path?: String", preview_certificate_type AS "preview_certificate_type: String", preview_custom_cert_resolver AS "preview_custom_cert_resolver?: String", preview_limit AS "preview_limit?: i64", is_preview_deployments_active AS "is_preview_deployments_active: i64", preview_require_collaborator_permissions AS "preview_require_collaborator_permissions: i64", rollback_active AS "rollback_active: i64", environment_id AS "environment_id: i64", server_id AS "server_id?: i64", build_server_id AS "build_server_id?: i64", registry_id AS "registry_id?: i64", rollback_registry_id AS "rollback_registry_id?: i64", build_registry_id AS "build_registry_id?: i64", github_provider_id AS "github_provider_id?: i64", gitlab_provider_id AS "gitlab_provider_id?: i64", gitea_provider_id AS "gitea_provider_id?: i64", bitbucket_provider_id AS "bitbucket_provider_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM applications WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Application) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO applications (name, app_name, description, source_type, build_type, app_status, trigger_type, build_args, build_secrets, dockerfile, docker_context_path, docker_build_stage, publish_directory, is_static_spa, create_env_file, railpack_version, heroku_version, command, args, env_var, build_path, clean_cache, drop_build_path, enable_submodules, watch_paths, refresh_token, icon, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas, health_check_swarm, restart_policy_swarm, placement_swarm, update_config_swarm, rollback_config_swarm, mode_swarm, labels_swarm, network_swarm, endpoint_spec_swarm, ulimits_swarm, stop_grace_period_swarm, repository, owner, branch, auto_deploy, gitlab_project_id, gitlab_repository, gitlab_owner, gitlab_branch, gitlab_build_path, gitlab_path_namespace, gitea_repository, gitea_owner, gitea_branch, gitea_build_path, bitbucket_repository, bitbucket_repository_slug, bitbucket_owner, bitbucket_branch, bitbucket_build_path, docker_image, docker_username, docker_password, registry_url, custom_git_url, custom_git_branch, custom_git_build_path, custom_git_ssh_key_id, preview_env, preview_build_args, preview_build_secrets, preview_labels, preview_wildcard, preview_port, preview_https, preview_path, preview_certificate_type, preview_custom_cert_resolver, preview_limit, is_preview_deployments_active, preview_require_collaborator_permissions, rollback_active, environment_id, server_id, build_server_id, registry_id, rollback_registry_id, build_registry_id, github_provider_id, gitlab_provider_id, gitea_provider_id, bitbucket_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.app_name,
            &item.description,
            &item.source_type,
            &item.build_type,
            &item.app_status,
            &item.trigger_type,
            &item.build_args,
            &item.build_secrets,
            &item.dockerfile,
            &item.docker_context_path,
            &item.docker_build_stage,
            &item.publish_directory,
            item.is_static_spa,
            item.create_env_file,
            &item.railpack_version,
            &item.heroku_version,
            &item.command,
            &item.args,
            &item.env_var,
            &item.build_path,
            item.clean_cache,
            &item.drop_build_path,
            item.enable_submodules,
            &item.watch_paths,
            &item.refresh_token,
            &item.icon,
            &item.memory_reservation,
            &item.memory_limit,
            &item.cpu_reservation,
            &item.cpu_limit,
            item.replicas,
            &item.health_check_swarm,
            &item.restart_policy_swarm,
            &item.placement_swarm,
            &item.update_config_swarm,
            &item.rollback_config_swarm,
            &item.mode_swarm,
            &item.labels_swarm,
            &item.network_swarm,
            &item.endpoint_spec_swarm,
            &item.ulimits_swarm,
            item.stop_grace_period_swarm,
            &item.repository,
            &item.owner,
            &item.branch,
            item.auto_deploy,
            item.gitlab_project_id,
            &item.gitlab_repository,
            &item.gitlab_owner,
            &item.gitlab_branch,
            &item.gitlab_build_path,
            &item.gitlab_path_namespace,
            &item.gitea_repository,
            &item.gitea_owner,
            &item.gitea_branch,
            &item.gitea_build_path,
            &item.bitbucket_repository,
            &item.bitbucket_repository_slug,
            &item.bitbucket_owner,
            &item.bitbucket_branch,
            &item.bitbucket_build_path,
            &item.docker_image,
            &item.docker_username,
            &item.docker_password,
            &item.registry_url,
            &item.custom_git_url,
            &item.custom_git_branch,
            &item.custom_git_build_path,
            item.custom_git_ssh_key_id,
            &item.preview_env,
            &item.preview_build_args,
            &item.preview_build_secrets,
            &item.preview_labels,
            &item.preview_wildcard,
            item.preview_port,
            item.preview_https,
            &item.preview_path,
            &item.preview_certificate_type,
            &item.preview_custom_cert_resolver,
            item.preview_limit,
            item.is_preview_deployments_active,
            item.preview_require_collaborator_permissions,
            item.rollback_active,
            item.environment_id,
            item.server_id,
            item.build_server_id,
            item.registry_id,
            item.rollback_registry_id,
            item.build_registry_id,
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

    pub async fn update(&self, id: i64, item: &Application) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET name = ?, app_name = ?, description = ?, source_type = ?, build_type = ?, app_status = ?, trigger_type = ?, build_args = ?, build_secrets = ?, dockerfile = ?, docker_context_path = ?, docker_build_stage = ?, publish_directory = ?, is_static_spa = ?, create_env_file = ?, railpack_version = ?, heroku_version = ?, command = ?, args = ?, env_var = ?, build_path = ?, clean_cache = ?, drop_build_path = ?, enable_submodules = ?, watch_paths = ?, refresh_token = ?, icon = ?, memory_reservation = ?, memory_limit = ?, cpu_reservation = ?, cpu_limit = ?, replicas = ?, health_check_swarm = ?, restart_policy_swarm = ?, placement_swarm = ?, update_config_swarm = ?, rollback_config_swarm = ?, mode_swarm = ?, labels_swarm = ?, network_swarm = ?, endpoint_spec_swarm = ?, ulimits_swarm = ?, stop_grace_period_swarm = ?, repository = ?, owner = ?, branch = ?, auto_deploy = ?, gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?, gitlab_build_path = ?, gitlab_path_namespace = ?, gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_build_path = ?, bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?, bitbucket_branch = ?, bitbucket_build_path = ?, docker_image = ?, docker_username = ?, docker_password = ?, registry_url = ?, custom_git_url = ?, custom_git_branch = ?, custom_git_build_path = ?, custom_git_ssh_key_id = ?, preview_env = ?, preview_build_args = ?, preview_build_secrets = ?, preview_labels = ?, preview_wildcard = ?, preview_port = ?, preview_https = ?, preview_path = ?, preview_certificate_type = ?, preview_custom_cert_resolver = ?, preview_limit = ?, is_preview_deployments_active = ?, preview_require_collaborator_permissions = ?, rollback_active = ?, environment_id = ?, server_id = ?, build_server_id = ?, registry_id = ?, rollback_registry_id = ?, build_registry_id = ?, github_provider_id = ?, gitlab_provider_id = ?, gitea_provider_id = ?, bitbucket_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.app_name,
            &item.description,
            &item.source_type,
            &item.build_type,
            &item.app_status,
            &item.trigger_type,
            &item.build_args,
            &item.build_secrets,
            &item.dockerfile,
            &item.docker_context_path,
            &item.docker_build_stage,
            &item.publish_directory,
            item.is_static_spa,
            item.create_env_file,
            &item.railpack_version,
            &item.heroku_version,
            &item.command,
            &item.args,
            &item.env_var,
            &item.build_path,
            item.clean_cache,
            &item.drop_build_path,
            item.enable_submodules,
            &item.watch_paths,
            &item.refresh_token,
            &item.icon,
            &item.memory_reservation,
            &item.memory_limit,
            &item.cpu_reservation,
            &item.cpu_limit,
            item.replicas,
            &item.health_check_swarm,
            &item.restart_policy_swarm,
            &item.placement_swarm,
            &item.update_config_swarm,
            &item.rollback_config_swarm,
            &item.mode_swarm,
            &item.labels_swarm,
            &item.network_swarm,
            &item.endpoint_spec_swarm,
            &item.ulimits_swarm,
            item.stop_grace_period_swarm,
            &item.repository,
            &item.owner,
            &item.branch,
            item.auto_deploy,
            item.gitlab_project_id,
            &item.gitlab_repository,
            &item.gitlab_owner,
            &item.gitlab_branch,
            &item.gitlab_build_path,
            &item.gitlab_path_namespace,
            &item.gitea_repository,
            &item.gitea_owner,
            &item.gitea_branch,
            &item.gitea_build_path,
            &item.bitbucket_repository,
            &item.bitbucket_repository_slug,
            &item.bitbucket_owner,
            &item.bitbucket_branch,
            &item.bitbucket_build_path,
            &item.docker_image,
            &item.docker_username,
            &item.docker_password,
            &item.registry_url,
            &item.custom_git_url,
            &item.custom_git_branch,
            &item.custom_git_build_path,
            item.custom_git_ssh_key_id,
            &item.preview_env,
            &item.preview_build_args,
            &item.preview_build_secrets,
            &item.preview_labels,
            &item.preview_wildcard,
            item.preview_port,
            item.preview_https,
            &item.preview_path,
            &item.preview_certificate_type,
            &item.preview_custom_cert_resolver,
            item.preview_limit,
            item.is_preview_deployments_active,
            item.preview_require_collaborator_permissions,
            item.rollback_active,
            item.environment_id,
            item.server_id,
            item.build_server_id,
            item.registry_id,
            item.rollback_registry_id,
            item.build_registry_id,
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
        sqlx::query!(r#"DELETE FROM applications WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> Result<Vec<Application>, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"SELECT id AS "id?: String", name AS "name: String", app_name AS "app_name: String", description AS "description?: String", source_type AS "source_type: String", build_type AS "build_type: String", app_status AS "app_status: String", trigger_type AS "trigger_type: String", build_args AS "build_args?: String", build_secrets AS "build_secrets?: String", dockerfile AS "dockerfile?: String", docker_context_path AS "docker_context_path?: String", docker_build_stage AS "docker_build_stage?: String", publish_directory AS "publish_directory?: String", is_static_spa AS "is_static_spa?: i64", create_env_file AS "create_env_file: i64", railpack_version AS "railpack_version?: String", heroku_version AS "heroku_version?: String", command AS "command?: String", args AS "args?: String", env_var AS "env_var?: String", build_path AS "build_path?: String", clean_cache AS "clean_cache: i64", drop_build_path AS "drop_build_path?: String", enable_submodules AS "enable_submodules: i64", watch_paths AS "watch_paths?: String", refresh_token AS "refresh_token?: String", icon AS "icon?: String", network_ids AS "network_ids: String", detach_rustploy_network AS "detach_rustploy_network: i64", memory_reservation AS "memory_reservation?: String", memory_limit AS "memory_limit?: String", cpu_reservation AS "cpu_reservation?: String", cpu_limit AS "cpu_limit?: String", replicas AS "replicas: i64", health_check_swarm AS "health_check_swarm?: String", restart_policy_swarm AS "restart_policy_swarm?: String", placement_swarm AS "placement_swarm?: String", update_config_swarm AS "update_config_swarm?: String", rollback_config_swarm AS "rollback_config_swarm?: String", mode_swarm AS "mode_swarm?: String", labels_swarm AS "labels_swarm?: String", network_swarm AS "network_swarm?: String", endpoint_spec_swarm AS "endpoint_spec_swarm?: String", ulimits_swarm AS "ulimits_swarm?: String", stop_grace_period_swarm AS "stop_grace_period_swarm?: i64", repository AS "repository?: String", owner AS "owner?: String", branch AS "branch?: String", auto_deploy AS "auto_deploy?: i64", gitlab_project_id AS "gitlab_project_id?: i64", gitlab_repository AS "gitlab_repository?: String", gitlab_owner AS "gitlab_owner?: String", gitlab_branch AS "gitlab_branch?: String", gitlab_build_path AS "gitlab_build_path?: String", gitlab_path_namespace AS "gitlab_path_namespace?: String", gitea_repository AS "gitea_repository?: String", gitea_owner AS "gitea_owner?: String", gitea_branch AS "gitea_branch?: String", gitea_build_path AS "gitea_build_path?: String", bitbucket_repository AS "bitbucket_repository?: String", bitbucket_repository_slug AS "bitbucket_repository_slug?: String", bitbucket_owner AS "bitbucket_owner?: String", bitbucket_branch AS "bitbucket_branch?: String", bitbucket_build_path AS "bitbucket_build_path?: String", docker_image AS "docker_image?: String", docker_username AS "docker_username?: String", docker_password AS "docker_password?: String", registry_url AS "registry_url?: String", custom_git_url AS "custom_git_url?: String", custom_git_branch AS "custom_git_branch?: String", custom_git_build_path AS "custom_git_build_path?: String", custom_git_ssh_key_id AS "custom_git_ssh_key_id?: i64", preview_env AS "preview_env?: String", preview_build_args AS "preview_build_args?: String", preview_build_secrets AS "preview_build_secrets?: String", preview_labels AS "preview_labels?: String", preview_wildcard AS "preview_wildcard?: String", preview_port AS "preview_port?: i64", preview_https AS "preview_https: i64", preview_path AS "preview_path?: String", preview_certificate_type AS "preview_certificate_type: String", preview_custom_cert_resolver AS "preview_custom_cert_resolver?: String", preview_limit AS "preview_limit?: i64", is_preview_deployments_active AS "is_preview_deployments_active: i64", preview_require_collaborator_permissions AS "preview_require_collaborator_permissions: i64", rollback_active AS "rollback_active: i64", environment_id AS "environment_id: i64", server_id AS "server_id?: i64", build_server_id AS "build_server_id?: i64", registry_id AS "registry_id?: i64", rollback_registry_id AS "rollback_registry_id?: i64", build_registry_id AS "build_registry_id?: i64", github_provider_id AS "github_provider_id?: i64", gitlab_provider_id AS "gitlab_provider_id?: i64", gitea_provider_id AS "gitea_provider_id?: i64", bitbucket_provider_id AS "bitbucket_provider_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM applications WHERE environment_id = ? ORDER BY created_at DESC, id DESC"#,
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
        source_type: String,
        build_type: String,
        environment_id: i64,
        server_id: Option<i64>,
    ) -> Result<Application, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO applications (name, app_name, description, source_type, build_type, environment_id, server_id) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            name,
            app_name,
            description,
            source_type,
            build_type,
            environment_id,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;

        self.get_by_id(res.last_insert_rowid())
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        build_type: String,
        trigger_type: String,
        env_var: Option<String>,
        icon: Option<String>,
        server_id: Option<i64>,
        build_server_id: Option<i64>,
        registry_id: Option<i64>,
        network_ids: String,
        detach_rustploy_network: i64,
    ) -> Result<Application, sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET name = ?, description = ?, build_type = ?, trigger_type = ?, env_var = ?, icon = ?, server_id = ?, build_server_id = ?, registry_id = ?, network_ids = ?, detach_rustploy_network = ? WHERE id = ?"#,
            name,
            description,
            build_type,
            trigger_type,
            env_var,
            icon,
            server_id,
            build_server_id,
            registry_id,
            network_ids,
            detach_rustploy_network,
            id
        )
        .execute(self.pool.as_ref())
        .await?;

        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn patch_build_config(
        &self,
        id: i64,
        build_args: Option<String>,
        build_secrets: Option<String>,
        dockerfile: Option<String>,
        docker_context_path: Option<String>,
        docker_build_stage: Option<String>,
        publish_directory: Option<String>,
        is_static_spa: Option<i64>,
        create_env_file: Option<i64>,
        railpack_version: Option<String>,
        heroku_version: Option<String>,
        command: Option<String>,
        args: Option<String>,
        build_path: Option<String>,
        clean_cache: Option<i64>,
        enable_submodules: Option<i64>,
        watch_paths: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               build_args = COALESCE(?, build_args), build_secrets = COALESCE(?, build_secrets),
               dockerfile = COALESCE(?, dockerfile), docker_context_path = COALESCE(?, docker_context_path),
               docker_build_stage = COALESCE(?, docker_build_stage), publish_directory = COALESCE(?, publish_directory),
               is_static_spa = COALESCE(?, is_static_spa), create_env_file = COALESCE(?, create_env_file),
               railpack_version = COALESCE(?, railpack_version), heroku_version = COALESCE(?, heroku_version),
               command = COALESCE(?, command), args = COALESCE(?, args), build_path = COALESCE(?, build_path),
               clean_cache = COALESCE(?, clean_cache), enable_submodules = COALESCE(?, enable_submodules),
               watch_paths = COALESCE(?, watch_paths)
               WHERE id = ?"#,
            build_args,
            build_secrets,
            dockerfile,
            docker_context_path,
            docker_build_stage,
            publish_directory,
            is_static_spa,
            create_env_file,
            railpack_version,
            heroku_version,
            command,
            args,
            build_path,
            clean_cache,
            enable_submodules,
            watch_paths,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn patch_resource_config(
        &self,
        id: i64,
        memory_reservation: Option<String>,
        memory_limit: Option<String>,
        cpu_reservation: Option<String>,
        cpu_limit: Option<String>,
        replicas: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               memory_reservation = ?, memory_limit = ?,
               cpu_reservation = ?, cpu_limit = ?,
               replicas = COALESCE(?, replicas)
               WHERE id = ?"#,
            memory_reservation,
            memory_limit,
            cpu_reservation,
            cpu_limit,
            replicas,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_github_source(
        &self,
        id: i64,
        repository: Option<String>,
        owner: Option<String>,
        branch: Option<String>,
        build_path: String,
        github_provider_id: Option<i64>,
        auto_deploy: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GITHUB',
               repository = ?, owner = ?, branch = ?, build_path = ?, github_provider_id = ?, auto_deploy = ?,
               gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
               gitlab_path_namespace = NULL, gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            repository,
            owner,
            branch,
            build_path,
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
        gitlab_build_path: String,
        gitlab_path_namespace: Option<String>,
        gitlab_provider_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GITLAB',
               gitlab_project_id = ?, gitlab_repository = ?, gitlab_owner = ?, gitlab_branch = ?,
               gitlab_build_path = ?, gitlab_path_namespace = ?, gitlab_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitea_repository = NULL, gitea_owner = NULL,
               gitea_branch = NULL, bitbucket_repository = NULL, bitbucket_repository_slug = NULL,
               bitbucket_owner = NULL, bitbucket_branch = NULL, docker_image = NULL, docker_username = NULL,
               docker_password = NULL, registry_url = NULL, custom_git_url = NULL, custom_git_branch = NULL,
               custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            gitlab_project_id,
            gitlab_repository,
            gitlab_owner,
            gitlab_branch,
            gitlab_build_path,
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
        gitea_build_path: String,
        gitea_provider_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GITEA',
               gitea_repository = ?, gitea_owner = ?, gitea_branch = ?, gitea_build_path = ?, gitea_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            gitea_repository,
            gitea_owner,
            gitea_branch,
            gitea_build_path,
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
        bitbucket_build_path: String,
        bitbucket_provider_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'BITBUCKET',
               bitbucket_repository = ?, bitbucket_repository_slug = ?, bitbucket_owner = ?,
               bitbucket_branch = ?, bitbucket_build_path = ?, bitbucket_provider_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            bitbucket_repository,
            bitbucket_repository_slug,
            bitbucket_owner,
            bitbucket_branch,
            bitbucket_build_path,
            bitbucket_provider_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_docker_source(
        &self,
        id: i64,
        docker_image: Option<String>,
        docker_username: Option<String>,
        docker_password: Option<String>,
        registry_url: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'DOCKER',
               docker_image = ?, docker_username = ?, docker_password = ?, registry_url = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            docker_image,
            docker_username,
            docker_password,
            registry_url,
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
        custom_git_build_path: String,
        custom_git_ssh_key_id: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'GIT',
               custom_git_url = ?, custom_git_branch = ?, custom_git_build_path = ?, custom_git_ssh_key_id = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL, drop_build_path = NULL
               WHERE id = ?"#,
            custom_git_url,
            custom_git_branch,
            custom_git_build_path,
            custom_git_ssh_key_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn set_drop_source(
        &self,
        id: i64,
        drop_build_path: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET
               source_type = 'DROP',
               drop_build_path = ?,
               repository = NULL, owner = NULL, branch = NULL, gitlab_project_id = NULL, gitlab_repository = NULL,
               gitlab_owner = NULL, gitlab_branch = NULL, gitlab_path_namespace = NULL,
               gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL,
               bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL, bitbucket_branch = NULL,
               docker_image = NULL, docker_username = NULL, docker_password = NULL, registry_url = NULL,
               custom_git_url = NULL, custom_git_branch = NULL, custom_git_ssh_key_id = NULL
               WHERE id = ?"#,
            drop_build_path,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<Application, sqlx::Error> {
        sqlx::query!(
            r#"UPDATE applications SET app_status = ? WHERE id = ?"#,
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
            r#"SELECT a.environment_id, e.project_id, a.server_id
                FROM applications a
                JOIN environments e ON e.id = a.environment_id
                WHERE a.id = ?"#,
        )
        .bind(id)
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn get_spec_row(&self, application_id: i64) -> Result<AppRow, sqlx::Error> {
        sqlx::query_as::<_, AppRow>(
            r#"SELECT
                a.app_name, a.source_type, a.build_type, a.build_args, a.build_secrets,
                a.dockerfile, a.docker_context_path, a.docker_build_stage,
                a.publish_directory, a.is_static_spa, a.build_path, a.command, a.args, a.env_var,
                a.clean_cache, a.network_ids, a.detach_rustploy_network,
                a.memory_reservation, a.memory_limit,
                a.cpu_reservation, a.cpu_limit, a.replicas, a.health_check_swarm,
                a.placement_swarm, a.stop_grace_period_swarm, a.repository, a.owner,
                a.branch, a.gitlab_repository, a.gitlab_owner, a.gitlab_branch,
                a.gitea_repository, a.gitea_branch, a.bitbucket_repository,
                a.bitbucket_owner, a.bitbucket_branch, a.docker_image, a.docker_username,
                a.docker_password, a.registry_url, a.custom_git_url, a.custom_git_branch,
                a.custom_git_ssh_key_id,
                e.env_var AS environment_env, p.env_var AS project_env,
                r.image_prefix AS registry_image_prefix, r.username AS registry_username,
                r.password AS registry_password, r.registry_url AS joined_registry_url,
                ghp.github_private_key AS github_token,
                glp.access_token AS gitlab_token,
                bbp.api_token AS bitbucket_token,
                gtp.access_token AS gitea_token,
                sk.private_key AS ssh_private_key
                FROM applications a
                JOIN environments e ON e.id = a.environment_id
                JOIN projects p ON p.id = e.project_id
                LEFT JOIN registries r ON r.id = a.registry_id
                LEFT JOIN github_providers ghp ON ghp.id = a.github_provider_id
                LEFT JOIN gitlab_providers glp ON glp.id = a.gitlab_provider_id
                LEFT JOIN bitbucket_providers bbp ON bbp.id = a.bitbucket_provider_id
                LEFT JOIN gitea_providers gtp ON gtp.id = a.gitea_provider_id
                LEFT JOIN ssh_keys sk ON sk.id = a.custom_git_ssh_key_id
                WHERE a.id = ?"#,
        )
        .bind(application_id)
        .fetch_one(self.pool.as_ref())
        .await
    }
}
