use crate::db::models::project_tags::ProjectTag;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct ProjectTagRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ProjectTagRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<ProjectTag>, sqlx::Error> {
        sqlx::query_as!(
            ProjectTag,
            r#"SELECT id AS "id?: i64", project_id AS "project_id: i64", tag_id AS "tag_id: i64" FROM project_tags"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<ProjectTag>, sqlx::Error> {
        sqlx::query_as!(
            ProjectTag,
            r#"SELECT id AS "id?: i64", project_id AS "project_id: i64", tag_id AS "tag_id: i64" FROM project_tags WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &ProjectTag) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO project_tags (project_id, tag_id) VALUES (?, ?)"#,
            item.project_id,
            item.tag_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &ProjectTag) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE project_tags SET project_id = ?, tag_id = ? WHERE id = ?"#,
            item.project_id,
            item.tag_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM project_tags WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
