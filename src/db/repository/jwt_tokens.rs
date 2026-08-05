use crate::db::models::jwt_tokens::JwtToken;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct JwtTokenRepository {
    pool: Arc<SqlitePool>,
}

#[derive(Debug, Clone)]
pub struct AuthSessionRow {
    pub session_id: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub active: bool,
}

#[singleton]
impl JwtTokenRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<JwtToken>, sqlx::Error> {
        sqlx::query_as!(
            JwtToken,
            r#"SELECT id AS "id?: i64", jti AS "jti: String", role AS "role: String", user_id AS "user_id: i64", is_blacklist AS "is_blacklist?: i64", blacklist_at AS "blacklist_at?: i64", expired_at AS "expired_at?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM jwt_tokens"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<JwtToken>, sqlx::Error> {
        sqlx::query_as!(
            JwtToken,
            r#"SELECT id AS "id?: i64", jti AS "jti: String", role AS "role: String", user_id AS "user_id: i64", is_blacklist AS "is_blacklist?: i64", blacklist_at AS "blacklist_at?: i64", expired_at AS "expired_at?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM jwt_tokens WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &JwtToken) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO jwt_tokens (jti, role, user_id, is_blacklist, blacklist_at, expired_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.jti,
            &item.role,
            item.user_id,
            item.is_blacklist,
            item.blacklist_at,
            item.expired_at,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &JwtToken) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE jwt_tokens SET jti = ?, role = ?, user_id = ?, is_blacklist = ?, blacklist_at = ?, expired_at = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.jti,
            &item.role,
            item.user_id,
            item.is_blacklist,
            item.blacklist_at,
            item.expired_at,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM jwt_tokens WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn blacklist_by_jti(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        jti: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = strftime('%s', 'now') WHERE jti = ?",
            jti
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn blacklist_all_by_user(&self, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = strftime('%s', 'now') WHERE user_id = ? AND is_blacklist = 0",
            user_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn insert_token(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        jti: String,
        role: String,
        user_id: i64,
        expired_at: i64,
        session_id: &str,
        token_kind: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO jwt_tokens (jti, role, user_id, expired_at, session_id, token_kind) VALUES (?, ?, ?, ?, ?, ?)",
            jti,
            role,
            user_id,
            expired_at,
            session_id,
            token_kind
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn session_id_for_jti(&self, jti: &str) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar!("SELECT session_id FROM jwt_tokens WHERE jti = ?", jti)
            .fetch_optional(self.pool.as_ref())
            .await
            .map(Option::flatten)
    }

    pub async fn list_sessions(&self, user_id: i64) -> Result<Vec<AuthSessionRow>, sqlx::Error> {
        sqlx::query_as!(
            AuthSessionRow,
            r#"SELECT session_id AS "session_id!: String",
                      MIN(created_at) AS "created_at!: i64",
                      MAX(expired_at) AS expires_at,
                      MAX(CASE WHEN is_blacklist = 0 AND expired_at > strftime('%s', 'now') THEN 1 ELSE 0 END) != 0 AS "active!: bool"
               FROM jwt_tokens
               WHERE user_id = ? AND session_id IS NOT NULL
               GROUP BY session_id
               ORDER BY created_at DESC
               LIMIT 100"#,
            user_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn revoke_session(
        &self,
        user_id: i64,
        session_id: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = strftime('%s', 'now') WHERE user_id = ? AND session_id = ? AND is_blacklist = 0",
            user_id,
            session_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn is_token_active(&self, jti: &str) -> Result<bool, sqlx::Error> {
        let active = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM jwt_tokens WHERE jti = ? AND is_blacklist = 0 AND expired_at > strftime('%s', 'now'))",
            jti
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(active == 1)
    }
}
