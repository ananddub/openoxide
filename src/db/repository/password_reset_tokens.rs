use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct PasswordResetTokenRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PasswordResetTokenRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn issue(
        &self,
        user_id: i64,
        token_hash: &str,
        expires_at: i64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            "UPDATE password_reset_tokens SET used_at = strftime('%s', 'now') WHERE user_id = ? AND used_at IS NULL",
            user_id
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            "INSERT INTO password_reset_tokens (user_id, token_hash, expires_at) VALUES (?, ?, ?)",
            user_id,
            token_hash,
            expires_at
        )
        .execute(&mut *tx)
        .await?;
        tx.commit().await
    }

    pub async fn complete(
        &self,
        token_hash: &str,
        password_hash: &str,
    ) -> Result<bool, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let user_id = sqlx::query_scalar!(
            r#"UPDATE password_reset_tokens
               SET used_at = strftime('%s', 'now')
               WHERE token_hash = ? AND used_at IS NULL
                 AND expires_at > strftime('%s', 'now')
               RETURNING user_id AS "user_id!: i64""#,
            token_hash
        )
        .fetch_optional(&mut *tx)
        .await?;
        let Some(user_id) = user_id else {
            tx.rollback().await?;
            return Ok(false);
        };
        sqlx::query!(
            "UPDATE users SET password = ? WHERE id = ?",
            password_hash,
            user_id
        )
        .execute(&mut *tx)
        .await?;
        sqlx::query!(
            "UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = strftime('%s', 'now') WHERE user_id = ? AND is_blacklist = 0",
            user_id
        )
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(true)
    }
}
