use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::compose::{ComposeOperationResult, ComposeRecord};

pub mod management;

#[derive(Debug, Clone, Validate, Deserialize, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ComposeServiceNetworkDto {
    #[validate(length(min = 1, max = 255))]
    pub service_name: String,
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateComposeDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    #[serde(default = "default_source_type")]
    pub source_type: String,
    #[serde(default = "default_compose_type")]
    pub compose_type: String,
    #[serde(default)]
    pub compose_file: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: Option<String>,
    pub compose_type: Option<String>,
    pub trigger_type: Option<String>,
    pub command: Option<String>,
    pub enable_submodules: Option<i64>,
    pub compose_path: Option<String>,
    pub suffix: Option<String>,
    pub randomize: Option<i64>,
    pub isolated_deployment: Option<i64>,
    pub isolated_deployments_volume: Option<i64>,
    pub watch_paths: Option<String>,
    pub service_networks: Option<Vec<ComposeServiceNetworkDto>>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeGithubSourceDto {
    #[validate(length(min = 1, max = 255))]
    pub repository: String,
    #[validate(length(min = 1, max = 255))]
    pub owner: String,
    #[validate(length(min = 1, max = 255))]
    pub branch: String,
    pub github_provider_id: Option<i64>,
    pub auto_deploy: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeGitlabSourceDto {
    pub gitlab_project_id: Option<i64>,
    #[validate(length(min = 1, max = 255))]
    pub gitlab_repository: String,
    #[validate(length(min = 1, max = 255))]
    pub gitlab_owner: String,
    #[validate(length(min = 1, max = 255))]
    pub gitlab_branch: String,
    pub gitlab_path_namespace: Option<String>,
    pub gitlab_provider_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeGiteaSourceDto {
    #[validate(length(min = 1, max = 255))]
    pub gitea_repository: String,
    #[validate(length(min = 1, max = 255))]
    pub gitea_owner: String,
    #[validate(length(min = 1, max = 255))]
    pub gitea_branch: String,
    pub gitea_provider_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeBitbucketSourceDto {
    #[validate(length(min = 1, max = 255))]
    pub bitbucket_repository: String,
    pub bitbucket_repository_slug: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub bitbucket_owner: String,
    #[validate(length(min = 1, max = 255))]
    pub bitbucket_branch: String,
    pub bitbucket_provider_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeCustomGitSourceDto {
    #[validate(length(min = 1, max = 500))]
    pub custom_git_url: String,
    #[validate(length(min = 1, max = 255))]
    pub custom_git_branch: String,
    pub custom_git_ssh_key_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchComposeRawSourceDto {
    pub compose_file: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ComposeResponseDto {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: String,
    pub source_type: String,
    pub compose_type: String,
    pub compose_status: String,
    pub trigger_type: String,
    pub repository: Option<String>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub gitlab_repository: Option<String>,
    pub gitlab_owner: Option<String>,
    pub gitlab_branch: Option<String>,
    pub gitea_repository: Option<String>,
    pub gitea_owner: Option<String>,
    pub gitea_branch: Option<String>,
    pub bitbucket_repository: Option<String>,
    pub bitbucket_owner: Option<String>,
    pub bitbucket_branch: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub command: String,
    pub compose_path: String,
    pub service_networks: Vec<ComposeServiceNetworkDto>,
    pub services: Vec<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub fn detect_compose_service_names(record: &ComposeRecord) -> Vec<String> {
    if !record.compose_file.trim().is_empty() {
        if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(&record.compose_file) {
            if let Some(services) = value.get("services").and_then(|s| s.as_mapping()) {
                let keys: Vec<String> = services
                    .keys()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect();
                if !keys.is_empty() {
                    return keys;
                }
            }
        }
    }

    let clean_path = record.compose_path.trim_start_matches("./");
    let candidates = [
        format!(".runtime/rustploy/compose/{}/source/docker-compose.yml", record.app_name),
        format!(".runtime/rustploy/compose/{}/source/docker-compose.yaml", record.app_name),
        format!(".runtime/rustploy/compose/{}/source/compose.yml", record.app_name),
        format!(".runtime/rustploy/compose/{}/source/compose.yaml", record.app_name),
        format!(".runtime/rustploy/compose/{}/source/{}", record.app_name, clean_path),
    ];

    for path in &candidates {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                if let Some(services) = value.get("services").and_then(|s| s.as_mapping()) {
                    let keys: Vec<String> = services
                        .keys()
                        .filter_map(|k| k.as_str().map(|s| s.to_string()))
                        .collect();
                    if !keys.is_empty() {
                        return keys;
                    }
                }
            }
        }
    }

    vec!["app".to_string()]
}

impl From<ComposeRecord> for ComposeResponseDto {
    fn from(value: ComposeRecord) -> Self {
        let services = detect_compose_service_names(&value);
        Self {
            id: value.id,
            name: value.name,
            app_name: value.app_name,
            description: value.description,
            env_var: value.env_var,
            compose_file: value.compose_file,
            source_type: value.source_type,
            compose_type: value.compose_type.as_str().to_string(),
            compose_status: value.compose_status,
            trigger_type: value.trigger_type,
            repository: value.repository,
            owner: value.owner,
            branch: value.branch,
            gitlab_repository: value.gitlab_repository,
            gitlab_owner: value.gitlab_owner,
            gitlab_branch: value.gitlab_branch,
            gitea_repository: value.gitea_repository,
            gitea_owner: value.gitea_owner,
            gitea_branch: value.gitea_branch,
            bitbucket_repository: value.bitbucket_repository,
            bitbucket_owner: value.bitbucket_owner,
            bitbucket_branch: value.bitbucket_branch,
            custom_git_url: value.custom_git_url,
            custom_git_branch: value.custom_git_branch,
            command: value.command,
            compose_path: value.compose_path,
            service_networks: parse_service_networks(&value.service_networks),
            services,
            environment_id: value.environment_id,
            server_id: value.server_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub(crate) fn serialize_service_networks(
    value: Option<&Vec<ComposeServiceNetworkDto>>,
) -> sqlx::Result<Option<String>> {
    value
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))
}

pub(crate) fn parse_service_networks(value: &str) -> Vec<ComposeServiceNetworkDto> {
    serde_json::from_str::<Vec<ComposeServiceNetworkDto>>(value).unwrap_or_default()
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposeOperationResponseDto {
    pub compose: ComposeResponseDto,
    pub deployment_id: Option<i64>,
    pub operation: String,
}

impl From<ComposeOperationResult> for ComposeOperationResponseDto {
    fn from(value: ComposeOperationResult) -> Self {
        Self {
            compose: ComposeResponseDto::from(value.compose),
            deployment_id: value.deployment_id,
            operation: value.operation.as_str().into(),
        }
    }
}

fn default_source_type() -> String {
    "GITHUB".into()
}

fn default_compose_type() -> String {
    "DOCKER-COMPOSE".into()
}
