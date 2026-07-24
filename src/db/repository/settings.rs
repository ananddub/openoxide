use crate::db::models::settings::Setting;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct SettingRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl SettingRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Setting>, sqlx::Error> {
        sqlx::query_as!(
            Setting,
            r#"SELECT id AS "id?: i64", server_ip AS "server_ip?: String", certificate_type AS "certificate_type: String", custom_cert_resolver AS "custom_cert_resolver?: String", https AS "https: i64", host AS "host?: String", lets_encrypt_email AS "lets_encrypt_email?: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", metrics_config AS "metrics_config: String", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM settings"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Setting>, sqlx::Error> {
        sqlx::query_as!(
            Setting,
            r#"SELECT id AS "id?: i64", server_ip AS "server_ip?: String", certificate_type AS "certificate_type: String", custom_cert_resolver AS "custom_cert_resolver?: String", https AS "https: i64", host AS "host?: String", lets_encrypt_email AS "lets_encrypt_email?: String", enable_docker_cleanup AS "enable_docker_cleanup: i64", log_cleanup_cron AS "log_cleanup_cron?: String", metrics_config AS "metrics_config: String", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM settings WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Setting) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO settings (server_ip, certificate_type, custom_cert_resolver, https, host, lets_encrypt_email, enable_docker_cleanup, log_cleanup_cron, metrics_config, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.server_ip,
            &item.certificate_type,
            &item.custom_cert_resolver,
            item.https,
            &item.host,
            &item.lets_encrypt_email,
            item.enable_docker_cleanup,
            &item.log_cleanup_cron,
            &item.metrics_config,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Setting) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE settings SET server_ip = ?, certificate_type = ?, custom_cert_resolver = ?, https = ?, host = ?, lets_encrypt_email = ?, enable_docker_cleanup = ?, log_cleanup_cron = ?, metrics_config = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.server_ip,
            &item.certificate_type,
            &item.custom_cert_resolver,
            item.https,
            &item.host,
            &item.lets_encrypt_email,
            item.enable_docker_cleanup,
            &item.log_cleanup_cron,
            &item.metrics_config,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM settings WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
