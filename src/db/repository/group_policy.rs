use crate::db::models::group_policy::GroupPolicy;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GroupPolicyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl GroupPolicyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<GroupPolicy>, sqlx::Error> {
        sqlx::query_as!(
            GroupPolicy,
            r#"SELECT id AS "id?: i64", group_id AS "group_id: i64", policy_id AS "policy_id: i64", created_at AS "created_at: i64" FROM group_policy"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<GroupPolicy>, sqlx::Error> {
        sqlx::query_as!(
            GroupPolicy,
            r#"SELECT id AS "id?: i64", group_id AS "group_id: i64", policy_id AS "policy_id: i64", created_at AS "created_at: i64" FROM group_policy WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &GroupPolicy) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO group_policy (group_id, policy_id, created_at) VALUES (?, ?, ?)"#,
            item.group_id,
            item.policy_id,
            item.created_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &GroupPolicy) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE group_policy SET group_id = ?, policy_id = ?, created_at = ? WHERE id = ?"#,
            item.group_id,
            item.policy_id,
            item.created_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM group_policy WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
