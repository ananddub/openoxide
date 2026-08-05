use auto_di::singleton;
use sqlx::SqlitePool;

#[derive(Clone, Copy)]
enum RenewalStatus {
    Succeeded,
    Failed,
}
impl RenewalStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "SUCCEEDED",
            Self::Failed => "FAILED",
        }
    }
}
use std::sync::Arc;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct CertificateRenewal {
    pub id: i64,
    pub certificate_id: i64,
    pub organization_id: i64,
    pub status: String,
    pub previous_expires_at: Option<i64>,
    pub new_expires_at: Option<i64>,
    pub error: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}

pub struct CertificateRenewalRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl CertificateRenewalRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
    pub async fn begin(
        &self,
        certificate_id: i64,
        organization_id: i64,
        previous_expires_at: Option<i64>,
    ) -> sqlx::Result<i64> {
        let result = sqlx::query!("INSERT INTO certificate_renewals (certificate_id, organization_id, status, previous_expires_at) VALUES (?, ?, 'RUNNING', ?)", certificate_id, organization_id, previous_expires_at).execute(self.pool.as_ref()).await?;
        Ok(result.last_insert_rowid())
    }
    pub async fn finish(
        &self,
        id: i64,
        new_expires_at: Option<i64>,
        error: Option<&str>,
    ) -> sqlx::Result<()> {
        let status = if error.is_some() {
            RenewalStatus::Failed
        } else {
            RenewalStatus::Succeeded
        };
        sqlx::query!("UPDATE certificate_renewals SET status=?, new_expires_at=?, error=?, finished_at=unixepoch() WHERE id=?", status.as_str(), new_expires_at, error, id).execute(self.pool.as_ref()).await?;
        Ok(())
    }
    pub async fn list(&self, certificate_id: i64) -> sqlx::Result<Vec<CertificateRenewal>> {
        sqlx::query_as!(CertificateRenewal, r#"SELECT id AS "id!: i64", certificate_id AS "certificate_id!: i64", organization_id AS "organization_id!: i64", status AS "status!: String", previous_expires_at, new_expires_at, error, started_at AS "started_at!: i64", finished_at FROM certificate_renewals WHERE certificate_id=? ORDER BY started_at DESC LIMIT 200"#, certificate_id).fetch_all(self.pool.as_ref()).await
    }
}
