use std::{
    path::{Component, Path},
    sync::Arc,
};

use auto_di::singleton;
use os::string_enum;
use serde::{Deserialize, Serialize};

use crate::{
    db::queries::FlashQueries,
    flash_gen::models::Patches,
    flash_gen::patches::{PatchesCreateForApplicationParams, PatchesUpdateForApplicationParams},
};

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ApplicationPatchType {
        default = Create;

        Create => "CREATE",
        Update => "UPDATE",
        Delete => "DELETE",
    }
}

#[derive(Debug, Deserialize)]
pub struct UpsertApplicationPatchDto {
    pub patch_type: ApplicationPatchType,
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApplicationPatchResponseDto {
    pub id: i64,
    pub patch_type: String,
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
    pub application_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Patches> for ApplicationPatchResponseDto {
    fn from(value: Patches) -> Self {
        Self {
            id: value.id,
            patch_type: value.patch_type,
            file_path: value.file_path,
            enabled: value.enabled != 0,
            content: value.content,
            application_id: value.service_id.unwrap_or_default(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

pub struct PatchService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl PatchService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    async fn ensure_application(&self, id: i64) -> sqlx::Result<()> {
        self.queries
            .app_sub_get_application_by_i_d(id)
            .await
            .map(|_| ())
    }

    pub async fn list(&self, application_id: i64) -> sqlx::Result<Vec<Patches>> {
        self.ensure_application(application_id).await?;
        self.queries
            .patches_list_by_application(application_id)
            .await
    }

    pub async fn create(
        &self,
        application_id: i64,
        input: UpsertApplicationPatchDto,
    ) -> sqlx::Result<Patches> {
        self.ensure_application(application_id).await?;
        let input = normalize_patch(input).map_err(sqlx::Error::Protocol)?;
        let params = PatchesCreateForApplicationParams {
            patch_type: input.patch_type.as_str().to_owned(),
            file_path: input.file_path,
            enabled: i64::from(input.enabled),
            content: input.content,
            service_id: application_id,
        };
        self.queries.patches_create_for_application(&params).await
    }

    pub async fn update(
        &self,
        application_id: i64,
        id: i64,
        input: UpsertApplicationPatchDto,
    ) -> sqlx::Result<Patches> {
        self.ensure_application(application_id).await?;
        let input = normalize_patch(input).map_err(sqlx::Error::Protocol)?;
        let params = PatchesUpdateForApplicationParams {
            patch_type: input.patch_type.as_str().to_owned(),
            file_path: input.file_path,
            enabled: i64::from(input.enabled),
            content: input.content,
            id,
            service_id: application_id,
        };
        self.queries.patches_update_for_application(&params).await
    }

    pub async fn delete(&self, application_id: i64, id: i64) -> sqlx::Result<bool> {
        self.ensure_application(application_id).await?;
        match self
            .queries
            .patches_delete_for_application(id, application_id)
            .await
        {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
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
    if !matches!(input.patch_type, ApplicationPatchType::Delete) && input.content.is_empty() {
        return Err("content is required for CREATE and UPDATE patches".into());
    }
    if matches!(input.patch_type, ApplicationPatchType::Delete) {
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
            patch_type: ApplicationPatchType::Update,
            file_path: "../secret".into(),
            enabled: true,
            content: "x".into(),
        };
        assert!(normalize_patch(input).is_err());
    }
}
