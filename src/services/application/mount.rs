use std::{path::Path, sync::Arc};

use auto_di::singleton;
use serde::{Deserialize, Serialize};

use crate::{
    db::models::mounts::Mount,
    db::queries::FlashQueries,
    flash_gen::models::Mounts,
    flash_gen::mount::{MountCreateForApplicationParams, MountUpdateForApplicationParams},
};

#[derive(Debug, Deserialize)]
pub struct UpsertApplicationMountDto {
    pub mount_type: crate::utils::builder::spec::MountType,
    pub host_path: Option<String>,
    pub volume_name: Option<String>,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize)]
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
            id: value.id,
            mount_type: value.mount_type,
            host_path: value.host_path,
            volume_name: value.volume_name,
            file_path: value.file_path,
            content: value.content,
            mount_path: value.mount_path,
            application_id: value.service_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

/// Flash bind parameters are non-optional for nullable columns, so `None` is
/// written as `""`; normalize empty strings back to `None` on read to preserve
/// the previous repository semantics.
fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.is_empty())
}

fn to_model(row: Mounts) -> Mount {
    Mount {
        id: row.id,
        mount_type: row.mount_type,
        service_type: row.service_type,
        host_path: non_empty(row.host_path),
        volume_name: non_empty(row.volume_name),
        file_path: non_empty(row.file_path),
        content: non_empty(row.content),
        mount_path: row.mount_path,
        service_id: row.service_id,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

pub struct MountService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl MountService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    async fn ensure_application(&self, id: i64) -> sqlx::Result<()> {
        self.queries
            .app_sub_get_application_by_i_d(id)
            .await
            .map(|_| ())
    }

    pub async fn list(&self, application_id: i64) -> sqlx::Result<Vec<Mount>> {
        self.ensure_application(application_id).await?;
        Ok(self
            .queries
            .mount_fetch_for_application(application_id)
            .await?
            .into_iter()
            .map(to_model)
            .collect())
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_application(application_id).await?;
        let input = normalize_mount(input).map_err(sqlx::Error::Protocol)?;
        let params = MountCreateForApplicationParams {
            mount_type: input.mount_type.as_str().to_owned(),
            host_path: input.host_path.unwrap_or_default(),
            volume_name: input.volume_name.unwrap_or_default(),
            file_path: input.file_path.unwrap_or_default(),
            content: input.content.unwrap_or_default(),
            mount_path: input.mount_path,
            service_id: application_id,
        };
        Ok(to_model(
            self.queries.mount_create_for_application(&params).await?,
        ))
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_application(application_id).await?;
        let input = normalize_mount(input).map_err(sqlx::Error::Protocol)?;
        let params = MountUpdateForApplicationParams {
            mount_type: input.mount_type.as_str().to_owned(),
            host_path: input.host_path.unwrap_or_default(),
            volume_name: input.volume_name.unwrap_or_default(),
            file_path: input.file_path.unwrap_or_default(),
            content: input.content.unwrap_or_default(),
            mount_path: input.mount_path,
            id,
            service_id: application_id,
        };
        Ok(to_model(
            self.queries.mount_update_for_application(&params).await?,
        ))
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        match self
            .queries
            .mount_delete_for_application(id, application_id)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
    }
}

fn normalize_mount(
    mut input: UpsertApplicationMountDto,
) -> Result<UpsertApplicationMountDto, String> {
    input.mount_path = input.mount_path.trim().to_string();
    if !Path::new(&input.mount_path).is_absolute() {
        return Err("mount_path must be an absolute container path".into());
    }

    match input.mount_type {
        crate::utils::builder::spec::MountType::Bind => {
            let host_path = input
                .host_path
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| "host_path is required for BIND mounts".to_string())?;
            if !Path::new(host_path).is_absolute() {
                return Err("host_path must be absolute for BIND mounts".into());
            }
            input.host_path = Some(host_path.to_string());
            input.volume_name = None;
            input.file_path = None;
            input.content = None;
        }
        crate::utils::builder::spec::MountType::Volume => {
            let name = input
                .volume_name
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| "volume_name is required for VOLUME mounts".to_string())?;
            input.volume_name = Some(name.to_string());
            input.host_path = None;
            input.file_path = None;
            input.content = None;
        }
        crate::utils::builder::spec::MountType::File => {
            if input.content.is_none() {
                return Err("content is required for FILE mounts".into());
            }
            input.host_path = None;
            input.volume_name = None;
        }
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_relative_bind_paths() {
        let input = UpsertApplicationMountDto {
            mount_type: crate::utils::builder::spec::MountType::Bind,
            host_path: Some("data".into()),
            volume_name: None,
            file_path: None,
            content: None,
            mount_path: "/app/data".into(),
        };
        assert!(normalize_mount(input).is_err());
    }
}
