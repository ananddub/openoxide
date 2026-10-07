use crate::flash_gen::models::{
    ApplicationMiddlewares, Domains, Mounts, Patches, Ports, Redirects, Security,
};
use crate::flash_gen::services::ApplicationGetSpecRowRow;
use crate::utils::builder::env::generate_env_app;
use crate::utils::builder::errors::AdapterError;
use crate::utils::builder::shared::mapper::{domain, mount_spec};
use crate::utils::builder::spec::{
    ApplicationSpec, BasicAuthSpec, BuildStrategy, BuildType, PatchSpec, PortSpec, RedirectSpec,
    SourceSpec, SourceType, StructuredMiddlewareSpec,
};
use crate::utils::builder::spec::{RegistryAuth, ResourceSpec};
use crate::utils::git::GitProviderBuilder;
use crate::utils::paths::openoxide_paths;
use std::collections::BTreeMap;

pub struct AppRowWithRelations {
    pub app: ApplicationGetSpecRowRow,
    pub domains: Vec<Domains>,
    pub mounts: Vec<Mounts>,
    pub patches: Vec<Patches>,
    pub ports: Vec<Ports>,
    pub redirects: Vec<Redirects>,
    pub security: Vec<Security>,
    pub middlewares: Vec<ApplicationMiddlewares>,
    pub networks: Vec<String>,
}

impl TryFrom<AppRowWithRelations> for ApplicationSpec {
    type Error = AdapterError;

    fn try_from(data: AppRowWithRelations) -> Result<Self, Self::Error> {
        let app = data.app;
        let domains = data.domains;
        let mounts = data.mounts;
        let patches = data.patches;
        let ports = data.ports;
        let redirects = data.redirects;
        let security = data.security;
        let middlewares = data.middlewares;
        let networks = data.networks;
        let source = source(&app)?;
        let build = build(&app)?;
        let environment = generate_env_app(
            app.environment_env.clone(),
            app.project_env.clone(),
            app.env_var.clone().as_deref().unwrap_or("").into(),
        );
        let registry = registry_auth(&app);
        let image = if source_type_str(&app) == "DOCKER" {
            app.docker_image
                .clone()
                .ok_or(AdapterError::MissingField("docker_image"))?
        } else if let Some(prefix) = &app.registry_image_prefix {
            format!("{}/{}:latest", prefix.trim_end_matches('/'), app.app_name)
        } else {
            format!("{}:latest", app.app_name)
        };
        let source = match source {
            SourceSpec::Docker { image, .. } => SourceSpec::Docker {
                image,
                registry: registry.clone(),
            },
            other => other,
        };
        let work_directory = openoxide_paths().application_code(&app.app_name);
        let domain_specs = domains
            .into_iter()
            .map(domain)
            .collect::<Result<Vec<_>, _>>()?;
        let mount_specs = mounts
            .into_iter()
            .map(|m| mount_spec(m, &openoxide_paths().application_files(&app.app_name)))
            .collect::<Result<Vec<_>, _>>()?;
        let patch_specs = patches
            .into_iter()
            .filter(|patch| patch.enabled != 0)
            .map(|patch| PatchSpec {
                patch_type: patch.patch_type,
                file_path: patch.file_path,
                content: patch.content,
            })
            .collect();
        let port_specs = ports
            .into_iter()
            .map(|port| {
                Ok(PortSpec {
                    published: u16::try_from(port.published_port)
                        .ok()
                        .filter(|port| *port != 0)
                        .ok_or_else(|| AdapterError::InvalidField {
                            field: "published_port",
                            message: "port must be between 1 and 65535".into(),
                        })?,
                    target: u16::try_from(port.target_port)
                        .ok()
                        .filter(|port| *port != 0)
                        .ok_or_else(|| AdapterError::InvalidField {
                            field: "target_port",
                            message: "port must be between 1 and 65535".into(),
                        })?,
                    protocol: port.protocol.to_ascii_lowercase(),
                    mode: port.publish_mode.to_ascii_lowercase(),
                })
            })
            .collect::<Result<Vec<_>, AdapterError>>()?;
        let redirect_specs = redirects
            .into_iter()
            .map(|redirect| RedirectSpec {
                key: redirect.id.to_string(),
                regex: redirect.regex,
                replacement: redirect.replacement,
                permanent: redirect.permanent != 0,
            })
            .collect();
        let basic_auth = security
            .into_iter()
            .map(|entry| {
                let password_hash = if entry.password.starts_with("$2") {
                    entry.password
                } else {
                    bcrypt::hash(entry.password, bcrypt::DEFAULT_COST).map_err(|error| {
                        AdapterError::InvalidField {
                            field: "security.password",
                            message: error.to_string(),
                        }
                    })?
                };
                Ok(BasicAuthSpec {
                    username: entry.username,
                    password_hash,
                })
            })
            .collect::<Result<Vec<_>, AdapterError>>()?;
        let middleware_specs = middlewares
            .into_iter()
            .filter(|item| item.enabled != 0)
            .map(structured_middleware)
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
            image_registry: if source_type_str(&app) == "DOCKER" {
                None
            } else {
                registry
            },
            environment,
            build_args: parse_env(app.build_args.as_deref().unwrap_or("")),
            build_secrets: parse_env(app.build_secrets.as_deref().unwrap_or("")),
            command: app
                .command
                .clone()
                .filter(|v| !v.trim().is_empty())
                .map(|v| shell_words(&v)),
            args: app
                .args
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(|v| {
                    // Stored as a JSON array by newer UIs, or a plain
                    // shell string ("--port 3000") by the general form.
                    serde_json::from_str::<Vec<String>>(v).unwrap_or_else(|_| shell_words(v))
                })
                .unwrap_or_default(),
            replicas: u32::try_from(app.replicas.max(1)).unwrap_or(1),
            networks,
            mounts: mount_specs,
            patches: patch_specs,
            ports: port_specs,
            redirects: redirect_specs,
            basic_auth,
            middlewares: middleware_specs,
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

#[derive(serde::Deserialize)]
struct MiddlewareConfig {
    #[serde(default)]
    headers: BTreeMap<String, String>,
    average: Option<i64>,
    burst: Option<i64>,
    #[serde(default)]
    source_ranges: Vec<String>,
}

fn structured_middleware(
    item: ApplicationMiddlewares,
) -> Result<StructuredMiddlewareSpec, AdapterError> {
    let config: MiddlewareConfig =
        serde_json::from_str(&item.config).map_err(|error| AdapterError::InvalidField {
            field: "application_middleware.config",
            message: error.to_string(),
        })?;
    match item.middleware_type.as_str() {
        "COMPRESS" => Ok(StructuredMiddlewareSpec::Compress { name: item.name }),
        "HEADERS" => Ok(StructuredMiddlewareSpec::Headers {
            name: item.name,
            headers: config.headers,
        }),
        "RATE_LIMIT" => Ok(StructuredMiddlewareSpec::RateLimit {
            name: item.name,
            average: config
                .average
                .ok_or(AdapterError::MissingField("middleware.average"))?,
            burst: config
                .burst
                .ok_or(AdapterError::MissingField("middleware.burst"))?,
        }),
        "IP_ALLOWLIST" => Ok(StructuredMiddlewareSpec::IpAllowList {
            name: item.name,
            source_ranges: config.source_ranges,
        }),
        _ => Err(AdapterError::InvalidField {
            field: "application_middleware.middleware_type",
            message: "unsupported middleware type".into(),
        }),
    }
}

fn source_type_str(a: &ApplicationGetSpecRowRow) -> &str {
    a.source_type.as_deref().unwrap_or("NONE")
}

fn source_type(a: &ApplicationGetSpecRowRow) -> SourceType {
    match source_type_str(a) {
        "CUSTOM" => SourceType::Git,
        other => SourceType::from(other),
    }
}

/// Extracts a token field from a provider `config_json` payload. A missing or
/// invalid config yields no token instead of failing the spec load.
fn config_token(config: &Option<String>, field: &str) -> Option<String> {
    let json = config.as_deref()?;
    let value: serde_json::Value = serde_json::from_str(json).ok()?;
    let token = value.get(field)?.as_str()?.trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

fn source(a: &ApplicationGetSpecRowRow) -> Result<SourceSpec, AdapterError> {
    let branch_val = |v: &Option<String>| {
        v.as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(str::to_owned)
    };
    let source_type = source_type(a);
    match source_type {
        SourceType::Docker => Ok(SourceSpec::Docker {
            image: a
                .docker_image
                .clone()
                .ok_or(AdapterError::MissingField("docker_image"))?,
            registry: None,
        }),
        // The uploaded ZIP is already on disk (installed by the drop-upload
        // flow before the build is queued); nothing to fetch.
        SourceType::Drop => Ok(SourceSpec::Drop),
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
                SourceType::Github => config_token(&a.github_config, "private_key")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Gitlab => config_token(&a.gitlab_config, "access_token")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Bitbucket => config_token(&a.bitbucket_config, "api_token")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Gitea => config_token(&a.gitea_config, "access_token")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Git => a
                    .ssh_private_key
                    .as_ref()
                    .map(|k| crate::utils::git::types::GitAuth::SshKey(k.clone())),
                _ => None,
            };

            let url = provider.repository_url();
            // The clone protocol MUST match the URL scheme. An SSH key is only
            // usable with an SSH URL (git@host:…/ssh://…); attaching a key to an
            // https:// URL previously forced the SSH protocol and produced a
            // malformed command ("unknown option: -i"). Derive from the URL and
            // ignore an unusable key.
            let url_is_ssh = url.starts_with("git@") || url.starts_with("ssh://");
            let auth = if auth_is_ssh_key(&auth) && !url_is_ssh {
                None
            } else {
                auth
            };
            let protocol = if url_is_ssh {
                crate::utils::provider::CloneProtocol::Ssh
            } else {
                crate::utils::provider::CloneProtocol::Https
            };

            Ok(SourceSpec::Git {
                url,
                branch: branch_val(branch),
                submodules: a.enable_submodules.unwrap_or(0) != 0,
                protocol,
                auth,
            })
        }
    }
}
fn auth_is_ssh_key(auth: &Option<crate::utils::git::types::GitAuth>) -> bool {
    matches!(auth, Some(crate::utils::git::types::GitAuth::SshKey(_)))
}

fn build(a: &ApplicationGetSpecRowRow) -> Result<Option<BuildStrategy>, AdapterError> {
    let source_type = source_type(a);
    if source_type == SourceType::Docker {
        return Ok(None);
    }
    let build_type = BuildType::from(a.build_type.as_deref().unwrap_or(""));
    // The spec query collapses NULL build paths to ""; treat empty as unset.
    let build_path = Some(a.build_path.as_str()).filter(|value| !value.is_empty());
    Ok(Some(match build_type {
        BuildType::Dockerfile => BuildStrategy::Dockerfile {
            dockerfile: join_relative(build_path, a.dockerfile.as_deref().unwrap_or("Dockerfile")),
            context: join_relative(build_path, a.docker_context_path.as_deref().unwrap_or(".")),
            target: a.docker_build_stage.clone(),
            no_cache: a.clean_cache.unwrap_or(0) != 0,
        },
        BuildType::Nixpacks => BuildStrategy::Nixpacks,
        BuildType::Paketo => BuildStrategy::Paketo,
        BuildType::Heroku => BuildStrategy::Heroku,
        BuildType::Railpack => BuildStrategy::Railpack {
            version: a
                .railpack_version
                .clone()
                .filter(|v| !v.trim().is_empty())
                .unwrap_or_else(|| "0.15.4".into()),
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
fn registry_auth(a: &ApplicationGetSpecRowRow) -> Option<RegistryAuth> {
    let username = a
        .docker_username
        .clone()
        .filter(|v| !v.is_empty())
        .or_else(|| a.registry_username.clone().filter(|v| !v.is_empty()))?;
    let password = a
        .docker_password
        .clone()
        .filter(|v| !v.is_empty())
        .or_else(|| a.registry_password.clone().filter(|v| !v.is_empty()))?;
    Some(RegistryAuth {
        registry: a
            .registry_url
            .clone()
            .filter(|v| !v.is_empty())
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
