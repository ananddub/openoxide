use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct PermissionCompatibilityRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl PermissionCompatibilityRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn has_legacy_full_access(&self, user_id: i64) -> sqlx::Result<bool> {
        let exists = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS(SELECT 1 FROM permission_legacy_full_access WHERE user_id=?)",
        )
        .bind(user_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(exists != 0)
    }
}
