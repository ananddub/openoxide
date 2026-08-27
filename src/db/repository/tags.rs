use crate::db::models::tags::Tag;
use crate::flash_gen::{
    db::Queries,
    models::Tags,
    tags::{TagsCreateParams, TagsGetAllRow, TagsUpdateParams},
};
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct TagRepository {
    queries: Queries,
}

#[singleton]
impl TagRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            queries: Queries::new(pool.as_ref().clone()),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Tag>, sqlx::Error> {
        self.queries
            .tags_get_all()
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Tag>, sqlx::Error> {
        match self.queries.tags_get_by_id(id).await {
            Ok(row) => Ok(Some(row.into())),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn create(&self, item: &Tag) -> Result<i64, sqlx::Error> {
        self.queries
            .tags_create(&TagsCreateParams {
                name: item.name.clone(),
                color: item.color.clone(),
                organization_id: item.organization_id,
                created_at: item.created_at,
            })
            .await?;
        self.queries.tags_get_last_insert_rowid().await
    }

    pub async fn update(&self, id: i64, item: &Tag) -> Result<(), sqlx::Error> {
        self.queries
            .tags_update(&TagsUpdateParams {
                name: item.name.clone(),
                color: item.color.clone(),
                organization_id: item.organization_id,
                created_at: item.created_at,
                id,
            })
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        self.queries.tags_delete(id).await
    }
}

impl From<TagsGetAllRow> for Tag {
    fn from(row: TagsGetAllRow) -> Self {
        Self {
            id: Some(row.id),
            name: row.name,
            color: row.color,
            organization_id: row.organization_id,
            created_at: row.created_at,
        }
    }
}

impl From<Tags> for Tag {
    fn from(row: Tags) -> Self {
        Self {
            id: Some(row.id),
            name: row.name,
            color: row.color,
            organization_id: row.organization_id,
            created_at: row.created_at,
        }
    }
}
