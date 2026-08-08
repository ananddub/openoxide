use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct MonitoringAgentStatus {
    pub server_id: i64,
    pub organization_id: i64,
    pub last_seen_at: Option<i64>,
    pub agent_version: Option<String>,
}

pub struct MonitoringAgentRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl MonitoringAgentRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn rotate(
        &self,
        server_id: i64,
        organization_id: i64,
        token: &str,
        hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO monitoring_agents (server_id, organization_id, token_hash, query_token, updated_at)
             VALUES (?, ?, ?, ?, strftime('%s','now'))
             ON CONFLICT(server_id) DO UPDATE SET organization_id=excluded.organization_id,
             token_hash=excluded.token_hash, query_token=excluded.query_token, updated_at=excluded.updated_at",
            server_id,
            organization_id,
            hash,
            token
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn server_belongs_to_organization(
        &self,
        server_id: i64,
        organization_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let linked = sqlx::query_scalar!(
            "SELECT EXISTS(
                SELECT 1 FROM applications a JOIN environments e ON e.id=a.environment_id JOIN projects p ON p.id=e.project_id WHERE a.server_id=? AND p.organization_id=?
                UNION ALL
                SELECT 1 FROM compose_projects c JOIN environments e ON e.id=c.environment_id JOIN projects p ON p.id=e.project_id WHERE c.server_id=? AND p.organization_id=?
            )",
            server_id,
            organization_id,
            server_id,
            organization_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;

        if linked != 0 {
            return Ok(true);
        }

        let existing = sqlx::query_scalar!(
            "SELECT organization_id FROM monitoring_agents WHERE server_id=?",
            server_id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;

        Ok(existing == Some(organization_id))
    }

    pub async fn get_token_hash(&self, server_id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar!(
            "SELECT token_hash FROM monitoring_agents WHERE server_id = ?",
            server_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn get_organization_id(&self, server_id: i64) -> Result<Option<i64>, sqlx::Error> {
        sqlx::query_scalar!(
            "SELECT organization_id FROM monitoring_agents WHERE server_id=?",
            server_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn touch_seen(&self, server_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE monitoring_agents SET last_seen_at=strftime('%s','now'), updated_at=strftime('%s','now') WHERE server_id=?",
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn query_token(&self, server_id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar!(
            "SELECT query_token FROM monitoring_agents WHERE server_id=?",
            server_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map(Option::flatten)
    }

    pub async fn status(
        &self,
        server_id: i64,
    ) -> Result<Option<MonitoringAgentStatus>, sqlx::Error> {
        sqlx::query_as!(
            MonitoringAgentStatus,
            r#"SELECT server_id AS "server_id!: i64", organization_id AS "organization_id!: i64", last_seen_at, agent_version FROM monitoring_agents WHERE server_id=?"#,
            server_id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
