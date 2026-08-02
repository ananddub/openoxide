use crate::db::models::ssh_keys::SshKey;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct SshKeyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl SshKeyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<SshKey>, sqlx::Error> {
        sqlx::query_as!(
            SshKey,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", private_key AS "private_key: String", public_key AS "public_key: String", last_used_at AS "last_used_at?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM ssh_keys"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<SshKey>, sqlx::Error> {
        sqlx::query_as!(
            SshKey,
            r#"SELECT id AS "id?: i64", name AS "name: String", description AS "description?: String", private_key AS "private_key: String", public_key AS "public_key: String", last_used_at AS "last_used_at?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM ssh_keys WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &SshKey) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO ssh_keys (name, description, private_key, public_key, last_used_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.description,
            &item.private_key,
            &item.public_key,
            item.last_used_at,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &SshKey) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?, last_used_at = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.description,
            &item.private_key,
            &item.public_key,
            item.last_used_at,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn is_in_use(&self, id: i64) -> Result<bool, sqlx::Error> {
        let count = sqlx::query_scalar!(
            r#"SELECT 
                (SELECT COUNT(*) FROM servers WHERE ssh_key_id = ?) +
                (SELECT COUNT(*) FROM applications WHERE custom_git_ssh_key_id = ?) +
                (SELECT COUNT(*) FROM compose_projects WHERE custom_git_ssh_key_id = ?) AS "total!: i64""#,
            id,
            id,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        Ok(count > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM ssh_keys WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn touch_ssh_key(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE ssh_keys SET last_used_at = strftime('%s', 'now') WHERE id = ?",
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn list_ordered(&self) -> Result<Vec<SshKey>, sqlx::Error> {
        sqlx::query_as!(
            SshKey,
            r#"SELECT id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at
               FROM ssh_keys ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_and_return(
        &self,
        name: String,
        description: Option<String>,
        private_key: String,
        public_key: String,
    ) -> Result<SshKey, sqlx::Error> {
        sqlx::query_as!(
            SshKey,
            r#"INSERT INTO ssh_keys (name, description, private_key, public_key)
               VALUES (?, ?, ?, ?)
               RETURNING id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at"#,
            name,
            description,
            private_key,
            public_key
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn update_and_return(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        private_key: String,
        public_key: String,
    ) -> Result<SshKey, sqlx::Error> {
        sqlx::query_as!(
            SshKey,
            r#"UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at"#,
            name,
            description,
            private_key,
            public_key,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn touch_and_return(&self, id: i64) -> Result<SshKey, sqlx::Error> {
        sqlx::query_as!(
            SshKey,
            r#"UPDATE ssh_keys SET last_used_at = strftime('%s', 'now')
               WHERE id = ?
               RETURNING id AS "id?", name, description, private_key, public_key, last_used_at, created_at, updated_at"#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }
}
