use crate::db::models::deployments::Deployment;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DeploymentRepository {
    pool: Arc<SqlitePool>,
}

#[derive(Debug, Clone)]
pub struct DeploymentClaim {
    pub id: i64,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub database_id: Option<i64>,
    pub database_kind: Option<String>,
    pub operation: Option<String>,
}

#[singleton]
impl DeploymentRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Deployment>, sqlx::Error> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?: i64", title AS "title: String", description AS "description?: String", status AS "status: String", state AS "state: String", log_path AS "log_path: String", pid AS "pid?: String", error_message AS "error_message?: String", operation AS "operation?: String", is_preview_deployment AS "is_preview_deployment: i64", started_at AS "started_at?: i64", last_state_at AS "last_state_at?: i64", finished_at AS "finished_at?: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", created_at AS "created_at: i64", database_id AS "database_id?: i64", database_kind AS "database_kind?: String" FROM deployments"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Deployment>, sqlx::Error> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?: i64", title AS "title: String", description AS "description?: String", status AS "status: String", state AS "state: String", log_path AS "log_path: String", pid AS "pid?: String", error_message AS "error_message?: String", operation AS "operation?: String", is_preview_deployment AS "is_preview_deployment: i64", started_at AS "started_at?: i64", last_state_at AS "last_state_at?: i64", finished_at AS "finished_at?: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", created_at AS "created_at: i64", database_id AS "database_id?: i64", database_kind AS "database_kind?: String" FROM deployments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Deployment) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, pid, error_message, operation, is_preview_deployment, started_at, last_state_at, finished_at, application_id, compose_id, server_id, created_at, database_id, database_kind) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.title,
            &item.description,
            &item.status,
            &item.state,
            &item.log_path,
            &item.pid,
            &item.error_message,
            &item.operation,
            item.is_preview_deployment,
            item.started_at,
            item.last_state_at,
            item.finished_at,
            item.application_id,
            item.compose_id,
            item.server_id,
            item.created_at,
            item.database_id,
            &item.database_kind
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Deployment) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET title = ?, description = ?, status = ?, state = ?, log_path = ?, pid = ?, error_message = ?, operation = ?, is_preview_deployment = ?, started_at = ?, last_state_at = ?, finished_at = ?, application_id = ?, compose_id = ?, server_id = ?, created_at = ?, database_id = ?, database_kind = ? WHERE id = ?"#,
            &item.title,
            &item.description,
            &item.status,
            &item.state,
            &item.log_path,
            &item.pid,
            &item.error_message,
            &item.operation,
            item.is_preview_deployment,
            item.started_at,
            item.last_state_at,
            item.finished_at,
            item.application_id,
            item.compose_id,
            item.server_id,
            item.created_at,
            item.database_id,
            &item.database_kind,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM deployments WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn has_running_deployment(&self, application_id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "exists_val!: i64""#,
            application_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn create_queued_deployment(
        &self,
        title: String,
        description: Option<String>,
        log_path: String,
        operation: String,
        application_id: i64,
        server_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, application_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'))"#,
            title,
            description,
            log_path,
            operation,
            application_id,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn update_log_path(&self, id: i64, log_path: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET log_path = ? WHERE id = ?"#,
            log_path,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn update_state_if_running(
        &self,
        id: i64,
        state: &str,
        error_message: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE deployments
             SET state         = ?,
                 error_message = COALESCE(?, error_message),
                 last_state_at = strftime('%s', 'now')
             WHERE id = ? AND status = 'RUNNING'",
        )
        .bind(state)
        .bind(error_message)
        .bind(id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn update_final_status(
        &self,
        id: i64,
        status: &str,
        error_message: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE deployments
             SET status        = ?,
                 state         = ?,
                 error_message = COALESCE(?, error_message),
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE id = ?",
        )
        .bind(status)
        .bind(status)
        .bind(error_message)
        .bind(id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn finalize_with_resource(
        &self,
        id: i64,
        deployment_status: &str,
        error_message: Option<&str>,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        database_kind: Option<&str>,
        resource_status: &str,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            "UPDATE deployments SET status=?, state=?, error_message=COALESCE(?, error_message), finished_at=strftime('%s','now'), last_state_at=strftime('%s','now'), pid=NULL WHERE id=?",
        )
        .bind(deployment_status).bind(deployment_status).bind(error_message).bind(id)
        .execute(&mut *tx).await?;

        if let Some(resource_id) = application_id {
            sqlx::query("UPDATE applications SET app_status=? WHERE id=?")
                .bind(resource_status).bind(resource_id).execute(&mut *tx).await?;
        } else if let Some(resource_id) = compose_id {
            sqlx::query("UPDATE compose_projects SET compose_status=? WHERE id=?")
                .bind(resource_status).bind(resource_id).execute(&mut *tx).await?;
        } else if let (Some(resource_id), Some(kind)) = (database_id, database_kind) {
            let table = match kind.to_ascii_lowercase().as_str() {
                "postgres" => "postgres_dbs", "mysql" => "mysql_dbs",
                "mariadb" => "mariadb_dbs", "mongo" => "mongo_dbs",
                "redis" => "redis_dbs", "libsql" => "libsql_dbs",
                _ => return Err(sqlx::Error::Protocol(format!("invalid database kind: {kind}"))),
            };
            let query = format!("UPDATE {table} SET app_status=? WHERE id=?");
            sqlx::query(sqlx::AssertSqlSafe(query)).bind(resource_status).bind(resource_id)
                .execute(&mut *tx).await?;
        }
        tx.commit().await
    }

    pub async fn set_application_status(&self, id: i64, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE applications SET app_status = ? WHERE id = ?")
            .bind(status)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn set_compose_status(&self, id: i64, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE compose_projects SET compose_status = ? WHERE id = ?")
            .bind(status)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn set_database_status(
        &self,
        id: i64,
        db_kind: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        let table_name = match db_kind {
            "postgres" => "postgres_dbs",
            "mysql" => "mysql_dbs",
            "mariadb" => "mariadb_dbs",
            "mongo" => "mongo_dbs",
            "redis" => "redis_dbs",
            "libsql" => "libsql_dbs",
            _ => return Ok(()),
        };
        let query_str = format!("UPDATE {} SET app_status = ? WHERE id = ?", table_name);
        sqlx::query(sqlx::AssertSqlSafe(&*query_str))
            .bind(status)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_database_deployment_context(
        &self,
        db_id: i64,
        kind: &str,
    ) -> Result<(Option<i64>, i64, i64, Option<String>, Option<String>), sqlx::Error> {
        let table = match kind {
            "postgres" => "postgres_dbs",
            "mysql" => "mysql_dbs",
            "mariadb" => "mariadb_dbs",
            "mongo" => "mongo_dbs",
            "redis" => "redis_dbs",
            "libsql" => "libsql_dbs",
            _ => return Err(sqlx::Error::Protocol(format!("unknown db kind: {}", kind))),
        };
        let query_str = format!(
            "SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
             FROM {} d
             JOIN environments e ON e.id = d.environment_id
             WHERE d.id = ?",
            table
        );
        let row = sqlx::query_as::<_, (Option<i64>, i64, i64, Option<String>, Option<String>)>(
            sqlx::AssertSqlSafe(&*query_str),
        )
        .bind(db_id)
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(row)
    }

    /// Owning organization of an application, via its environment's project.
    pub async fn application_organization_id(
        &self,
        application_id: i64,
    ) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar!(
            r#"SELECT p.organization_id AS "organization_id: i64"
               FROM applications a
               JOIN environments e ON e.id = a.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE a.id = ?"#,
            application_id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn compose_organization_id(&self, compose_id: i64) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar!(
            r#"SELECT p.organization_id AS "organization_id: i64"
               FROM compose_projects c
               JOIN environments e ON e.id = c.environment_id
               JOIN projects p ON p.id = e.project_id
               WHERE c.id = ?"#,
            compose_id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    /// Databases live in per-kind tables, so the table name is interpolated
    /// from a fixed allowlist rather than bound as a parameter.
    pub async fn database_organization_id(
        &self,
        database_id: i64,
        kind: &str,
    ) -> Result<i64, sqlx::Error> {
        let table = match kind {
            "postgres" => "postgres_dbs",
            "mysql" => "mysql_dbs",
            "mariadb" => "mariadb_dbs",
            "mongo" => "mongo_dbs",
            "redis" => "redis_dbs",
            "libsql" => "libsql_dbs",
            _ => return Err(sqlx::Error::Protocol(format!("unknown db kind: {kind}"))),
        };

        let query_str = format!(
            "SELECT p.organization_id
             FROM {table} d
             JOIN environments e ON e.id = d.environment_id
             JOIN projects p ON p.id = e.project_id
             WHERE d.id = ?"
        );

        sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(&*query_str))
            .bind(database_id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn get_database_app_name(
        &self,
        db_id: i64,
        kind: &str,
    ) -> Result<String, sqlx::Error> {
        let table = match kind {
            "postgres" => "postgres_dbs",
            "mysql" => "mysql_dbs",
            "mariadb" => "mariadb_dbs",
            "mongo" => "mongo_dbs",
            "redis" => "redis_dbs",
            "libsql" => "libsql_dbs",
            _ => return Err(sqlx::Error::Protocol(format!("unknown db kind: {}", kind))),
        };
        let query_str = format!("SELECT app_name FROM {} WHERE id = ?", table);
        sqlx::query_scalar::<_, String>(sqlx::AssertSqlSafe(&*query_str))
            .bind(db_id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn has_running_status_deployment(
        &self,
        application_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE application_id = ? AND status = 'RUNNING') AS "exists_val!: i64""#,
            application_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn request_cancel_deployment(&self, application_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE application_id = ? AND status = 'RUNNING'"#,
            application_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn has_running_compose_deployment(
        &self,
        compose_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "exists_val!: i64""#,
            compose_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn create_queued_compose_deployment(
        &self,
        title: String,
        description: Option<String>,
        log_path: String,
        operation: String,
        compose_id: i64,
        server_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, compose_id, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, strftime('%s', 'now'))"#,
            title,
            description,
            log_path,
            operation,
            compose_id,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn has_running_status_compose_deployment(
        &self,
        compose_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE compose_id = ? AND status = 'RUNNING') AS "exists_val!: i64""#,
            compose_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn request_cancel_compose_deployment(
        &self,
        compose_id: i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE compose_id = ? AND status = 'RUNNING'"#,
            compose_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn has_running_database_deployment(
        &self,
        database_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM deployments WHERE database_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "exists_val!: i64""#,
            database_id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(res != 0)
    }

    pub async fn create_queued_database_deployment(
        &self,
        title: String,
        description: Option<String>,
        log_path: String,
        operation: String,
        database_id: i64,
        database_kind: String,
        server_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        let res = sqlx::query!(
            r#"INSERT INTO deployments (title, description, status, state, log_path, operation, database_id, database_kind, server_id, last_state_at)
               VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, ?, strftime('%s', 'now'))"#,
            title,
            description,
            log_path,
            operation,
            database_id,
            database_kind,
            server_id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn get_cancel_info(
        &self,
        id: i64,
    ) -> Result<Option<(Option<i64>, Option<i64>, Option<i64>, String)>, sqlx::Error> {
        let res = sqlx::query!(
            r#"SELECT application_id, compose_id, database_id, status FROM deployments WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res.map(|r| (r.application_id, r.compose_id, r.database_id, r.status)))
    }

    pub async fn cancel_queued_deployment(&self, id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'QUEUED'"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn set_cancel_requested(&self, id: i64) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"UPDATE deployments SET state = 'CANCEL_REQUESTED', last_state_at = strftime('%s', 'now') WHERE id = ? AND status = 'RUNNING'"#,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn list_filtered(
        &self,
        status: Option<String>,
        state: Option<String>,
        application_id: Option<i64>,
        compose_id: Option<i64>,
        database_id: Option<i64>,
        server_id: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Deployment>, sqlx::Error> {
        sqlx::query_as!(
            Deployment,
            r#"SELECT id AS "id?: i64", title AS "title: String", description AS "description?: String", status AS "status: String", state AS "state: String", log_path AS "log_path: String", pid AS "pid?: String", error_message AS "error_message?: String", operation AS "operation?: String", is_preview_deployment AS "is_preview_deployment: i64", started_at AS "started_at?: i64", last_state_at AS "last_state_at?: i64", finished_at AS "finished_at?: i64", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", server_id AS "server_id?: i64", created_at AS "created_at: i64", database_id AS "database_id?: i64", database_kind AS "database_kind?: String"
               FROM deployments
               WHERE (? IS NULL OR status = ?)
                 AND (? IS NULL OR state = ?)
                 AND (? IS NULL OR application_id = ?)
                 AND (? IS NULL OR compose_id = ?)
                 AND (? IS NULL OR database_id = ?)
                 AND (? IS NULL OR server_id = ?)
               ORDER BY COALESCE(started_at, created_at) DESC, id DESC
               LIMIT ? OFFSET ?"#,
            status,
            status,
            state,
            state,
            application_id,
            application_id,
            compose_id,
            compose_id,
            database_id,
            database_id,
            server_id,
            server_id,
            limit,
            offset,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_status(&self, id: i64) -> Result<Option<String>, sqlx::Error> {
        let res = sqlx::query_scalar!(r#"SELECT status FROM deployments WHERE id = ?"#, id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(res)
    }

    pub async fn get_latest_application_deployment_id(
        &self,
        application_id: i64,
    ) -> Result<Option<i64>, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT id AS "id: i64" FROM deployments WHERE application_id = ? ORDER BY id DESC LIMIT 1"#,
            application_id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res)
    }

    pub async fn get_latest_compose_deployment_id(
        &self,
        compose_id: i64,
    ) -> Result<Option<i64>, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT id AS "id: i64" FROM deployments WHERE compose_id = ? ORDER BY id DESC LIMIT 1"#,
            compose_id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res)
    }

    pub async fn get_latest_database_deployment_id(
        &self,
        database_id: i64,
    ) -> Result<Option<i64>, sqlx::Error> {
        let res = sqlx::query_scalar!(
            r#"SELECT id AS "id: i64" FROM deployments WHERE database_id = ? ORDER BY id DESC LIMIT 1"#,
            database_id
        )
        .fetch_optional(self.pool.as_ref())
        .await?;
        Ok(res)
    }

    pub async fn get_queued_count(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM deployments WHERE status = 'QUEUED'")
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn get_queued_ids_for_application(
        &self,
        application_id: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT id FROM deployments WHERE application_id = ? AND status = 'QUEUED'",
        )
        .bind(application_id)
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn cancel_queued_for_application(
        &self,
        application_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query(
            "UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE application_id = ? AND status = 'QUEUED'",
        )
        .bind(application_id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(rows.rows_affected() > 0)
    }

    pub async fn get_queued_ids_for_compose(
        &self,
        compose_id: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar("SELECT id FROM deployments WHERE compose_id = ? AND status = 'QUEUED'")
            .bind(compose_id)
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn cancel_queued_for_compose(&self, compose_id: i64) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query(
            "UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE compose_id = ? AND status = 'QUEUED'",
        )
        .bind(compose_id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(rows.rows_affected() > 0)
    }

    pub async fn get_queued_ids_for_database(
        &self,
        database_id: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar("SELECT id FROM deployments WHERE database_id = ? AND status = 'QUEUED'")
            .bind(database_id)
            .fetch_all(self.pool.as_ref())
            .await
    }

    pub async fn cancel_queued_for_database(&self, database_id: i64) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query(
            "UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE database_id = ? AND status = 'QUEUED'",
        )
        .bind(database_id)
        .execute(self.pool.as_ref())
        .await?;
        Ok(rows.rows_affected() > 0)
    }

    pub async fn mark_running_as_recovered(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE deployments
             SET status        = 'ERROR',
                 state         = 'RECOVERED_AFTER_RESTART',
                 error_message = COALESCE(error_message, 'server restarted while deployment was running'),
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE status = 'RUNNING'",
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn recover_stale_application_statuses(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE applications SET app_status = 'ERROR'
             WHERE app_status IN ('STARTING', 'BUILDING')
             AND id IN (
                 SELECT application_id FROM deployments
                 WHERE state = 'RECOVERED_AFTER_RESTART' AND application_id IS NOT NULL
             )",
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn recover_stale_compose_statuses(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE compose_projects SET compose_status = 'ERROR'
             WHERE compose_status IN ('STARTING', 'BUILDING')
             AND id IN (
                 SELECT compose_id FROM deployments
                 WHERE state = 'RECOVERED_AFTER_RESTART' AND compose_id IS NOT NULL
             )",
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn recover_stale_database_statuses(&self) -> Result<(), sqlx::Error> {
        for table in &[
            "postgres_dbs",
            "mysql_dbs",
            "mariadb_dbs",
            "mongo_dbs",
            "redis_dbs",
            "libsql_dbs",
        ] {
            let query_str = format!(
                "UPDATE {} SET app_status = 'ERROR' WHERE app_status IN ('STARTING', 'BUILDING') AND id IN (
                    SELECT database_id FROM deployments WHERE state = 'RECOVERED_AFTER_RESTART' AND database_id IS NOT NULL
                )",
                table
            );
            sqlx::query(sqlx::AssertSqlSafe(&*query_str))
                .execute(self.pool.as_ref())
                .await?;
        }
        Ok(())
    }

    pub async fn get_running_remote_jobs(&self) -> Result<Vec<(i64, i64, String)>, sqlx::Error> {
        sqlx::query_as::<_, (i64, i64, String)>(
            "SELECT id, server_id, pid FROM deployments
             WHERE status = 'RUNNING' AND server_id IS NOT NULL AND pid IS NOT NULL",
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_recovered_remote_jobs(&self) -> Result<Vec<(i64, i64, String)>, sqlx::Error> {
        sqlx::query_as::<_, (i64, i64, String)>(
            "SELECT id, server_id, pid FROM deployments
             WHERE status = 'ERROR' AND state = 'RECOVERED_AFTER_RESTART'
             AND server_id IS NOT NULL AND pid IS NOT NULL",
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn set_pid(&self, id: i64, pid: &str) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE deployments SET pid = ? WHERE id = ?")
            .bind(pid)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn set_pid_null(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE deployments SET pid = NULL WHERE id = ?")
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn get_queued_deployments_grouped(
        &self,
    ) -> Result<Vec<(i64, Option<i64>)>, sqlx::Error> {
        sqlx::query_as::<_, (i64, Option<i64>)>(
            "SELECT id, server_id FROM deployments
             WHERE status = 'QUEUED'
             ORDER BY created_at ASC, id ASC",
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn claim_queued_deployment(
        &self,
        id: i64,
    ) -> Result<Option<DeploymentClaim>, sqlx::Error> {
        sqlx::query_as!(
            DeploymentClaim,
            r#"UPDATE deployments
               SET status        = 'RUNNING',
                   state         = 'QUEUE',
                   started_at    = COALESCE(started_at, strftime('%s', 'now')),
                   last_state_at = strftime('%s', 'now')
               WHERE id = ? AND status = 'QUEUED'
               RETURNING id, application_id, compose_id, database_id, database_kind, operation"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
