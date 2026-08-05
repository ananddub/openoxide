use std::{
    path::{Component, Path},
    sync::Arc,
};

use auto_di::singleton;

use crate::{
    api::dto::application::patch::UpsertApplicationPatchDto,
    db::models::patches::Patch,
    repository::{ApplicationRepository, PatchRepository},
};

pub struct PatchService {
    applications: Arc<ApplicationRepository>,
    patches: Arc<PatchRepository>,
}

#[singleton]
impl PatchService {
    fn new(applications: Arc<ApplicationRepository>, patches: Arc<PatchRepository>) -> Self {
        Self {
            applications,
            patches,
        }
    }

    async fn ensure_application(&self, id: i64) -> sqlx::Result<()> {
        self.applications
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
            .map(|_| ())
    }

    pub async fn list(&self, application_id: i64) -> sqlx::Result<Vec<Patch>> {
        self.ensure_application(application_id).await?;
        self.patches.list_by_application(application_id).await
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationPatchDto,
    ) -> sqlx::Result<Patch> {
        self.ensure_application(application_id).await?;
        let input = normalize_patch(input).map_err(sqlx::Error::Protocol)?;
        self.patches
            .create_for_application(
                application_id,
                input.patch_type.as_str(),
                &input.file_path,
                i64::from(input.enabled),
                &input.content,
            )
            .await
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationPatchDto,
    ) -> sqlx::Result<Patch> {
        self.ensure_application(application_id).await?;
        let input = normalize_patch(input).map_err(sqlx::Error::Protocol)?;
        self.patches
            .update_for_application(
                id,
                application_id,
                input.patch_type.as_str(),
                &input.file_path,
                i64::from(input.enabled),
                &input.content,
            )
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        self.patches
            .delete_for_application(id, application_id)
            .await
    }
}

fn normalize_patch(
    mut input: UpsertApplicationPatchDto,
) -> Result<UpsertApplicationPatchDto, String> {
    input.file_path = input.file_path.trim().replace('\\', "/");
    let path = Path::new(&input.file_path);
    if path.is_absolute()
        || path.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err("file_path must be a safe relative repository path".into());
    }
    if !matches!(
        input.patch_type,
        crate::api::dto::application::patch::ApplicationPatchType::Delete
    ) && input.content.is_empty()
    {
        return Err("content is required for CREATE and UPDATE patches".into());
    }
    if matches!(
        input.patch_type,
        crate::api::dto::application::patch::ApplicationPatchType::Delete
    ) {
        input.content.clear();
    } else if !input.content.ends_with('\n') {
        input.content.push('\n');
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_parent_directory_traversal() {
        let input = UpsertApplicationPatchDto {
            patch_type: crate::api::dto::application::patch::ApplicationPatchType::Update,
            file_path: "../secret".into(),
            enabled: true,
            content: "x".into(),
        };
        assert!(normalize_patch(input).is_err());
    }
}
