use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BackgroundPolicy {
    pub panel_backup_cron: String,
    pub log_cleanup_cron: String,
    pub log_retention_days: i64,
    pub panel_backup_enabled: bool,
    pub log_cleanup_enabled: bool,
}

pub struct BackgroundPolicyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl BackgroundPolicyRepository {
    fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
    pub async fn get(&self) -> sqlx::Result<BackgroundPolicy> {
        let row = sqlx::query("SELECT panel_backup_cron, log_cleanup_cron, log_retention_days, panel_backup_enabled, log_cleanup_enabled FROM background_policies WHERE id = 1").fetch_one(self.pool.as_ref()).await?;
        use sqlx::Row;
        Ok(BackgroundPolicy {
            panel_backup_cron: row.try_get(0)?,
            log_cleanup_cron: row.try_get(1)?,
            log_retention_days: row.try_get(2)?,
            panel_backup_enabled: row.try_get::<i64, _>(3)? != 0,
            log_cleanup_enabled: row.try_get::<i64, _>(4)? != 0,
        })
    }
    pub async fn update(&self, policy: &BackgroundPolicy) -> sqlx::Result<()> {
        sqlx::query("UPDATE background_policies SET panel_backup_cron = ?, log_cleanup_cron = ?, log_retention_days = ?, panel_backup_enabled = ?, log_cleanup_enabled = ?, updated_at = strftime('%s', 'now') WHERE id = 1")
            .bind(&policy.panel_backup_cron).bind(&policy.log_cleanup_cron).bind(policy.log_retention_days)
            .bind(i64::from(policy.panel_backup_enabled)).bind(i64::from(policy.log_cleanup_enabled))
            .execute(self.pool.as_ref()).await?;
        Ok(())
    }
}
