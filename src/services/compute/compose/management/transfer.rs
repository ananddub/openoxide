use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    core::cache::{AppStateCache, CacheEnum, CacheKey},
    db::queries::FlashQueries,
    flash_gen::compose_extra::{
        ComposeCreateServiceNoServerParams, ComposeCreateServiceWithServerParams,
        ComposeMountCreateParams, ComposePatchCreateParams,
    },
    flash_gen::domains::DomainsCreateAndReturnParams,
    flash_gen::models::{Domains, Mounts, Patches, ServiceGit, ServiceCompose, Services},
    flash_gen::service_compose::CreateServiceComposeParams,
    flash_gen::service_git::{
        SetBitbucketSourceParams, SetCustomGitSourceParams, SetGiteaSourceParams,
        SetGithubSourceParams, SetGitlabSourceParams,
    },
    flash_gen::services::UpdateServiceCommandParams,
    services::compute::compose::{ComposeRecord, queries::generate_app_name},
};

const SCHEMA_VERSION: u32 = 1;

pub struct ComposeTransferService {
    queries: Arc<FlashQueries>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeTransferService {
    fn new(queries: Arc<FlashQueries>, cache: Arc<AppStateCache>) -> Self {
        Self { queries, cache }
    }

    pub async fn export(
        &self,
        id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<ComposeExportBundleDto> {
        let service = self.queries.get_service_by_i_d(id).await?;
        if service.service_type != "COMPOSE" {
            return Err(sqlx::Error::RowNotFound);
        }
        let ext = self.queries.get_service_compose(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        let domains = self.queries.domains_list_by_compose_raw(id).await?;
        let mounts = self.queries.mount_fetch_for_compose(id).await?;
        let patches = self.queries.patches_list_by_compose(id).await?;
        let compose = ComposeExportProject::from_service(
            (&service, &ext, git.as_ref()),
            if include_secrets {
                service.env_var.clone()
            } else {
                None
            },
            if include_secrets {
                ext.compose_file.clone()
            } else {
                redact_compose_secrets(&ext.compose_file)
            },
        );
        Ok(ComposeExportBundleDto {
            schema_version: SCHEMA_VERSION,
            exported_at: chrono::Utc::now().timestamp(),
            secrets_included: include_secrets,
            compose,
            domains: domains.into_iter().map(ComposeExportDomain::from).collect(),
            mounts: mounts.into_iter().map(ComposeExportMount::from).collect(),
            patches: patches.into_iter().map(ComposeExportPatch::from).collect(),
        })
    }

    pub async fn import(&self, input: ImportComposeDto) -> sqlx::Result<ComposeRecord> {
        let bundle: ComposeExportBundleDto = serde_json::from_str(&input.archive)
            .map_err(|error| sqlx::Error::Protocol(format!("invalid compose archive: {error}")))?;
        if bundle.schema_version != SCHEMA_VERSION {
            return Err(sqlx::Error::Protocol(format!(
                "unsupported compose export schema version {}",
                bundle.schema_version
            )));
        }
        // Old EnvironmentRepository::get_by_id returned Option; :one surfaces
        // a miss as RowNotFound.
        self.queries
            .environments_get_by_id(input.target_environment_id)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => {
                    sqlx::Error::Protocol("target environment not found".into())
                }
                other => other,
            })?;
        let name = input.name.unwrap_or_else(|| bundle.compose.name.clone());
        let app_name = generate_app_name(&name);
        // Flash bind params for nullable columns are non-Option; "" is the
        // NULL encoding and is normalized back to None on read.
        let description = bundle.compose.description.clone().unwrap_or_default();
        let service = match input.target_server_id {
            Some(server_id) => {
                self.queries
                    .compose_create_service_with_server(&ComposeCreateServiceWithServerParams {
                        name: name.clone(),
                        app_name: app_name.clone(),
                        description,
                        environment_id: input.target_environment_id,
                        server_id,
                    })
                    .await?
            }
            None => {
                self.queries
                    .compose_create_service_no_server(&ComposeCreateServiceNoServerParams {
                        name: name.clone(),
                        app_name: app_name.clone(),
                        description,
                        environment_id: input.target_environment_id,
                    })
                    .await?
            }
        };
        let id = service.id;

        if let Some(env_var) = bundle.compose.env_var.clone() {
            self.queries.update_service_env_var(&env_var, id).await?;
        }
        if !bundle.compose.command.is_empty() {
            self.queries
                .update_service_command(&UpdateServiceCommandParams {
                    command: bundle.compose.command.clone(),
                    args: String::new(),
                    id,
                })
                .await?;
        }

        self.queries
            .create_service_compose(&CreateServiceComposeParams {
                service_id: id,
                compose_type: bundle.compose.compose_type.clone(),
                compose_file: bundle.compose.compose_file.clone(),
                compose_path: bundle.compose.compose_path.clone(),
                suffix: bundle.compose.suffix.clone(),
                randomize: bundle.compose.randomize,
                isolated_deployment: bundle.compose.isolated_deployment,
                isolated_deployments_volume: bundle.compose.isolated_deployments_volume,
                service_networks: bundle.compose.service_networks.clone(),
            })
            .await?;

        self.import_git_source(id, &bundle.compose).await?;

        if let Err(error) = self.import_relations(id, &bundle).await {
            // ON DELETE CASCADE removes the created rows and relations.
            let _ = self.queries.delete_service(id).await;
            return Err(error);
        }

        let ext = self.queries.get_service_compose(id).await?;
        let git = self.queries.get_service_git_by_i_d(id).await.ok();
        let record = ComposeRecord::from((service, ext, git));
        self.cache
            .insert(CacheKey::Compose(id), CacheEnum::Compose(record.clone()))
            .await;
        Ok(record)
    }

    /// Recreates the archived git source. Old archives carried the custom-git
    /// marker as "GIT"; the unified schema stores "CUSTOM" but accepts both
    /// spellings on import.
    async fn import_git_source(&self, id: i64, compose: &ComposeExportProject) -> sqlx::Result<()> {
        let trigger_type = compose.trigger_type.clone();
        let auto_deploy = compose.auto_deploy;
        let watch_paths = compose.watch_paths.clone().unwrap_or_default();
        let enable_submodules = compose.enable_submodules;
        match compose.source_type.as_str() {
            "GITHUB" => {
                self.queries
                    .set_github_source(&SetGithubSourceParams {
                        service_id: id,
                        trigger_type,
                        repository: compose.repository.clone().unwrap_or_default(),
                        owner: compose.owner.clone().unwrap_or_default(),
                        branch: compose.branch.clone().unwrap_or_default(),
                        github_provider_id: compose.github_provider_id.unwrap_or(0),
                        auto_deploy,
                        watch_paths,
                        enable_submodules,
                    })
                    .await?;
            }
            "GITLAB" => {
                self.queries
                    .set_gitlab_source(&SetGitlabSourceParams {
                        service_id: id,
                        trigger_type,
                        gitlab_project_id: compose.gitlab_project_id.unwrap_or(0),
                        gitlab_repository: compose.gitlab_repository.clone().unwrap_or_default(),
                        gitlab_owner: compose.gitlab_owner.clone().unwrap_or_default(),
                        gitlab_branch: compose.gitlab_branch.clone().unwrap_or_default(),
                        gitlab_build_path: String::new(),
                        gitlab_path_namespace: compose
                            .gitlab_path_namespace
                            .clone()
                            .unwrap_or_default(),
                        gitlab_provider_id: compose.gitlab_provider_id.unwrap_or(0),
                        auto_deploy,
                        watch_paths,
                        enable_submodules,
                    })
                    .await?;
            }
            "GITEA" => {
                self.queries
                    .set_gitea_source(&SetGiteaSourceParams {
                        service_id: id,
                        trigger_type,
                        gitea_repository: compose.gitea_repository.clone().unwrap_or_default(),
                        gitea_owner: compose.gitea_owner.clone().unwrap_or_default(),
                        gitea_branch: compose.gitea_branch.clone().unwrap_or_default(),
                        gitea_build_path: String::new(),
                        gitea_provider_id: compose.gitea_provider_id.unwrap_or(0),
                        auto_deploy,
                        watch_paths,
                        enable_submodules,
                    })
                    .await?;
            }
            "BITBUCKET" => {
                self.queries
                    .set_bitbucket_source(&SetBitbucketSourceParams {
                        service_id: id,
                        trigger_type,
                        bitbucket_repository: compose
                            .bitbucket_repository
                            .clone()
                            .unwrap_or_default(),
                        bitbucket_repository_slug: compose
                            .bitbucket_repository_slug
                            .clone()
                            .unwrap_or_default(),
                        bitbucket_owner: compose.bitbucket_owner.clone().unwrap_or_default(),
                        bitbucket_branch: compose.bitbucket_branch.clone().unwrap_or_default(),
                        bitbucket_build_path: String::new(),
                        bitbucket_provider_id: compose.bitbucket_provider_id.unwrap_or(0),
                        auto_deploy,
                        watch_paths,
                        enable_submodules,
                    })
                    .await?;
            }
            "GIT" | "CUSTOM" => {
                self.queries
                    .set_custom_git_source(&SetCustomGitSourceParams {
                        service_id: id,
                        trigger_type,
                        custom_git_url: compose.custom_git_url.clone().unwrap_or_default(),
                        custom_git_branch: compose.custom_git_branch.clone().unwrap_or_default(),
                        custom_git_build_path: String::new(),
                        custom_git_ssh_key_id: compose.custom_git_ssh_key_id.unwrap_or(0),
                        auto_deploy,
                        watch_paths,
                        enable_submodules,
                    })
                    .await?;
            }
            // RAW / DROP (and anything else) is represented by the absence of
            // a service_git row in the unified schema.
            _ => {}
        }
        Ok(())
    }

    async fn import_relations(&self, id: i64, bundle: &ComposeExportBundleDto) -> sqlx::Result<()> {
        for domain in &bundle.domains {
            self.queries
                .domains_create_and_return(&DomainsCreateAndReturnParams {
                    host: domain.host.clone(),
                    https: domain.https,
                    port: domain.port.unwrap_or(3000),
                    path: domain.path.clone().unwrap_or_default(),
                    internal_path: domain.internal_path.clone().unwrap_or_default(),
                    custom_entrypoint: domain.custom_entrypoint.clone().unwrap_or_default(),
                    service_name: domain.service_name.clone().unwrap_or_default(),
                    custom_cert_resolver: domain.custom_cert_resolver.clone().unwrap_or_default(),
                    strip_path: domain.strip_path,
                    middlewares: domain.middlewares.clone(),
                    domain_type: "COMPOSE".into(),
                    certificate_type: domain.certificate_type.clone(),
                    service_id: id,
                })
                .await?;
        }
        for mount in &bundle.mounts {
            self.queries
                .compose_mount_create(&ComposeMountCreateParams {
                    mount_type: mount.mount_type.clone(),
                    host_path: mount.host_path.clone().unwrap_or_default(),
                    volume_name: mount.volume_name.clone().unwrap_or_default(),
                    file_path: mount.file_path.clone().unwrap_or_default(),
                    content: mount.content.clone().unwrap_or_default(),
                    mount_path: mount.mount_path.clone(),
                    service_id: id,
                })
                .await?;
        }
        for patch in &bundle.patches {
            self.queries
                .compose_patch_create(&ComposePatchCreateParams {
                    patch_type: patch.patch_type.clone(),
                    file_path: patch.file_path.clone(),
                    enabled: patch.enabled,
                    content: patch.content.clone(),
                    service_id: id,
                })
                .await?;
        }
        Ok(())
    }
}

fn redact_compose_secrets(value: &str) -> String {
    value
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("-") && line.contains("/run/secrets/") {
                line.to_string()
            } else if line.to_ascii_lowercase().contains("password")
                || line.to_ascii_lowercase().contains("secret")
                || line.to_ascii_lowercase().contains("token")
            {
                line.split_once(':')
                    .map(|(key, _)| format!("{key}: REDACTED"))
                    .unwrap_or_else(|| line.to_string())
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Archive payload contract ───────────────────────────────────────────
//
// The bundle keeps the JSON field names of the retired compose_projects /
// domains / mounts / patches models so archives exported by older builds
// stay importable (SCHEMA_VERSION is unchanged).

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeExportBundleDto {
    pub schema_version: u32,
    pub exported_at: i64,
    pub secrets_included: bool,
    pub compose: ComposeExportProject,
    pub domains: Vec<ComposeExportDomain>,
    pub mounts: Vec<ComposeExportMount>,
    pub patches: Vec<ComposeExportPatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeExportProject {
    pub id: Option<i64>,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: String,
    pub refresh_token: Option<String>,
    pub source_type: String,
    pub compose_type: String,
    pub compose_status: String,
    pub last_error: Option<String>,
    pub trigger_type: String,
    pub repository: Option<String>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub auto_deploy: i64,
    pub gitlab_project_id: Option<i64>,
    pub gitlab_repository: Option<String>,
    pub gitlab_owner: Option<String>,
    pub gitlab_branch: Option<String>,
    pub gitlab_path_namespace: Option<String>,
    pub bitbucket_repository: Option<String>,
    pub bitbucket_repository_slug: Option<String>,
    pub bitbucket_owner: Option<String>,
    pub bitbucket_branch: Option<String>,
    pub gitea_repository: Option<String>,
    pub gitea_owner: Option<String>,
    pub gitea_branch: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub custom_git_ssh_key_id: Option<i64>,
    pub command: String,
    pub enable_submodules: i64,
    pub compose_path: String,
    pub suffix: String,
    pub randomize: i64,
    pub isolated_deployment: i64,
    pub isolated_deployments_volume: i64,
    pub watch_paths: Option<String>,
    pub service_networks: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub github_provider_id: Option<i64>,
    pub gitlab_provider_id: Option<i64>,
    pub gitea_provider_id: Option<i64>,
    pub bitbucket_provider_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ComposeExportProject {
    /// Builds the legacy archive shape from the unified-schema rows.
    /// `env_var`/`compose_file` carry the (possibly redacted) export values;
    /// `refresh_token` is always dropped.
    fn from_service(
        (service, ext, git): (&Services, &ServiceCompose, Option<&ServiceGit>),
        env_var: Option<String>,
        compose_file: String,
    ) -> Self {
        // Source type resolution mirrors ComposeGetSpecRow: the service_git
        // row wins, otherwise RAW/DROP derive from the stored compose file.
        let source_type = git
            .map(|git| git.source_type.clone())
            .unwrap_or_else(|| {
                if ext.compose_file.is_empty() {
                    "DROP".into()
                } else {
                    "RAW".into()
                }
            });
        Self {
            id: Some(service.id),
            name: service.name.clone(),
            app_name: service.app_name.clone(),
            description: service.description.clone(),
            env_var,
            compose_file,
            refresh_token: None,
            source_type,
            compose_type: ext.compose_type.clone(),
            compose_status: service.app_status.clone(),
            last_error: service.last_error.clone(),
            trigger_type: git
                .map(|git| git.trigger_type.clone())
                .unwrap_or_else(|| "PUSH".into()),
            repository: git.and_then(|git| git.repository.clone()),
            owner: git.and_then(|git| git.owner.clone()),
            branch: git.and_then(|git| git.branch.clone()),
            auto_deploy: git.and_then(|git| git.auto_deploy).unwrap_or(0),
            gitlab_project_id: git.and_then(|git| git.gitlab_project_id),
            gitlab_repository: git.and_then(|git| git.gitlab_repository.clone()),
            gitlab_owner: git.and_then(|git| git.gitlab_owner.clone()),
            gitlab_branch: git.and_then(|git| git.gitlab_branch.clone()),
            gitlab_path_namespace: git.and_then(|git| git.gitlab_path_namespace.clone()),
            bitbucket_repository: git.and_then(|git| git.bitbucket_repository.clone()),
            bitbucket_repository_slug: git.and_then(|git| git.bitbucket_repository_slug.clone()),
            bitbucket_owner: git.and_then(|git| git.bitbucket_owner.clone()),
            bitbucket_branch: git.and_then(|git| git.bitbucket_branch.clone()),
            gitea_repository: git.and_then(|git| git.gitea_repository.clone()),
            gitea_owner: git.and_then(|git| git.gitea_owner.clone()),
            gitea_branch: git.and_then(|git| git.gitea_branch.clone()),
            custom_git_url: git.and_then(|git| git.custom_git_url.clone()),
            custom_git_branch: git.and_then(|git| git.custom_git_branch.clone()),
            custom_git_ssh_key_id: git.and_then(|git| git.custom_git_ssh_key_id),
            command: service.command.clone().unwrap_or_default(),
            enable_submodules: git.map(|git| git.enable_submodules).unwrap_or(0),
            compose_path: ext.compose_path.clone(),
            suffix: ext.suffix.clone(),
            randomize: ext.randomize,
            isolated_deployment: ext.isolated_deployment,
            isolated_deployments_volume: ext.isolated_deployments_volume,
            watch_paths: git.and_then(|git| git.watch_paths.clone()),
            service_networks: ext.service_networks.clone(),
            environment_id: service.environment_id,
            server_id: service.server_id,
            github_provider_id: git.and_then(|git| git.github_provider_id),
            gitlab_provider_id: git.and_then(|git| git.gitlab_provider_id),
            gitea_provider_id: git.and_then(|git| git.gitea_provider_id),
            bitbucket_provider_id: git.and_then(|git| git.bitbucket_provider_id),
            created_at: service.created_at,
            updated_at: service.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeExportDomain {
    pub id: Option<i64>,
    pub host: String,
    pub https: i64,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub internal_path: Option<String>,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    pub strip_path: i64,
    pub middlewares: String,
    pub domain_type: String,
    pub certificate_type: String,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Domains> for ComposeExportDomain {
    fn from(value: Domains) -> Self {
        Self {
            id: Some(value.id),
            host: value.host,
            https: value.https,
            port: value.port,
            path: value.path,
            internal_path: value.internal_path,
            custom_entrypoint: value.custom_entrypoint,
            service_name: value.service_name,
            custom_cert_resolver: value.custom_cert_resolver,
            strip_path: value.strip_path,
            middlewares: value.middlewares,
            domain_type: value.domain_type,
            certificate_type: value.certificate_type,
            application_id: None,
            compose_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeExportMount {
    pub id: Option<i64>,
    pub mount_type: String,
    pub service_type: String,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub mount_path: String,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Mounts> for ComposeExportMount {
    fn from(value: Mounts) -> Self {
        Self {
            id: Some(value.id),
            mount_type: value.mount_type,
            service_type: value.service_type,
            host_path: value.host_path,
            volume_name: value.volume_name,
            file_path: value.file_path,
            content: value.content,
            mount_path: value.mount_path,
            compose_id: Some(value.service_id),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeExportPatch {
    pub id: Option<i64>,
    pub patch_type: String,
    pub file_path: String,
    pub enabled: i64,
    pub content: String,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Patches> for ComposeExportPatch {
    fn from(value: Patches) -> Self {
        Self {
            id: Some(value.id),
            patch_type: value.patch_type,
            file_path: value.file_path,
            enabled: value.enabled,
            content: value.content,
            application_id: None,
            compose_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ImportComposeDto {
    pub target_environment_id: i64,
    pub target_server_id: Option<i64>,
    pub name: Option<String>,
    pub archive: String,
}
