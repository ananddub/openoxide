use crate::db::models::tags::Tag;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct TagRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl TagRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Tag>, sqlx::Error> {
        sqlx::query_as!(
            Tag,
            r#"SELECT id AS "id?: i64", name AS "name: String", color AS "color: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64" FROM tags"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Tag>, sqlx::Error> {
        sqlx::query_as!(
            Tag,
            r#"SELECT id AS "id?: i64", name AS "name: String", color AS "color: String", organization_id AS "organization_id: i64", created_at AS "created_at: i64" FROM tags WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Tag) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO tags (name, color, organization_id, created_at) VALUES (?, ?, ?, ?)"#,
            &item.name,
            &item.color,
            item.organization_id,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Tag) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE tags SET name = ?, color = ?, organization_id = ?, created_at = ? WHERE id = ?"#,
            &item.name,
            &item.color,
            item.organization_id,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM tags WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
