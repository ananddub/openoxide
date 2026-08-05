use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PersonalAccessTokenRow {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub token_prefix: String,
    pub expires_at: Option<i64>,
    pub last_used_at: Option<i64>,
    pub revoked_at: Option<i64>,
    pub created_at: i64,
}

pub struct PersonalAccessTokenRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PersonalAccessTokenRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: i64,
        name: &str,
        token_prefix: &str,
        token_hash: &str,
        expires_at: Option<i64>,
    ) -> Result<PersonalAccessTokenRow, sqlx::Error> {
        sqlx::query_as!(
            PersonalAccessTokenRow,
            r#"INSERT INTO personal_access_tokens (user_id, name, token_prefix, token_hash, expires_at)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id AS "id!: i64", user_id AS "user_id!: i64", name AS "name!: String",
                         token_prefix AS "token_prefix!: String", expires_at, last_used_at,
                         revoked_at, created_at AS "created_at!: i64""#,
            user_id,
            name,
            token_prefix,
            token_hash,
            expires_at
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn list(&self, user_id: i64) -> Result<Vec<PersonalAccessTokenRow>, sqlx::Error> {
        sqlx::query_as!(
            PersonalAccessTokenRow,
            r#"SELECT id AS "id!: i64", user_id AS "user_id!: i64", name AS "name!: String",
                      token_prefix AS "token_prefix!: String", expires_at, last_used_at,
                      revoked_at, created_at AS "created_at!: i64"
               FROM personal_access_tokens WHERE user_id = ? ORDER BY created_at DESC"#,
            user_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn authenticate(
        &self,
        token_hash: &str,
    ) -> Result<Option<PersonalAccessTokenRow>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query_as!(
            PersonalAccessTokenRow,
            r#"SELECT id AS "id!: i64", user_id AS "user_id!: i64", name AS "name!: String",
                      token_prefix AS "token_prefix!: String", expires_at, last_used_at,
                      revoked_at, created_at AS "created_at!: i64"
               FROM personal_access_tokens
               WHERE token_hash = ? AND revoked_at IS NULL
                 AND (expires_at IS NULL OR expires_at > strftime('%s', 'now'))"#,
            token_hash
        )
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(ref token) = row {
            sqlx::query!(
                "UPDATE personal_access_tokens SET last_used_at = strftime('%s', 'now') WHERE id = ?",
                token.id
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(row)
    }

    pub async fn revoke(&self, user_id: i64, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE personal_access_tokens SET revoked_at = strftime('%s', 'now') WHERE id = ? AND user_id = ? AND revoked_at IS NULL",
            id,
            user_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }
}
