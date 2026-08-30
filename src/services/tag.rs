use auto_di::singleton;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    db::{models::tags::Tag, queries::FlashQueries},
    flash_gen::{
        models::Tags,
        tags::{TagsCreateParams, TagsGetAllRow, TagsUpdateParams},
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,
    pub color: String,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagDto {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone)]
pub struct TagService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl TagService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn list_all(&self) -> sqlx::Result<Vec<Tag>> {
        self.queries
            .tags_get_all()
            .await
            .map(|rows| rows.into_iter().map(tag_from_row).collect())
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Option<Tag>> {
        match self.queries.tags_get_by_id(id).await {
            Ok(row) => Ok(Some(tag_from_model(row))),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn create(&self, dto: CreateTagDto) -> sqlx::Result<Tag> {
        let params = TagsCreateParams {
            name: dto.name,
            color: dto.color,
            organization_id: dto.organization_id.unwrap_or(1),
            created_at: chrono::Utc::now().timestamp(),
        };
        self.queries.tags_create(&params).await?;
        let id = self.queries.tags_get_last_insert_rowid().await?;
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update(&self, id: i64, dto: UpdateTagDto) -> sqlx::Result<Tag> {
        let existing = self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)?;
        let updated = Tag {
            id,
            name: dto.name.unwrap_or(existing.name),
            color: dto.color.unwrap_or(existing.color),
            organization_id: existing.organization_id,
            created_at: existing.created_at,
        };
        self.queries
            .tags_update(&TagsUpdateParams {
                name: updated.name.clone(),
                color: updated.color.clone(),
                organization_id: updated.organization_id,
                created_at: updated.created_at,
                id,
            })
            .await?;
        Ok(updated)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.queries.tags_delete(id).await
    }

    pub async fn list_project_tags(&self, project_id: i64) -> sqlx::Result<Vec<Tag>> {
        self.queries
            .list_tags_by_project(project_id)
            .await
            .map(|rows| rows.into_iter().map(tag_from_model).collect())
    }

    pub async fn attach_project_tag(&self, project_id: i64, tag_id: i64) -> sqlx::Result<()> {
        self.queries.tags_add_to_project(project_id, tag_id).await
    }

    pub async fn detach_project_tag(&self, project_id: i64, tag_id: i64) -> sqlx::Result<()> {
        self.queries
            .tags_remove_from_project(project_id, tag_id)
            .await
    }
}

fn tag_from_row(row: TagsGetAllRow) -> Tag {
    Tag {
        id: row.id,
        name: row.name,
        color: row.color,
        organization_id: row.organization_id,
        created_at: row.created_at,
    }
}

fn tag_from_model(row: Tags) -> Tag {
    Tag {
        id: row.id,
        name: row.name,
        color: row.color,
        organization_id: row.organization_id,
        created_at: row.created_at,
    }
}
