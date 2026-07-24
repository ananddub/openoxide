use crate::db::models::atlas_schema_revisions::AtlasSchemaRevision;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct AtlasSchemaRevisionRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl AtlasSchemaRevisionRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<AtlasSchemaRevision>, sqlx::Error> {
        sqlx::query_as!(
            AtlasSchemaRevision,
            r#"SELECT version AS "version: String", description AS "description: String", type AS "atlas_schema_revision_type: i64", applied AS "applied: i64", total AS "total: i64", executed_at AS "executed_at: chrono::NaiveDateTime", execution_time AS "execution_time: i64", error AS "error?: String", error_stmt AS "error_stmt?: String", hash AS "hash: String", partial_hashes AS "partial_hashes?: String", operator_version AS "operator_version: String" FROM atlas_schema_revisions"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(
        &self,
        version: &str,
    ) -> Result<Option<AtlasSchemaRevision>, sqlx::Error> {
        sqlx::query_as!(
            AtlasSchemaRevision,
            r#"SELECT version AS "version: String", description AS "description: String", type AS "atlas_schema_revision_type: i64", applied AS "applied: i64", total AS "total: i64", executed_at AS "executed_at: chrono::NaiveDateTime", execution_time AS "execution_time: i64", error AS "error?: String", error_stmt AS "error_stmt?: String", hash AS "hash: String", partial_hashes AS "partial_hashes?: String", operator_version AS "operator_version: String" FROM atlas_schema_revisions WHERE version = ?"#,
            version
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &AtlasSchemaRevision) -> Result<String, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO atlas_schema_revisions (version, description, type, applied, total, executed_at, execution_time, error, error_stmt, hash, partial_hashes, operator_version) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.version,
            &item.description,
            item.atlas_schema_revision_type,
            item.applied,
            item.total,
            &item.executed_at,
            item.execution_time,
            &item.error,
            &item.error_stmt,
            &item.hash,
            &item.partial_hashes,
            &item.operator_version
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(item.version.clone())
    }

    pub async fn update(
        &self,
        version: &str,
        item: &AtlasSchemaRevision,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE atlas_schema_revisions SET description = ?, type = ?, applied = ?, total = ?, executed_at = ?, execution_time = ?, error = ?, error_stmt = ?, hash = ?, partial_hashes = ?, operator_version = ? WHERE version = ?"#,
            &item.description,
            item.atlas_schema_revision_type,
            item.applied,
            item.total,
            &item.executed_at,
            item.execution_time,
            &item.error,
            &item.error_stmt,
            &item.hash,
            &item.partial_hashes,
            &item.operator_version,
            version
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, version: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM atlas_schema_revisions WHERE version = ?"#,
            version
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
