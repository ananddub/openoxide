use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::dto::application::mount::ApplicationMountType;
use crate::db::models::{
    compose_projects::ComposeProject, domains::Domain, mounts::Mount, patches::Patch,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComposeResourceKind {
    Config,
    Secret,
}
impl ComposeResourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Config => "CONFIG",
            Self::Secret => "SECRET",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComposePatchType {
    Update,
    Delete,
}
impl ComposePatchType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Update => "UPDATE",
            Self::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposeArchiveDto {
    pub format: String,
    pub schema_version: i64,
    pub archive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeExportBundleDto {
    pub schema_version: u32,
    pub exported_at: i64,
    pub secrets_included: bool,
    pub compose: ComposeProject,
    pub domains: Vec<Domain>,
    pub mounts: Vec<Mount>,
    pub patches: Vec<Patch>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct ComposeExportQueryDto {
    #[serde(default)]
    pub include_secrets: bool,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct ImportComposeDto {
    #[validate(range(min = 1))]
    pub target_environment_id: i64,
    pub target_server_id: Option<i64>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 2))]
    pub archive: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct InstallComposeTemplateDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    #[validate(length(min = 1))]
    pub compose_file: String,
    pub env_var: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct ComposePreviewDto {
    #[validate(length(min = 1))]
    pub compose_file: String,
    pub randomize: bool,
    pub suffix: Option<String>,
    pub isolated_deployment: bool,
    pub isolated_deployments_volume: bool,
    pub app_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposePreviewResponseDto {
    pub compose_file: String,
    pub services: Vec<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct RemoveComposeServiceDto {
    #[validate(length(min = 1, max = 255))]
    pub service_name: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct MoveComposeDto {
    #[validate(range(min = 1))]
    pub target_environment_id: i64,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposeTokenDto {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposeCleanupDto {
    pub affected: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertComposeResourceDto {
    pub kind: ComposeResourceKind,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub file: Option<String>,
    pub external: bool,
    pub services: Vec<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct DeleteComposeResourceDto {
    pub kind: ComposeResourceKind,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertComposeMountDto {
    pub mount_type: ApplicationMountType,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    #[validate(length(min = 1, max = 4_096))]
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposeMountResponseDto {
    pub id: i64,
    pub mount_type: String,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub mount_path: String,
    pub compose_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertComposePatchDto {
    pub patch_type: ComposePatchType,
    #[validate(length(min = 1, max = 4_096))]
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ComposePatchResponseDto {
    pub id: i64,
    pub patch_type: String,
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
    pub compose_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Mount> for ComposeMountResponseDto {
    fn from(value: Mount) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            mount_type: value.mount_type,
            host_path: value.host_path,
            volume_name: value.volume_name,
            file_path: value.file_path,
            content: value.content,
            mount_path: value.mount_path,
            compose_id: value.compose_id.unwrap_or_default(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Patch> for ComposePatchResponseDto {
    fn from(value: Patch) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            patch_type: value.patch_type,
            file_path: value.file_path,
            enabled: value.enabled != 0,
            content: value.content,
            compose_id: value.compose_id.unwrap_or_default(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
