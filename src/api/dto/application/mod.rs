use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::dto::database::parse_json_string_vec;
use crate::db::models::rollbacks::Rollback;
use crate::services::application::ApplicationRecord;

pub mod import_export;
pub mod middleware;
pub mod mount;
pub mod network;
pub mod patch;
pub mod port;
pub mod redirect;
pub mod security;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateApplicationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    #[serde(default = "default_source_type")]
    pub source_type: String,
    #[serde(default = "default_build_type")]
    pub build_type: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchApplicationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub build_type: Option<String>,
    pub trigger_type: Option<String>,
    pub env_var: Option<String>,
    pub icon: Option<String>,
    pub server_id: Option<i64>,
    pub build_server_id: Option<i64>,
    pub registry_id: Option<i64>,
    pub network_ids: Option<Vec<String>>,
    pub detach_rustploy_network: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct MoveApplicationDto {
    #[validate(range(min = 1))]
    pub target_environment_id: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchGithubSourceDto {
    #[validate(length(min = 1, max = 255))]
    pub repository: String,
    #[validate(length(min = 1, max = 255))]
    pub owner: String,
    #[validate(length(min = 1, max = 255))]
    pub branch: Option<String>,
    pub build_path: Option<String>,
    pub github_provider_id: Option<i64>,
    pub auto_deploy: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchGitlabSourceDto {
    pub gitlab_project_id: Option<i64>,
    #[validate(length(min = 1, max = 255))]
    pub gitlab_repository: String,
    #[validate(length(min = 1, max = 255))]
    pub gitlab_owner: String,
    #[validate(length(min = 1, max = 255))]
    pub gitlab_branch: Option<String>,
    pub gitlab_build_path: Option<String>,
    pub gitlab_path_namespace: Option<String>,
    pub gitlab_provider_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchGiteaSourceDto {
    #[validate(length(min = 1, max = 255))]
    pub gitea_repository: String,
    #[validate(length(min = 1, max = 255))]
    pub gitea_owner: String,
    #[validate(length(min = 1, max = 255))]
    pub gitea_branch: Option<String>,
    pub gitea_build_path: Option<String>,
    pub gitea_provider_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchBitbucketSourceDto {
    #[validate(length(min = 1, max = 255))]
    pub bitbucket_repository: String,
    pub bitbucket_repository_slug: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub bitbucket_owner: String,
    #[validate(length(min = 1, max = 255))]
    pub bitbucket_branch: Option<String>,
    pub bitbucket_build_path: Option<String>,
    pub bitbucket_provider_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchDockerSourceDto {
    #[validate(length(min = 1, max = 500))]
    pub docker_image: String,
    pub docker_username: Option<String>,
    pub docker_password: Option<String>,
    pub registry_url: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchCustomGitSourceDto {
    #[validate(length(min = 1, max = 500))]
    pub custom_git_url: String,
    #[validate(length(min = 1, max = 255))]
    pub custom_git_branch: Option<String>,
    pub custom_git_build_path: Option<String>,
    pub custom_git_ssh_key_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchDropSourceDto {
    pub drop_build_path: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchBuildConfigDto {
    pub build_args: Option<String>,
    pub build_secrets: Option<String>,
    pub dockerfile: Option<String>,
    pub docker_context_path: Option<String>,
    pub docker_build_stage: Option<String>,
    pub publish_directory: Option<String>,
    pub is_static_spa: Option<i64>,
    pub create_env_file: Option<i64>,
    pub railpack_version: Option<String>,
    pub heroku_version: Option<String>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub build_path: Option<String>,
    pub clean_cache: Option<i64>,
    pub enable_submodules: Option<i64>,
    pub watch_paths: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchResourceConfigDto {
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchPreviewConfigDto {
    pub preview_env: Option<String>,
    pub preview_build_args: Option<String>,
    pub preview_build_secrets: Option<String>,
    pub preview_labels: Option<String>,
    #[validate(length(min = 1, max = 253))]
    pub preview_wildcard: Option<String>,
    #[validate(range(min = 1, max = 65535))]
    pub preview_port: i64,
    pub preview_https: bool,
    #[validate(length(min = 1, max = 255))]
    pub preview_path: String,
    pub preview_certificate_type: crate::utils::traefik::types::CertificateType,
    pub preview_custom_cert_resolver: Option<String>,
    #[validate(range(min = 1, max = 20))]
    pub preview_limit: i64,
    pub active: bool,
    pub require_collaborator_permissions: bool,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationResponseDto {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub build_type: String,
    pub app_status: String,
    pub trigger_type: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub build_server_id: Option<i64>,
    pub registry_id: Option<i64>,
    pub network_ids: Vec<String>,
    pub detach_rustploy_network: i64,
    pub env_var: Option<String>,
    pub icon: Option<String>,
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
    pub docker_image: Option<String>,
    pub registry_url: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub preview_env: Option<String>,
    pub preview_build_args: Option<String>,
    pub preview_build_secrets: Option<String>,
    pub preview_labels: Option<String>,
    pub preview_wildcard: Option<String>,
    pub preview_port: Option<i64>,
    pub preview_https: bool,
    pub preview_path: Option<String>,
    pub preview_certificate_type: String,
    pub preview_custom_cert_resolver: Option<String>,
    pub preview_limit: Option<i64>,
    pub previews_active: bool,
    pub preview_require_collaborator_permissions: bool,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<ApplicationRecord> for ApplicationResponseDto {
    fn from(value: ApplicationRecord) -> Self {
        Self {
            id: value.id,
            name: value.name,
            app_name: value.app_name,
            description: value.description,
            source_type: value.source_type,
            build_type: value.build_type,
            app_status: value.app_status,
            trigger_type: value.trigger_type,
            environment_id: value.environment_id,
            server_id: value.server_id,
            build_server_id: value.build_server_id,
            registry_id: value.registry_id,
            network_ids: parse_json_string_vec(&value.network_ids),
            detach_rustploy_network: value.detach_rustploy_network,
            env_var: value.env_var,
            icon: value.icon,
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
            docker_image: value.docker_image,
            registry_url: value.registry_url,
            custom_git_url: value.custom_git_url,
            custom_git_branch: value.custom_git_branch,
            preview_env: value.preview_env,
            preview_build_args: value.preview_build_args,
            preview_build_secrets: value.preview_build_secrets,
            preview_labels: value.preview_labels,
            preview_wildcard: value.preview_wildcard,
            preview_port: value.preview_port,
            preview_https: value.preview_https,
            preview_path: value.preview_path,
            preview_certificate_type: value.preview_certificate_type,
            preview_custom_cert_resolver: value.preview_custom_cert_resolver,
            preview_limit: value.preview_limit,
            previews_active: value.previews_active,
            preview_require_collaborator_permissions: value
                .preview_require_collaborator_permissions,
            memory_reservation: value.memory_reservation,
            memory_limit: value.memory_limit,
            cpu_reservation: value.cpu_reservation,
            cpu_limit: value.cpu_limit,
            replicas: value.replicas,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationOperationResponseDto {
    pub application: ApplicationResponseDto,
    pub deployment_id: Option<i64>,
    pub operation: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationWebhookTokenResponseDto {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationCleanupResponseDto {
    pub affected: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationRollbackResponseDto {
    pub id: i64,
    pub deployment_id: i64,
    pub version: i64,
    pub image: Option<String>,
    pub has_context: bool,
    pub created_at: i64,
}

impl From<Rollback> for ApplicationRollbackResponseDto {
    fn from(value: Rollback) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            deployment_id: value.deployment_id,
            version: value.version,
            image: value.image,
            has_context: value.full_context.is_some(),
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationRollbackTriggerResponseDto {
    pub deployment_id: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationForceKillResponseDto {
    pub deployment_id: i64,
    pub status: String,
}

fn default_source_type() -> String {
    "GITHUB".into()
}

fn default_build_type() -> String {
    "NIXPACKS".into()
}
