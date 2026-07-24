use auto_di::resolve;

use super::{DatabaseKind, DatabaseOperation, DatabaseOperationResult, DatabaseService};
use crate::utils::builder::queue::BuilderQueue;

impl DatabaseService {
    pub async fn run_operation(
        &self,
        kind: DatabaseKind,
        id: i64,
        operation: DatabaseOperation,
    ) -> sqlx::Result<DatabaseOperationResult> {
        let running_deployment = self.repo_deploy.has_running_database_deployment(id).await?;
        if running_deployment {
            return Err(sqlx::Error::Protocol(
                "database deployment already queued or running; cancel it first".into(),
            ));
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .ensure_capacity()
            .await?;

        let kind_str = kind.as_str();

        let (server_id, name) = match kind {
            DatabaseKind::Postgres => self.repo_postgres.get_server_id_and_name(id).await?,
            DatabaseKind::Mysql => self.repo_mysql.get_server_id_and_name(id).await?,
            DatabaseKind::Mariadb => self.repo_mariadb.get_server_id_and_name(id).await?,
            DatabaseKind::Mongo => self.repo_mongo.get_server_id_and_name(id).await?,
            DatabaseKind::Redis => self.repo_redis.get_server_id_and_name(id).await?,
            DatabaseKind::Libsql => self.repo_libsql.get_server_id_and_name(id).await?,
        };

        match kind {
            DatabaseKind::Postgres => self.repo_postgres.update_status(id, "RUNNING").await?,
            DatabaseKind::Mysql => self.repo_mysql.update_status(id, "RUNNING").await?,
            DatabaseKind::Mariadb => self.repo_mariadb.update_status(id, "RUNNING").await?,
            DatabaseKind::Mongo => self.repo_mongo.update_status(id, "RUNNING").await?,
            DatabaseKind::Redis => self.repo_redis.update_status(id, "RUNNING").await?,
            DatabaseKind::Libsql => self.repo_libsql.update_status(id, "RUNNING").await?,
        };

        let log_path = format!("pending-db-{}", id);
        let deployment_id = self
            .repo_deploy
            .create_queued_database_deployment(
                operation.title().to_string(),
                Some(format!(
                    "{} requested for database {}",
                    operation.as_str(),
                    name
                )),
                log_path,
                operation.as_str().to_string(),
                id,
                kind_str.to_string(),
                server_id,
            )
            .await?;

        let log_path = crate::utils::paths::rustploy_paths().deployment_log_file(deployment_id);
        self.repo_deploy
            .update_log_path(deployment_id, &log_path)
            .await?;

        if let Ok(mut log) =
            crate::utils::builder::queue::deployment_log::DeploymentLog::open(deployment_id).await
        {
            let _ = log
                .write_line(&format!(
                    "[QUEUED] database deployment queued for {}",
                    operation.as_str()
                ))
                .await;
        }

        resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .notify();

        Ok(DatabaseOperationResult {
            database: self.get_by_id(kind, id).await?,
            operation,
        })
    }
}
