use crate::utils::builder::compose::spec::{ComposeRuntime, ComposeSource, ComposeSpec};
use crate::utils::builder::errors::AdapterError;
use crate::utils::builder::shared::mapper::{domain, mount_spec};
use crate::utils::builder::spec::{RuntimeType, SourceType};
use crate::utils::git::GitProviderBuilder;
use crate::{
    db::models::{domains::Domain, mounts::Mount},
    utils::builder::env::generate_env_app,
    utils::paths::rustploy_paths,
};

pub struct ComposeRowWithRelations {
    pub compose: ComposeRow,
    pub domains: Vec<Domain>,
    pub mounts: Vec<Mount>,
}

#[derive(sqlx::FromRow)]
pub struct ComposeRow {
    pub app_name: String,
    pub source_type: String,
    pub compose_type: String,
    pub compose_file: String,
    pub env_var: Option<String>,
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
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub custom_git_ssh_key_id: Option<i64>,
    pub enable_submodules: i64,
    pub compose_path: String,
    pub environment_env: String,
    pub project_env: String,
    pub github_token: Option<String>,
    pub gitlab_token: Option<String>,
    pub bitbucket_token: Option<String>,
    pub gitea_token: Option<String>,
    pub ssh_private_key: Option<String>,
}

impl TryFrom<ComposeRowWithRelations> for ComposeSpec {
    type Error = AdapterError;

    fn try_from(data: ComposeRowWithRelations) -> Result<Self, Self::Error> {
        let compose = data.compose;
        let domains = data.domains;
        let mounts = data.mounts;
        let paths = rustploy_paths();
        let app_dir = paths.compose_dir(&compose.app_name);
        Ok(ComposeSpec {
            app_name: compose.app_name.clone(),
            stack_name: compose.app_name.clone(),
            source: source(&compose)?,
            runtime: runtime(&compose.compose_type)?,
            work_directory: paths.compose_source(&compose.app_name),
            compose_path: if compose.source_type == "RAW" {
                format!("{app_dir}/compose.yml")
            } else {
                compose.compose_path
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

            domains: domains
                .into_iter()
                .map(domain)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn source(compose: &ComposeRow) -> Result<ComposeSource, AdapterError> {
    let branch = |value: &Option<String>| value.clone().unwrap_or_else(|| "main".into());
    let source_type = SourceType::from(compose.source_type.as_str());
    match source_type {
        SourceType::Raw => Ok(ComposeSource::Raw {
            content: compose.compose_file.clone(),
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
                SourceType::Github => compose
                    .github_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Gitlab => compose
                    .gitlab_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Bitbucket => compose
                    .bitbucket_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
                SourceType::Gitea => compose
                    .gitea_token
                    .as_ref()
                    .map(|t| crate::utils::git::types::GitAuth::Token(t.clone())),
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
                submodules: compose.enable_submodules != 0,
                protocol,
                auth,
            })
        }
    }
}

fn runtime(value: &str) -> Result<ComposeRuntime, AdapterError> {
    let runtime_type = RuntimeType::from(value);
    match runtime_type {
        RuntimeType::Compose => Ok(ComposeRuntime::Compose),
        RuntimeType::Stack => Ok(ComposeRuntime::Stack),
    }
}
