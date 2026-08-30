use std::{path::Component, sync::Arc};

use auto_di::singleton;
use serde::Deserialize;

use crate::{
    db::queries::FlashQueries,
    flash_gen::compose_extra::{ComposePatchCreateParams, ComposePatchUpdateParams},
    flash_gen::models::Patches,
    services::compute::compose::ComposeService,
};

pub struct ComposePatchService {
    compose: Arc<ComposeService>,
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ComposePatchService {
    fn new(compose: Arc<ComposeService>, queries: Arc<FlashQueries>) -> Self {
        Self { compose, queries }
    }

    pub async fn list(&self, compose_id: i64) -> sqlx::Result<Vec<Patches>> {
        self.compose.get_by_id(compose_id).await?;
        self.queries.patches_list_by_compose(compose_id).await
    }

    pub async fn create(
        &self,
        compose_id: i64,
        input: UpsertComposePatchDto,
    ) -> sqlx::Result<Patches> {
        self.compose.get_by_id(compose_id).await?;
        validate_patch(&input)?;
        self.queries
            .compose_patch_create(&ComposePatchCreateParams {
                patch_type: input.patch_type.as_str().to_string(),
                file_path: input.file_path,
                enabled: i64::from(input.enabled),
                content: input.content,
                service_id: compose_id,
            })
            .await
    }

    pub async fn update(
        &self,
        compose_id: i64,
        id: i64,
        input: UpsertComposePatchDto,
    ) -> sqlx::Result<Patches> {
        self.compose.get_by_id(compose_id).await?;
        validate_patch(&input)?;
        // Old update_for_compose returned Option; :one RETURNING surfaces a
        // miss as RowNotFound, matching the old `.ok_or(RowNotFound)`.
        self.queries
            .compose_patch_update(&ComposePatchUpdateParams {
                patch_type: input.patch_type.as_str().to_string(),
                file_path: input.file_path,
                enabled: i64::from(input.enabled),
                content: input.content,
                id,
                service_id: compose_id,
            })
            .await
    }

    pub async fn delete(&self, compose_id: i64, id: i64) -> sqlx::Result<bool> {
        self.compose.get_by_id(compose_id).await?;
        // Old delete_for_compose returned rows_affected == 1.
        Ok(self
            .queries
            .compose_patch_delete(id, compose_id)
            .await
            .is_ok())
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

// ── JSON payload contract (moved from the retired api::dto layer) ──────

use os::string_enum;
use serde::Serialize;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum ComposePatchType {
        default = Update;

        Update => "UPDATE",
        Delete => "DELETE",
    }
}

#[derive(Debug, Deserialize)]
pub struct UpsertComposePatchDto {
    pub patch_type: ComposePatchType,
    pub file_path: String,
    pub enabled: bool,
    pub content: String,
}
