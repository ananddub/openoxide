use crate::db::models::groups::Group;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GroupRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl GroupRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Group>, sqlx::Error> {
        sqlx::query_as!(
            Group,
            r#"SELECT id AS "id?: i64", name AS "name: String", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM groups"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Group>, sqlx::Error> {
        sqlx::query_as!(
            Group,
            r#"SELECT id AS "id?: i64", name AS "name: String", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM groups WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Group) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO groups (name, created_at, updated_at) VALUES (?, ?, ?)"#,
            &item.name,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Group) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE groups SET name = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM groups WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn create_owner_group_if_not_exists(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query!("INSERT OR IGNORE INTO groups (name) VALUES ('owner')")
            .execute(&mut **tx)
            .await?;
        let id = sqlx::query_scalar!("SELECT id FROM groups WHERE name = 'owner'")
            .fetch_one(&mut **tx)
            .await?;
        Ok(id)
    }

    pub async fn get_user_final_permissions(
        &self,
        user_id: i64,
        org_id: i64,
    ) -> Result<Vec<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>(
            r#"
            WITH user_permissions AS (
                SELECT p.action, up.effect
                FROM user_policy up
                JOIN policy p ON p.id = up.policy_id
                WHERE up.user_id = ? AND up.org_id = ?
            )
            SELECT DISTINCT action FROM (
                SELECT p.action
                FROM group_policy gp
                JOIN policy p ON p.id = gp.policy_id
                JOIN organization_members om ON om.group_id = gp.group_id
                WHERE om.user_id = ? AND om.organization_id = ?
                UNION ALL
                SELECT action FROM user_permissions WHERE effect = 'GRANT'
            ) perms
            WHERE action NOT IN (SELECT action FROM user_permissions WHERE effect = 'DENY')
            "#,
        )
        .bind(user_id)
        .bind(org_id)
        .bind(user_id)
        .bind(org_id)
        .fetch_all(self.pool.as_ref())
        .await
    }
}
