use std::{path::Component, sync::Arc};

use auto_di::singleton;

use crate::{
    api::dto::compose::management::UpsertComposePatchDto, db::models::patches::Patch,
    repository::PatchRepository, services::compose::ComposeService,
};

pub struct ComposePatchService {
    compose: Arc<ComposeService>,
    patches: Arc<PatchRepository>,
}

#[singleton]
impl ComposePatchService {
    fn new(compose: Arc<ComposeService>, patches: Arc<PatchRepository>) -> Self {
        Self { compose, patches }
    }

    pub async fn list(&self, compose_id: i64) -> sqlx::Result<Vec<Patch>> {
        self.compose.get_by_id(compose_id).await?;
        self.patches.list_by_compose(compose_id).await
    }

    pub async fn create(
        &self,
        compose_id: i64,
        input: UpsertComposePatchDto,
    ) -> sqlx::Result<Patch> {
        self.compose.get_by_id(compose_id).await?;
        validate_patch(&input)?;
        self.patches
            .create_for_compose(
                compose_id,
                input.patch_type.as_str(),
                &input.file_path,
                i64::from(input.enabled),
                &input.content,
            )
            .await
    }

    pub async fn update(
        &self,
        compose_id: i64,
        id: i64,
        input: UpsertComposePatchDto,
    ) -> sqlx::Result<Patch> {
        self.compose.get_by_id(compose_id).await?;
        validate_patch(&input)?;
        self.patches
            .update_for_compose(
                id,
                compose_id,
                input.patch_type.as_str(),
                &input.file_path,
                i64::from(input.enabled),
                &input.content,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, compose_id: i64, id: i64) -> sqlx::Result<bool> {
        self.compose.get_by_id(compose_id).await?;
        self.patches.delete_for_compose(id, compose_id).await
    }
}

fn validate_patch(input: &UpsertComposePatchDto) -> sqlx::Result<()> {
    let path = std::path::Path::new(&input.file_path);
    if path.is_absolute()
        || path.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(sqlx::Error::Protocol("unsafe patch file path".into()));
    }
    Ok(())
}
