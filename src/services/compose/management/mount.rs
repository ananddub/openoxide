use std::{path::Path, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::compose::management::UpsertComposeMountDto,
    db::models::mounts::Mount,
    repository::{ComposeProjectRepository, MountRepository},
};

pub struct ComposeMountService {
    projects: Arc<ComposeProjectRepository>,
    mounts: Arc<MountRepository>,
}

#[singleton]
impl ComposeMountService {
    fn new(projects: Arc<ComposeProjectRepository>, mounts: Arc<MountRepository>) -> Self {
        Self { projects, mounts }
    }

    async fn ensure_compose(&self, id: i64) -> sqlx::Result<()> {
        self.projects
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }

    pub async fn list(&self, compose_id: i64) -> sqlx::Result<Vec<Mount>> {
        self.ensure_compose(compose_id).await?;
        self.mounts.fetch_for_compose(compose_id).await
    }

    pub async fn create(
        &self,
        compose_id: i64,
        input: UpsertComposeMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_compose(compose_id).await?;
        let input = normalize(input)?;
        self.mounts
            .create_for_compose(
                compose_id,
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
        compose_id: i64,
        id: i64,
        input: UpsertComposeMountDto,
    ) -> sqlx::Result<Mount> {
        self.ensure_compose(compose_id).await?;
        let input = normalize(input)?;
        self.mounts
            .update_for_compose(
                id,
                compose_id,
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

    pub async fn delete(&self, compose_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_compose(compose_id).await?;
        self.mounts.delete_for_compose(id, compose_id).await
    }
}

fn normalize(mut input: UpsertComposeMountDto) -> sqlx::Result<UpsertComposeMountDto> {
    input.mount_path = input.mount_path.trim().to_string();
    if !Path::new(&input.mount_path).is_absolute() {
        return Err(sqlx::Error::Protocol("mount_path must be absolute".into()));
    }
    match input.mount_type {
        crate::api::dto::application::mount::ApplicationMountType::Bind => {
            let value = input
                .host_path
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| sqlx::Error::Protocol("host_path is required for BIND".into()))?;
            if !Path::new(value).is_absolute() {
                return Err(sqlx::Error::Protocol("host_path must be absolute".into()));
            }
            input.host_path = Some(value.into());
            input.volume_name = None;
            input.file_path = None;
            input.content = None;
        }
        crate::api::dto::application::mount::ApplicationMountType::Volume => {
            let value = input
                .volume_name
                .as_deref()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .ok_or_else(|| {
                    sqlx::Error::Protocol("volume_name is required for VOLUME".into())
                })?;
            input.volume_name = Some(value.into());
            input.host_path = None;
            input.file_path = None;
            input.content = None;
        }
        crate::api::dto::application::mount::ApplicationMountType::File => {
            if input.content.is_none() {
                return Err(sqlx::Error::Protocol("content is required for FILE".into()));
            }
            input.host_path = None;
            input.volume_name = None;
        }
    }
    Ok(input)
}
