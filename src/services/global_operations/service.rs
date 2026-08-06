use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use super::{GlobalResourceDto, GlobalSearchOptions};

pub struct GlobalOperationsService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl GlobalOperationsService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn search(
        &self,
        options: GlobalSearchOptions,
    ) -> sqlx::Result<Vec<GlobalResourceDto>> {
        let pattern = format!("%{}%", options.query.trim());
        let rows = sqlx::query!(r#"
            SELECT resource_type, id, name, status FROM (
                SELECT 'APPLICATION' resource_type, CAST(id AS TEXT) id, name, app_status status FROM applications
                UNION ALL SELECT 'COMPOSE', CAST(id AS TEXT), name, compose_status FROM compose_projects
                UNION ALL SELECT 'SERVER', CAST(id AS TEXT), name, server_status FROM servers
                UNION ALL SELECT 'PROJECT', CAST(id AS TEXT), name, NULL FROM projects
                UNION ALL SELECT 'ENVIRONMENT', CAST(id AS TEXT), name, NULL FROM environments
                UNION ALL SELECT 'POSTGRES', CAST(id AS TEXT), name, app_status FROM postgres_dbs
                UNION ALL SELECT 'MYSQL', CAST(id AS TEXT), name, app_status FROM mysql_dbs
                UNION ALL SELECT 'MARIADB', CAST(id AS TEXT), name, app_status FROM mariadb_dbs
                UNION ALL SELECT 'MONGO', CAST(id AS TEXT), name, app_status FROM mongo_dbs
                UNION ALL SELECT 'REDIS', CAST(id AS TEXT), name, app_status FROM redis_dbs
                UNION ALL SELECT 'LIBSQL', CAST(id AS TEXT), name, app_status FROM libsql_dbs
            ) resources WHERE name LIKE ? COLLATE NOCASE ORDER BY name LIMIT ?
        "#, pattern, options.limit).fetch_all(self.db.as_ref()).await?;
        Ok(rows
            .into_iter()
            .map(|row| GlobalResourceDto {
                resource_type: row.resource_type,
                id: row.id,
                name: row.name,
                status: row.status,
            })
            .collect())
    }

    pub async fn cleanup_deployment_queue(&self) -> sqlx::Result<u64> {
        let result = sqlx::query!("UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE status = 'QUEUED'").execute(self.db.as_ref()).await?;
        Ok(result.rows_affected())
    }
}
