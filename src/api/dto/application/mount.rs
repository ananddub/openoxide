use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::mounts::Mount;

pub use crate::utils::builder::spec::MountType as ApplicationMountType;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertApplicationMountDto {
    pub mount_type: ApplicationMountType,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    #[validate(length(min = 1, max = 4_096))]
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationMountResponseDto {
    pub id: i64,
    pub mount_type: String,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub mount_path: String,
    pub application_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Mount> for ApplicationMountResponseDto {
    fn from(value: Mount) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            mount_type: value.mount_type,
            host_path: value.host_path,
            volume_name: value.volume_name,
            file_path: value.file_path,
            content: value.content,
            mount_path: value.mount_path,
            application_id: value.application_id.unwrap_or_default(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
