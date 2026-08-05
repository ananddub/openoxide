use std::{path::Path, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::application::mount::UpsertApplicationMountDto,
    db::models::mounts::Mount,
    repository::{ApplicationRepository, MountRepository},
};

pub struct MountService {
    applications: Arc<ApplicationRepository>,
    mounts: Arc<MountRepository>,
}

#[singleton]
impl MountService {
    fn new(applications: Arc<ApplicationRepository>, mounts: Arc<MountRepository>) -> Self {
        Self {
            applications,
            mounts,
        }
    }

    async fn ensure_application(&self, id: i64) -> sqlx::Result<()> {
        self.applications
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }

    pub async fn list(&self, application_id: i64) -> sqlx::Result<Vec<Mount>> {
        self.ensure_application(application_id).await?;
        self.mounts.fetch_for_application(application_id).await
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_application(application_id).await?;
        let input = normalize_mount(input).map_err(sqlx::Error::Protocol)?;
        self.mounts
            .create_for_application(
                application_id,
                input.mount_type.as_str(),
                input.host_path.as_deref(),
                input.volume_name.as_deref(),
                input.file_path.as_deref(),
                input.content.as_deref(),
                &input.mount_path,
            )
            .await
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_application(application_id).await?;
        let input = normalize_mount(input).map_err(sqlx::Error::Protocol)?;
        self.mounts
            .update_for_application(
                id,
                application_id,
                input.mount_type.as_str(),
                input.host_path.as_deref(),
                input.volume_name.as_deref(),
                input.file_path.as_deref(),
                input.content.as_deref(),
                &input.mount_path,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        self.mounts.delete_for_application(id, application_id).await
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
        crate::api::dto::application::mount::ApplicationMountType::Bind => {
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
        crate::api::dto::application::mount::ApplicationMountType::Volume => {
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
        crate::api::dto::application::mount::ApplicationMountType::File => {
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
            mount_type: crate::api::dto::application::mount::ApplicationMountType::Bind,
            host_path: Some("data".into()),
            volume_name: None,
            file_path: None,
            content: None,
            mount_path: "/app/data".into(),
        };
        assert!(normalize_mount(input).is_err());
    }
}
