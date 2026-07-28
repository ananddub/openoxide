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
        if matches!(operation, DatabaseOperation::Stop) {
            self.cancel_operation(kind, id).await?;
            return Ok(DatabaseOperationResult {
                database: self.get_by_id(kind, id).await?,
                operation: DatabaseOperation::Stop,
            });
        }

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

        let target_status = match operation {
            DatabaseOperation::Start
            | DatabaseOperation::Deploy
            | DatabaseOperation::Redeploy
            | DatabaseOperation::Reload => "STARTING",
            DatabaseOperation::Stop => "STOPPING",
        };

        match kind {
            DatabaseKind::Postgres => self.repo_postgres.update_status(id, target_status).await?,
            DatabaseKind::Mysql => self.repo_mysql.update_status(id, target_status).await?,
            DatabaseKind::Mariadb => self.repo_mariadb.update_status(id, target_status).await?,
            DatabaseKind::Mongo => self.repo_mongo.update_status(id, target_status).await?,
            DatabaseKind::Redis => self.repo_redis.update_status(id, target_status).await?,
            DatabaseKind::Libsql => self.repo_libsql.update_status(id, target_status).await?,
        };
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Database(id))
            .await;

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

    pub async fn cancel_operation(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<bool> {
        let queue = resolve::<BuilderQueue>()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        let _ = queue.cancel_queued_database(id).await?;

        if let Ok(state) = resolve::<crate::utils::builder::hash_state::ApplicationState>().await {
            state.cancel_by_id(crate::utils::builder::custom_type::IdType::DatabaseId(id));
        }

        if let (Ok(app_name), Ok((server_id, _))) = (
            self.repo_deploy
                .get_database_app_name(id, kind.as_str())
                .await,
            self.database_server_id_and_name(kind, id).await,
        ) {
            let docker_cli = self.database_docker(server_id).await?;
            if let Ok(services) = docker_cli
                .services()
                .list()
                .filter(crate::utils::docker::query::ServiceFilter::name(format!(
                    "{}_",
                    app_name
                )))
                .run_json()
                .await
            {
                for s in services {
                    if &s.replicas != "0/0" {
                        let _ = docker_cli
                            .services()
                            .scale()
                            .service(&s.name, 0)
                            .run()
                            .await;
                    }
                }
            } else {
                let service_name = format!("{}_db", app_name);
                let _ = docker_cli
                    .services()
                    .scale()
                    .service(&service_name, 0)
                    .run()
                    .await;
            }
        }

        match kind {
            DatabaseKind::Postgres => self.repo_postgres.update_status(id, "STOPPED").await?,
            DatabaseKind::Mysql => self.repo_mysql.update_status(id, "STOPPED").await?,
            DatabaseKind::Mariadb => self.repo_mariadb.update_status(id, "STOPPED").await?,
            DatabaseKind::Mongo => self.repo_mongo.update_status(id, "STOPPED").await?,
            DatabaseKind::Redis => self.repo_redis.update_status(id, "STOPPED").await?,
            DatabaseKind::Libsql => self.repo_libsql.update_status(id, "STOPPED").await?,
        };
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Database(id))
            .await;

        Ok(true)
    }

    async fn database_server_id_and_name(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<(Option<i64>, String)> {
        match kind {
            DatabaseKind::Postgres => self.repo_postgres.get_server_id_and_name(id).await,
            DatabaseKind::Mysql => self.repo_mysql.get_server_id_and_name(id).await,
            DatabaseKind::Mariadb => self.repo_mariadb.get_server_id_and_name(id).await,
            DatabaseKind::Mongo => self.repo_mongo.get_server_id_and_name(id).await,
            DatabaseKind::Redis => self.repo_redis.get_server_id_and_name(id).await,
            DatabaseKind::Libsql => self.repo_libsql.get_server_id_and_name(id).await,
        }
    }

    async fn database_docker(
        &self,
        server_id: Option<i64>,
    ) -> sqlx::Result<crate::utils::docker::DockerCli> {
        match server_id {
            Some(server_id) => {
                let executor =
                    crate::services::compose::remote::remote_executor(self.db.as_ref(), server_id)
                        .await
                        .map_err(sqlx::Error::Protocol)?;
                Ok(crate::utils::docker::DockerCli::from_remote_executor(
                    executor,
                ))
            }
            None => Ok(crate::utils::docker::DockerCli::new_local()),
        }
    }
}
