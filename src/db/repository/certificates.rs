use crate::db::models::certificates::Certificate;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct CertificateRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl CertificateRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self, organization_id: i64) -> Result<Vec<Certificate>, sqlx::Error> {
        sqlx::query_as!(
            Certificate,
            r#"SELECT id AS "id?: i64", name AS "name: String", certificate_data AS "certificate_data: String", private_key AS "private_key: String", certificate_path AS "certificate_path: String", auto_renew AS "auto_renew: i64", server_id AS "server_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM certificates WHERE organization_id = ? ORDER BY created_at DESC, id DESC"#,
            organization_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Option<Certificate>, sqlx::Error> {
        sqlx::query_as!(Certificate, r#"SELECT id AS "id?: i64", name AS "name: String", certificate_data AS "certificate_data: String", private_key AS "private_key: String", certificate_path AS "certificate_path: String", auto_renew AS "auto_renew: i64", server_id AS "server_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM certificates WHERE id = ? AND organization_id = ?"#, id, organization_id)
            .fetch_optional(self.pool.as_ref()).await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Certificate>, sqlx::Error> {
        sqlx::query_as!(
            Certificate,
            r#"SELECT id AS "id?: i64", name AS "name: String", certificate_data AS "certificate_data: String", private_key AS "private_key: String", certificate_path AS "certificate_path: String", auto_renew AS "auto_renew: i64", server_id AS "server_id?: i64", organization_id AS "organization_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM certificates WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Certificate) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO certificates (name, certificate_data, private_key, certificate_path, auto_renew, server_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.name,
            &item.certificate_data,
            &item.private_key,
            &item.certificate_path,
            item.auto_renew,
            item.server_id,
            item.organization_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Certificate) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE certificates SET name = ?, certificate_data = ?, private_key = ?, certificate_path = ?, auto_renew = ?, server_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.name,
            &item.certificate_data,
            &item.private_key,
            &item.certificate_path,
            item.auto_renew,
            item.server_id,
            item.organization_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"DELETE FROM certificates WHERE id = ? AND organization_id = ?"#,
            id,
            organization_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM certificates WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
