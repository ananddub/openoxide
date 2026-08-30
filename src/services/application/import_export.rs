use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    db::queries::FlashQueries,
    flash_gen::{
        application_middlewares::ApplicationMiddlewaresCreateParams, models,
        mount::MountCreateForApplicationParams, patches::PatchesCreateParams,
        ports::PortsCreateParams, security::SecurityCreateParams,
    },
    services::application::{queries::generate_app_name, redirect::Redirect, ApplicationRecord},
};

const APPLICATION_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Export bundle for a single application service.
///
/// The deleted `applications` table is now split across the unified
/// `services` / `service_application` / `service_git` tables, so the bundle
/// carries the three generated models instead of the old monolithic
/// `db::models::applications::Application`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationExportBundleDto {
    pub schema_version: u32,
    pub exported_at: i64,
    pub secrets_included: bool,
    pub application: models::Services,
    pub service_application: models::ServiceApplication,
    pub service_git: Option<models::ServiceGit>,
    pub domains: Vec<models::Domains>,
    pub ports: Vec<models::Ports>,
    pub mounts: Vec<models::Mounts>,
    pub redirects: Vec<Redirect>,
    pub security: Vec<models::Security>,
    pub patches: Vec<models::Patches>,
    pub middlewares: Vec<models::ApplicationMiddlewares>,
}

#[derive(Debug, Deserialize)]
pub struct ImportApplicationDto {
    pub target_environment_id: i64,
    pub target_server_id: Option<i64>,
    pub name: Option<String>,
    pub archive: String,
}

/// Flash bind parameters are non-optional for nullable columns, so `None` is
/// written as `""`; normalize empty strings back to `None` on read to preserve
/// the previous repository semantics in the exported bundle.
fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.is_empty())
}

fn normalize_domain(row: models::Domains) -> models::Domains {
    models::Domains {
        port: row.port.filter(|port| *port != 0),
        path: non_empty(row.path),
        internal_path: non_empty(row.internal_path),
        custom_entrypoint: non_empty(row.custom_entrypoint),
        service_name: non_empty(row.service_name),
        custom_cert_resolver: non_empty(row.custom_cert_resolver),
        ..row
    }
}

fn normalize_mount(row: models::Mounts) -> models::Mounts {
    models::Mounts {
        host_path: non_empty(row.host_path),
        volume_name: non_empty(row.volume_name),
        file_path: non_empty(row.file_path),
        content: non_empty(row.content),
        ..row
    }
}

pub struct ApplicationTransferService {
    queries: Arc<FlashQueries>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ApplicationTransferService {
    fn new(queries: Arc<FlashQueries>, cache: Arc<AppStateCache>) -> Self {
        Self { queries, cache }
    }

    pub async fn export(
        &self,
        application_id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<ApplicationExportBundleDto> {
        let mut application = self
            .queries
            .app_sub_get_application_by_i_d(application_id)
            .await?;
        let mut service_application = self
            .queries
            .get_service_application(application_id)
            .await?;
        let service_git = match self.queries.get_service_git_by_i_d(application_id).await {
            Ok(git) => Some(git),
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error),
        };
        let domains = self
            .queries
            .domains_list_by_application_raw(application_id)
            .await?
            .into_iter()
            .map(normalize_domain)
            .collect();
        let ports = self.queries.ports_list_by_application(application_id).await?;
        let mounts = self
            .queries
            .mount_fetch_for_application(application_id)
            .await?
            .into_iter()
            .map(normalize_mount)
            .collect();
        let redirects = self.list_redirects(application_id).await?;
        let mut security = self.queries.security_list_by_application(application_id).await?;
        let patches = self.queries.patches_list_by_application(application_id).await?;
        let middlewares = self
            .queries
            .application_middlewares_list_by_application(application_id)
            .await?;

        application.refresh_token = None;
        if !include_secrets {
            application.env_var = None;
            service_application.build_secrets = None;
            service_application.preview_env = None;
            service_application.preview_build_secrets = None;
            service_application.docker_password = None;
            for item in &mut security {
                item.password.clear();
            }
        }

        Ok(ApplicationExportBundleDto {
            schema_version: APPLICATION_EXPORT_SCHEMA_VERSION,
            exported_at: chrono::Utc::now().timestamp(),
            secrets_included: include_secrets,
            application,
            service_application,
            service_git,
            domains,
            ports,
            mounts,
            redirects,
            security,
            patches,
            middlewares,
        })
    }

    pub async fn import(&self, input: ImportApplicationDto) -> sqlx::Result<ApplicationRecord> {
        let bundle: ApplicationExportBundleDto =
            serde_json::from_str(&input.archive).map_err(|error| {
                sqlx::Error::Protocol(format!("invalid application archive: {error}"))
            })?;
        if bundle.schema_version != APPLICATION_EXPORT_SCHEMA_VERSION {
            return Err(sqlx::Error::Protocol(format!(
                "unsupported application export schema version {}",
                bundle.schema_version
            )));
        }
        match self
            .queries
            .environments_get_by_id(input.target_environment_id)
            .await
        {
            Ok(_) => {}
            Err(sqlx::Error::RowNotFound) => {
                return Err(sqlx::Error::Protocol("target environment not found".into()));
            }
            Err(error) => return Err(error),
        }

        let name = input
            .name
            .unwrap_or_else(|| bundle.application.name.clone());
        let now = chrono::Utc::now().timestamp();
        let mut service = bundle.application.clone();
        service.id = 0;
        service.service_type = "APPLICATION".into();
        service.name = name.clone();
        service.app_name = generate_app_name(&name);
        service.app_status = "IDLE".into();
        service.environment_id = input.target_environment_id;
        service.server_id = input.target_server_id;
        service.refresh_token = None;
        service.created_at = now;
        service.updated_at = now;

        let application_id = self.insert_service(&service).await?;
        let result = self.import_relations(application_id, bundle, now).await;
        if let Err(error) = result {
            if let Err(cleanup_error) = self.queries.delete_service(application_id).await {
                tracing::error!(application_id, %cleanup_error, "failed to clean up partial application import");
            }
            return Err(error);
        }

        let service = self
            .queries
            .app_sub_get_application_by_i_d(application_id)
            .await?;
        let service_application = self
            .queries
            .get_service_application(application_id)
            .await?;
        let service_git = match self.queries.get_service_git_by_i_d(application_id).await {
            Ok(git) => Some(git),
            Err(sqlx::Error::RowNotFound) => None,
            Err(error) => return Err(error),
        };
        let record = ApplicationRecord::from((service, service_application, service_git));
        self.cache
            .insert(
                CacheKey::Application(application_id),
                CacheEnum::Application(record.clone()),
            )
            .await;
        Ok(record)
    }

    /// Raw SQL is required for redirects because the FlashORM schema parser
    /// silently drops the `unique_config_key` column.
    async fn list_redirects(&self, service_id: i64) -> sqlx::Result<Vec<Redirect>> {
        sqlx::query_as::<_, Redirect>(
            "SELECT id, regex, replacement, permanent, unique_config_key, service_id, created_at, updated_at \
             FROM redirects WHERE service_id = ? ORDER BY id",
        )
        .bind(service_id)
        .fetch_all(&self.queries.pool)
        .await
    }

    /// Raw SQL INSERT for the `services` row: the generated `CreateService`
    /// binds `server_id` as a non-optional integer (0 for `None`), which
    /// violates the `server_id` foreign key when no target server is given.
    async fn insert_service(&self, service: &models::Services) -> sqlx::Result<i64> {
        let result = sqlx::query(
            "INSERT INTO services \
             (service_type, name, app_name, description, icon, app_status, last_error, env_var, \
              command, args, refresh_token, docker_image, memory_reservation, memory_limit, \
              cpu_reservation, cpu_limit, replicas, health_check_swarm, restart_policy_swarm, \
              placement_swarm, update_config_swarm, rollback_config_swarm, mode_swarm, \
              labels_swarm, network_swarm, endpoint_spec_swarm, ulimits_swarm, \
              stop_grace_period_swarm, network_ids, detach_rustploy_network, environment_id, \
              server_id, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&service.service_type)
        .bind(&service.name)
        .bind(&service.app_name)
        .bind(&service.description)
        .bind(&service.icon)
        .bind(&service.app_status)
        .bind(&service.last_error)
        .bind(&service.env_var)
        .bind(&service.command)
        .bind(&service.args)
        .bind(&service.refresh_token)
        .bind(&service.docker_image)
        .bind(&service.memory_reservation)
        .bind(&service.memory_limit)
        .bind(&service.cpu_reservation)
        .bind(&service.cpu_limit)
        .bind(service.replicas)
        .bind(&service.health_check_swarm)
        .bind(&service.restart_policy_swarm)
        .bind(&service.placement_swarm)
        .bind(&service.update_config_swarm)
        .bind(&service.rollback_config_swarm)
        .bind(&service.mode_swarm)
        .bind(&service.labels_swarm)
        .bind(&service.network_swarm)
        .bind(&service.endpoint_spec_swarm)
        .bind(&service.ulimits_swarm)
        .bind(&service.stop_grace_period_swarm)
        .bind(&service.network_ids)
        .bind(service.detach_rustploy_network)
        .bind(service.environment_id)
        .bind(&service.server_id)
        .bind(service.created_at)
        .bind(service.updated_at)
        .execute(&self.queries.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    /// Raw SQL INSERT so every nullable build/preview column keeps its
    /// exported NULL instead of the empty-string bind the generated query
    /// would produce.
    async fn insert_service_application(
        &self,
        ext: &models::ServiceApplication,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO service_application \
             (service_id, build_type, build_args, build_secrets, dockerfile, docker_context_path, \
              docker_build_stage, publish_directory, is_static_spa, create_env_file, \
              railpack_version, heroku_version, build_path, clean_cache, drop_build_path, \
              docker_username, docker_password, registry_url, build_server_id, registry_id, \
              rollback_registry_id, build_registry_id, rollback_active, preview_env, \
              preview_build_args, preview_build_secrets, preview_labels, preview_wildcard, \
              preview_port, preview_https, preview_path, preview_certificate_type, \
              preview_custom_cert_resolver, preview_limit, is_preview_deployments_active, \
              preview_require_collaborator_permissions, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(ext.service_id)
        .bind(&ext.build_type)
        .bind(&ext.build_args)
        .bind(&ext.build_secrets)
        .bind(&ext.dockerfile)
        .bind(&ext.docker_context_path)
        .bind(&ext.docker_build_stage)
        .bind(&ext.publish_directory)
        .bind(&ext.is_static_spa)
        .bind(ext.create_env_file)
        .bind(&ext.railpack_version)
        .bind(&ext.heroku_version)
        .bind(&ext.build_path)
        .bind(ext.clean_cache)
        .bind(&ext.drop_build_path)
        .bind(&ext.docker_username)
        .bind(&ext.docker_password)
        .bind(&ext.registry_url)
        .bind(&ext.build_server_id)
        .bind(&ext.registry_id)
        .bind(&ext.rollback_registry_id)
        .bind(&ext.build_registry_id)
        .bind(ext.rollback_active)
        .bind(&ext.preview_env)
        .bind(&ext.preview_build_args)
        .bind(&ext.preview_build_secrets)
        .bind(&ext.preview_labels)
        .bind(&ext.preview_wildcard)
        .bind(&ext.preview_port)
        .bind(ext.preview_https)
        .bind(&ext.preview_path)
        .bind(&ext.preview_certificate_type)
        .bind(&ext.preview_custom_cert_resolver)
        .bind(&ext.preview_limit)
        .bind(ext.is_preview_deployments_active)
        .bind(ext.preview_require_collaborator_permissions)
        .bind(ext.updated_at)
        .execute(&self.queries.pool)
        .await?;
        Ok(())
    }

    /// Raw SQL INSERT so every nullable git column keeps its exported NULL
    /// instead of the empty-string bind the generated query would produce.
    async fn insert_service_git(&self, git: &models::ServiceGit) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO service_git \
             (service_id, source_type, trigger_type, auto_deploy, watch_paths, enable_submodules, \
              repository, owner, branch, gitlab_project_id, gitlab_repository, gitlab_owner, \
              gitlab_branch, gitlab_build_path, gitlab_path_namespace, gitea_repository, \
              gitea_owner, gitea_branch, gitea_build_path, bitbucket_repository, \
              bitbucket_repository_slug, bitbucket_owner, bitbucket_branch, \
              bitbucket_build_path, custom_git_url, custom_git_branch, custom_git_build_path, \
              custom_git_ssh_key_id, github_provider_id, gitlab_provider_id, gitea_provider_id, \
              bitbucket_provider_id, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(git.service_id)
        .bind(&git.source_type)
        .bind(&git.trigger_type)
        .bind(&git.auto_deploy)
        .bind(&git.watch_paths)
        .bind(git.enable_submodules)
        .bind(&git.repository)
        .bind(&git.owner)
        .bind(&git.branch)
        .bind(&git.gitlab_project_id)
        .bind(&git.gitlab_repository)
        .bind(&git.gitlab_owner)
        .bind(&git.gitlab_branch)
        .bind(&git.gitlab_build_path)
        .bind(&git.gitlab_path_namespace)
        .bind(&git.gitea_repository)
        .bind(&git.gitea_owner)
        .bind(&git.gitea_branch)
        .bind(&git.gitea_build_path)
        .bind(&git.bitbucket_repository)
        .bind(&git.bitbucket_repository_slug)
        .bind(&git.bitbucket_owner)
        .bind(&git.bitbucket_branch)
        .bind(&git.bitbucket_build_path)
        .bind(&git.custom_git_url)
        .bind(&git.custom_git_branch)
        .bind(&git.custom_git_build_path)
        .bind(&git.custom_git_ssh_key_id)
        .bind(&git.github_provider_id)
        .bind(&git.gitlab_provider_id)
        .bind(&git.gitea_provider_id)
        .bind(&git.bitbucket_provider_id)
        .bind(git.created_at)
        .bind(git.updated_at)
        .execute(&self.queries.pool)
        .await?;
        Ok(())
    }

    async fn import_relations(
        &self,
        application_id: i64,
        bundle: ApplicationExportBundleDto,
        now: i64,
    ) -> sqlx::Result<()> {
        let mut service_application = bundle.service_application;
        service_application.service_id = application_id;
        service_application.build_server_id = None;
        service_application.registry_id = None;
        service_application.rollback_registry_id = None;
        service_application.build_registry_id = None;
        service_application.updated_at = now;
        self.insert_service_application(&service_application)
            .await?;

        if let Some(mut git) = bundle.service_git {
            git.service_id = application_id;
            git.github_provider_id = None;
            git.gitlab_provider_id = None;
            git.gitea_provider_id = None;
            git.bitbucket_provider_id = None;
            git.custom_git_ssh_key_id = None;
            git.created_at = now;
            git.updated_at = now;
            self.insert_service_git(&git).await?;
        }

        for item in bundle.domains {
            // Raw SQL keeps the nullable routing columns (port, path, ...)
            // as real NULLs, matching the old repository INSERT exactly.
            sqlx::query(
                "INSERT INTO domains \
                 (host, https, port, path, internal_path, custom_entrypoint, service_name, \
                  custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type, \
                  service_id) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'APPLICATION', ?, ?)",
            )
            .bind(&item.host)
            .bind(item.https)
            .bind(item.port)
            .bind(&item.path)
            .bind(&item.internal_path)
            .bind(&item.custom_entrypoint)
            .bind(&item.service_name)
            .bind(&item.custom_cert_resolver)
            .bind(item.strip_path)
            .bind(&item.middlewares)
            .bind(&item.certificate_type)
            .bind(application_id)
            .execute(&self.queries.pool)
            .await?;
        }
        for item in bundle.ports {
            let params = PortsCreateParams {
                published_port: item.published_port,
                target_port: item.target_port,
                protocol: item.protocol,
                publish_mode: item.publish_mode,
                service_id: application_id,
                created_at: item.created_at,
            };
            self.queries.ports_create(&params).await?;
        }
        for item in bundle.mounts {
            let params = MountCreateForApplicationParams {
                mount_type: item.mount_type,
                host_path: item.host_path.unwrap_or_default(),
                volume_name: item.volume_name.unwrap_or_default(),
                file_path: item.file_path.unwrap_or_default(),
                content: item.content.unwrap_or_default(),
                mount_path: item.mount_path,
                service_id: application_id,
            };
            self.queries.mount_create_for_application(&params).await?;
        }
        for item in bundle.redirects {
            // Raw SQL: unique_config_key is invisible to the generator, and
            // the old repository INSERT preserved created_at/updated_at.
            sqlx::query(
                "INSERT INTO redirects \
                 (regex, replacement, permanent, unique_config_key, service_id, created_at, \
                  updated_at) \
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&item.regex)
            .bind(&item.replacement)
            .bind(item.permanent)
            .bind(item.unique_config_key)
            .bind(application_id)
            .bind(item.created_at)
            .bind(item.updated_at)
            .execute(&self.queries.pool)
            .await?;
        }
        for item in bundle.security {
            if item.password.is_empty() {
                continue;
            }
            let params = SecurityCreateParams {
                username: item.username,
                password: item.password,
                service_id: application_id,
                created_at: item.created_at,
            };
            self.queries.security_create(&params).await?;
        }
        for item in bundle.patches {
            let params = PatchesCreateParams {
                patch_type: item.patch_type,
                file_path: item.file_path,
                enabled: item.enabled,
                content: item.content,
                service_id: application_id,
                created_at: item.created_at,
                updated_at: item.updated_at,
            };
            self.queries.patches_create(&params).await?;
        }
        for item in bundle.middlewares {
            let params = ApplicationMiddlewaresCreateParams {
                service_id: application_id,
                name: item.name,
                middleware_type: item.middleware_type,
                enabled: item.enabled,
                config: item.config,
            };
            self.queries.application_middlewares_create(&params).await?;
        }
        Ok(())
    }
}
