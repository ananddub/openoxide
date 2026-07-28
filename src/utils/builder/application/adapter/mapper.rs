use crate::db::models::{domains::Domain, mounts::Mount};
use crate::utils::builder::env::generate_env_app;
use crate::utils::builder::errors::AdapterError;
use crate::utils::builder::shared::mapper::{domain, mount_spec};
use crate::utils::builder::spec::{
    ApplicationSpec, BuildStrategy, BuildType, SourceSpec, SourceType,
};
use crate::utils::builder::spec::{RegistryAuth, ResourceSpec};
use crate::utils::git::GitProviderBuilder;
use crate::utils::paths::rustploy_paths;
use std::collections::BTreeMap;

pub struct AppRowWithRelations {
    pub app: AppRow,
    pub domains: Vec<Domain>,
    pub mounts: Vec<Mount>,
    pub networks: Vec<String>,
}

impl TryFrom<AppRowWithRelations> for ApplicationSpec {
    type Error = AdapterError;

    fn try_from(data: AppRowWithRelations) -> Result<Self, Self::Error> {
        let app = data.app;
        let domains = data.domains;
        let mounts = data.mounts;
        let networks = data.networks;
        let source = source(&app)?;
        let build = build(&app)?;
        let environment = generate_env_app(
            app.environment_env.clone(),
            app.project_env.clone(),
            app.env_var.clone().as_deref().unwrap_or("").into(),
        );
        let registry = registry_auth(&app);
        let image = if app.source_type == "DOCKER" {
            app.docker_image
                .clone()
                .ok_or(AdapterError::MissingField("docker_image"))?
        } else if let Some(prefix) = &app.registry_image_prefix {
            format!("{}/{}:latest", prefix.trim_end_matches('/'), app.app_name)
        } else {
            format!("{}:latest", app.app_name)
        };
        let source = match source {
            SourceSpec::Docker { image, .. } => SourceSpec::Docker { image, registry },
            other => other,
        };
        let work_directory = rustploy_paths().application_code(&app.app_name);
        let domain_specs = domains
            .into_iter()
            .map(domain)
            .collect::<Result<Vec<_>, _>>()?;
        let mount_specs = mounts
            .into_iter()
            .map(|m| mount_spec(m, &rustploy_paths().application_files(&app.app_name)))
            .collect::<Result<Vec<_>, _>>()?;
        let healthcheck = app
            .health_check_swarm
            .as_deref()
            .filter(|v| !v.trim().is_empty())
            .map(serde_json::from_str)
            .transpose()
            .map_err(|e| AdapterError::InvalidField {
                field: "health_check_swarm",
                message: e.to_string(),
            })?;
        let placement_constraints = app
            .placement_swarm
            .as_deref()
            .and_then(|v| serde_json::from_str::<serde_json::Value>(v).ok())
            .and_then(|v| {
                v.get("constraints")
                    .or_else(|| v.get("Constraints"))
                    .cloned()
            })
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();
        Ok(ApplicationSpec {
            app_name: app.app_name.clone(),
            stack_name: app.app_name.clone(),
            source,
            build,
            work_directory,
            image,
            environment,
            build_args: parse_env(app.build_args.as_deref().unwrap_or("")),
            build_secrets: parse_env(app.build_secrets.as_deref().unwrap_or("")),
            command: app.command.map(|v| shell_words(&v)),
            args: app
                .args
                .as_deref()
                .and_then(|v| serde_json::from_str(v).ok())
                .unwrap_or_default(),
            replicas: u32::try_from(app.replicas.max(1)).unwrap_or(1),
            networks,
            mounts: mount_specs,
            domains: domain_specs,
            resources: ResourceSpec {
                memory_limit: app.memory_limit,
                memory_reservation: app.memory_reservation,
                cpu_limit: app.cpu_limit,
                cpu_reservation: app.cpu_reservation,
            },
            healthcheck,
            placement_constraints,
            stop_grace_period: app.stop_grace_period_swarm.map(|v| format!("{v}s")),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct AppRow {
    pub app_name: String,
    pub source_type: String,
    pub build_type: String,
    pub build_args: Option<String>,
    pub build_secrets: Option<String>,
    pub dockerfile: Option<String>,
    pub docker_context_path: Option<String>,
    pub docker_build_stage: Option<String>,
    pub publish_directory: Option<String>,
    pub is_static_spa: Option<i64>,
    pub build_path: Option<String>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env_var: Option<String>,
    pub clean_cache: i64,
    pub network_ids: String,
    pub detach_rustploy_network: i64,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: i64,
    pub health_check_swarm: Option<String>,
    pub placement_swarm: Option<String>,
    pub stop_grace_period_swarm: Option<i64>,
    pub repository: Option<String>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub gitlab_repository: Option<String>,
    pub gitlab_owner: Option<String>,
    pub gitlab_branch: Option<String>,
    pub gitea_repository: Option<String>,
    pub gitea_branch: Option<String>,
    pub bitbucket_repository: Option<String>,
    pub bitbucket_owner: Option<String>,
    pub bitbucket_branch: Option<String>,
    pub docker_image: Option<String>,
    pub docker_username: Option<String>,
    pub docker_password: Option<String>,
    pub registry_url: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub custom_git_ssh_key_id: Option<i64>,
    pub environment_env: String,
    pub project_env: String,
    pub registry_image_prefix: Option<String>,
    pub registry_username: Option<String>,
    pub registry_password: Option<String>,
    pub joined_registry_url: Option<String>,
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
    pub bitbucket_token: Option<String>,
    pub gitea_token: Option<String>,
    pub ssh_private_key: Option<String>,
}

fn source(a: &AppRow) -> Result<SourceSpec, AdapterError> {
    let branch_val = |v: &Option<String>| {
        v.as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(str::to_owned)
    };
    let source_type = SourceType::from(a.source_type.as_str());
    match source_type {
        SourceType::Docker => Ok(SourceSpec::Docker {
            image: a
                .docker_image
                .clone()
                .ok_or(AdapterError::MissingField("docker_image"))?,
            registry: None,
        }),
        _ => {
            let provider = GitProviderBuilder::new(source_type)
                .github(a.owner.as_deref(), a.repository.as_deref())
                .gitlab(a.gitlab_owner.as_deref(), a.gitlab_repository.as_deref())
                .bitbucket(
                    a.bitbucket_owner.as_deref(),
                    a.bitbucket_repository.as_deref(),
                )
                .gitea(a.gitea_repository.as_deref())
                .custom(a.custom_git_url.as_deref())
                .build()?;

            let branch = match source_type {
                SourceType::Github => &a.branch,
                SourceType::Gitlab => &a.gitlab_branch,
                SourceType::Bitbucket => &a.bitbucket_branch,
                SourceType::Gitea => &a.gitea_branch,
                SourceType::Git => &a.custom_git_branch,
                _ => &None, // Should be caught by builder
            };

            let auth = match source_type {
                SourceType::Github => a
                    .github_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Gitlab => a
                    .gitlab_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Bitbucket => a
                    .bitbucket_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Gitea => a
                    .gitea_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Git => a
                    .ssh_private_key
                    .as_ref()
                    .map(|k| crate::utils::git::types::GitAuth::SshKey(k.clone())),
                _ => None,
            };

            let protocol = if let Some(crate::utils::git::types::GitAuth::SshKey(_)) = auth {
                crate::utils::provider::CloneProtocol::Ssh
            } else {
                crate::utils::provider::CloneProtocol::Https
            };

            Ok(SourceSpec::Git {
                url: provider.repository_url(),
                branch: branch_val(branch),
                submodules: false,
                protocol,
                auth,
            })
        }
    }
}
fn build(a: &AppRow) -> Result<Option<BuildStrategy>, AdapterError> {
    let source_type = SourceType::from(a.source_type.as_str());
    if source_type == SourceType::Docker {
        return Ok(None);
    }
    let build_type = BuildType::from(a.build_type.as_str());
    Ok(Some(match build_type {
        BuildType::Dockerfile => BuildStrategy::Dockerfile {
            dockerfile: join_relative(
                a.build_path.as_deref(),
                a.dockerfile.as_deref().unwrap_or("Dockerfile"),
            ),
            context: join_relative(
                a.build_path.as_deref(),
                a.docker_context_path.as_deref().unwrap_or("."),
            ),
            target: a.docker_build_stage.clone(),
            no_cache: a.clean_cache != 0,
        },
        BuildType::Nixpacks => BuildStrategy::Nixpacks,
        BuildType::Paketo => BuildStrategy::Paketo,
        BuildType::Heroku => BuildStrategy::Heroku,
        BuildType::Railpack => BuildStrategy::Railpack {
            version: "0.15.4".into(),
        },
        BuildType::Static => BuildStrategy::Static {
            publish_directory: a
                .publish_directory
                .clone()
                .ok_or(AdapterError::MissingField("publish_directory"))?,
            spa: a.is_static_spa.unwrap_or(0) != 0,
        },
    }))
}
fn registry_auth(a: &AppRow) -> Option<RegistryAuth> {
    let username = a
        .docker_username
        .clone()
        .or_else(|| a.registry_username.clone())?;
    let password = a
        .docker_password
        .clone()
        .or_else(|| a.registry_password.clone())?;
    Some(RegistryAuth {
        registry: a
            .registry_url
            .clone()
            .or_else(|| a.joined_registry_url.clone())
            .unwrap_or_default(),
        username,
        password,
    })
}

fn parse_env(value: &str) -> BTreeMap<String, String> {
    value
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (k, v) = line.split_once('=')?;
            Some((k.trim().into(), v.trim().into()))
        })
        .collect()
}

fn shell_words(value: &str) -> Vec<String> {
    value.split_whitespace().map(str::to_owned).collect()
}

fn join_relative(base: Option<&str>, child: &str) -> String {
    let base = base.unwrap_or("").trim().trim_matches('/');
    let child = child.trim().trim_start_matches('/');
    if base.is_empty() {
        if child.is_empty() {
            ".".into()
        } else {
            child.into()
        }
    } else if child.is_empty() || child == "." {
        base.into()
    } else {
        format!("{base}/{child}")
    }
}
