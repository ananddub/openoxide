use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::db::models::user_policy::UserPolicy;

pub struct UserPolicyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl UserPolicyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn list_by_user_and_org(
        &self,
        user_id: i64,
        org_id: i64,
    ) -> Result<Vec<UserPolicy>, sqlx::Error> {
        sqlx::query_as::<_, UserPolicy>(
            "SELECT * FROM user_policy WHERE user_id = ? AND org_id = ?",
        )
        .bind(user_id)
        .bind(org_id)
        .fetch_all(&*self.pool)
        .await
    }

    pub async fn upsert(
        &self,
        user_id: i64,
        org_id: i64,
        policy_id: i64,
        effect: &str,
    ) -> Result<UserPolicy, sqlx::Error> {
        sqlx::query_as::<_, UserPolicy>(
            r#"
            INSERT INTO user_policy (user_id, org_id, policy_id, effect)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(user_id, org_id, policy_id) DO UPDATE SET effect = excluded.effect
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(org_id)
        .bind(policy_id)
        .bind(effect)
        .fetch_one(&*self.pool)
        .await
    }

    pub async fn delete(
        &self,
        user_id: i64,
        org_id: i64,
        policy_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_policy WHERE user_id = ? AND org_id = ? AND policy_id = ?")
            .bind(user_id)
            .bind(org_id)
            .bind(policy_id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}
