use crate::db::models::two_factor::TwoFactor;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct TwoFactorRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl TwoFactorRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<TwoFactor>, sqlx::Error> {
        sqlx::query_as!(
            TwoFactor,
            r#"SELECT id AS "id?: i64", secret AS "secret: String", backup_codes AS "backup_codes: String", user_id AS "user_id: i64" FROM two_factor"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<TwoFactor>, sqlx::Error> {
        sqlx::query_as!(
            TwoFactor,
            r#"SELECT id AS "id?: i64", secret AS "secret: String", backup_codes AS "backup_codes: String", user_id AS "user_id: i64" FROM two_factor WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &TwoFactor) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO two_factor (secret, backup_codes, user_id) VALUES (?, ?, ?)"#,
            &item.secret,
            &item.backup_codes,
            item.user_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &TwoFactor) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE two_factor SET secret = ?, backup_codes = ?, user_id = ? WHERE id = ?"#,
            &item.secret,
            &item.backup_codes,
            item.user_id,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM two_factor WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_by_user_id(&self, user_id: i64) -> Result<Option<TwoFactor>, sqlx::Error> {
        sqlx::query_as!(
            TwoFactor,
            r#"SELECT id AS "id?: i64", secret AS "secret: String", backup_codes AS "backup_codes: String", user_id AS "user_id: i64" FROM two_factor WHERE user_id = ?"#,
            user_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn upsert(&self, item: &TwoFactor) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO two_factor (secret, backup_codes, user_id) VALUES (?, ?, ?)
               ON CONFLICT(user_id) DO UPDATE SET secret = excluded.secret, backup_codes = excluded.backup_codes"#,
            &item.secret,
            &item.backup_codes,
            item.user_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn consume_recovery_code(
        &self,
        user_id: i64,
        backup_codes: &str,
        code_hash: &str,
    ) -> Result<bool, sqlx::Error> {
        let mut codes: Vec<String> = serde_json::from_str(backup_codes).unwrap_or_default();
        let Some(index) = codes.iter().position(|value| value == code_hash) else {
            return Ok(false);
        };
        codes.remove(index);
        let updated = serde_json::to_string(&codes).map_err(|error| {
            sqlx::Error::Protocol(format!("could not encode recovery codes: {error}"))
        })?;
        let result = sqlx::query!(
            "UPDATE two_factor SET backup_codes = ? WHERE user_id = ? AND backup_codes = ?",
            updated,
            user_id,
            backup_codes
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn replace_recovery_codes(
        &self,
        user_id: i64,
        backup_codes: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE two_factor SET backup_codes = ? WHERE user_id = ?",
            backup_codes,
            user_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete_by_user_id(&self, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM two_factor WHERE user_id = ?", user_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
