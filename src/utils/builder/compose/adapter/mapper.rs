use crate::flash_gen::models::{Domains, Mounts, Patches};
use crate::flash_gen::service_compose::ComposeGetSpecRowRow;
use crate::utils::builder::compose::spec::{ComposeRuntime, ComposeSource, ComposeSpec};
use crate::utils::builder::env::generate_env_app;
use crate::utils::builder::errors::AdapterError;
use crate::utils::builder::shared::mapper::{domain, mount_spec};
use crate::utils::builder::spec::{PatchSpec, RuntimeType, SourceType};
use crate::utils::git::GitProviderBuilder;
use crate::utils::paths::openoxide_paths;

pub struct ComposeRowWithRelations {
    pub compose: ComposeGetSpecRowRow,
    pub domains: Vec<Domains>,
    pub mounts: Vec<Mounts>,
    pub patches: Vec<Patches>,
    pub service_networks: Vec<crate::utils::builder::compose::spec::ComposeServiceNetworkSpec>,
}

impl TryFrom<ComposeRowWithRelations> for ComposeSpec {
    type Error = AdapterError;

    fn try_from(data: ComposeRowWithRelations) -> Result<Self, Self::Error> {
        let compose = data.compose;
        let domains = data.domains;
        let mounts = data.mounts;
        let patches = data.patches;
        let service_networks = data.service_networks;
        let paths = openoxide_paths();
        let app_dir = paths.compose_dir(&compose.app_name);
        Ok(ComposeSpec {
            app_name: compose.app_name.clone(),
            stack_name: compose.app_name.clone(),
            source: source(&compose)?,
            runtime: runtime(compose.compose_type.as_deref().unwrap_or(""))?,
            work_directory: paths.compose_source(&compose.app_name),
            compose_path: if source_type_str(&compose) == "RAW" {
                format!("{app_dir}/compose.yml")
            } else {
                compose.compose_path.clone().unwrap_or_default()
            },
            rendered_stack_file: format!("{app_dir}/rendered-stack.yml"),
            env_file: format!("{app_dir}/.env"),
            environment: generate_env_app(
                compose.environment_env,
                compose.project_env,
                compose.env_var.unwrap_or_default(),
            ),
            mounts: mounts
                .into_iter()
                .map(|mount| mount_spec(mount, &paths.compose_files(&compose.app_name)))
                .collect::<Result<Vec<_>, _>>()?,
            patches: patches
                .into_iter()
                .filter(|patch| patch.enabled != 0)
                .map(|patch| PatchSpec {
                    patch_type: patch.patch_type,
                    file_path: patch.file_path,
                    content: patch.content,
                })
                .collect(),

            domains: domains
                .into_iter()
                .map(domain)
                .collect::<Result<Vec<_>, _>>()?,
            service_networks,
            randomize: compose.randomize.unwrap_or(0) != 0,
            suffix: compose.suffix.clone().unwrap_or_default(),
            isolated_deployment: compose.isolated_deployment.unwrap_or(0) != 0,
            isolated_deployments_volume: compose.isolated_deployments_volume.unwrap_or(0) != 0,
            command: {
                let command = compose.command.trim();
                (!command.is_empty()).then(|| command.to_owned())
            },
        })
    }
}

fn source_type_str(compose: &ComposeGetSpecRowRow) -> &str {
    compose.source_type.as_deref().unwrap_or("DROP")
}

fn source(compose: &ComposeGetSpecRowRow) -> Result<ComposeSource, AdapterError> {
    let branch = |value: &Option<String>| value.clone().unwrap_or_else(|| "main".into());
    let source_type = match source_type_str(compose) {
        "CUSTOM" => SourceType::Git,
        other => SourceType::from(other),
    };
    match source_type {
        SourceType::Raw => Ok(ComposeSource::Raw {
            content: compose.compose_file.clone().unwrap_or_default(),
        }),
        SourceType::Drop => Ok(ComposeSource::Drop),
        _ => {
            let provider = GitProviderBuilder::new(source_type)
                .github(compose.owner.as_deref(), compose.repository.as_deref())
                .gitlab(
                    compose.gitlab_owner.as_deref(),
                    compose.gitlab_repository.as_deref(),
                )
                .bitbucket(
                    compose.bitbucket_owner.as_deref(),
                    compose.bitbucket_repository.as_deref(),
                )
                .gitea(compose.gitea_repository.as_deref())
                .custom(compose.custom_git_url.as_deref())
                .build()?;

            let branch_val = match source_type {
                SourceType::Github => &compose.branch,
                SourceType::Gitlab => &compose.gitlab_branch,
                SourceType::Bitbucket => &compose.bitbucket_branch,
                SourceType::Gitea => &compose.gitea_branch,
                SourceType::Git => &compose.custom_git_branch,
                _ => &None, // Should be caught by builder
            };

            let auth = match source_type {
                SourceType::Github => config_token(&compose.github_config, "private_key")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Gitlab => config_token(&compose.gitlab_config, "access_token")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Bitbucket => config_token(&compose.bitbucket_config, "api_token")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Gitea => config_token(&compose.gitea_config, "access_token")
                    .map(crate::utils::git::types::GitAuth::Token),
                SourceType::Git => compose
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

            Ok(ComposeSource::Git {
                url: provider.repository_url(),
                branch: branch(branch_val),
                submodules: compose.enable_submodules.unwrap_or(0) != 0,
                protocol,
                auth,
            })
        }
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

fn runtime(value: &str) -> Result<ComposeRuntime, AdapterError> {
    let runtime_type = RuntimeType::from(value);
    match runtime_type {
        RuntimeType::Compose => Ok(ComposeRuntime::Compose),
        RuntimeType::Stack => Ok(ComposeRuntime::Stack),
    }
}
