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
        resolve::<crate::repository::ServerManagementRepository>()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?
            .assert_deployable(server_id)
            .await?;

        if matches!(operation, DatabaseOperation::Stop) {
            let _ = self.cancel_operation(kind, id).await;
            return Ok(DatabaseOperationResult {
                database: self.get_by_id(kind, id).await?,
                operation,
            });
        }

        let target_status = match operation {
            DatabaseOperation::Deploy | DatabaseOperation::Redeploy => "DEPLOYING",
            _ => "STARTING",
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

        let log_path = crate::utils::paths::openoxide_paths().deployment_log_file(deployment_id);
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

    pub async fn stop_operation(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<bool> {
        self.cancel_operation(kind, id).await
    }

    pub async fn cancel_operation(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<bool> {
        let db_record = self.get_by_id(kind, id).await.ok();
        let current_st_str = db_record.as_ref().map(|d| d.app_status.as_str()).unwrap_or("IDLE");
        let current_status = crate::services::database::types::DatabaseRuntimeStatus::from_str(current_st_str)
            .unwrap_or(crate::services::database::types::DatabaseRuntimeStatus::Idle);

        let (intermediate, final_st) = if current_status.is_building() {
            (
                crate::services::database::types::DatabaseRuntimeStatus::Cancelling.as_str(),
                crate::services::database::types::DatabaseRuntimeStatus::Cancelled.as_str(),
            )
        } else {
            (
                crate::services::database::types::DatabaseRuntimeStatus::Stopping.as_str(),
                crate::services::database::types::DatabaseRuntimeStatus::Stopped.as_str(),
            )
        };

        match kind {
            DatabaseKind::Postgres => {
                let _ = self.repo_postgres.update_status(id, intermediate).await;
            }
            DatabaseKind::Mysql => {
                let _ = self.repo_mysql.update_status(id, intermediate).await;
            }
            DatabaseKind::Mariadb => {
                let _ = self.repo_mariadb.update_status(id, intermediate).await;
            }
            DatabaseKind::Mongo => {
                let _ = self.repo_mongo.update_status(id, intermediate).await;
            }
            DatabaseKind::Redis => {
                let _ = self.repo_redis.update_status(id, intermediate).await;
            }
            DatabaseKind::Libsql => {
                let _ = self.repo_libsql.update_status(id, intermediate).await;
            }
        }
        self.cache
            .invalidate(&crate::core::cache::CacheKey::Database(id))
            .await;

        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_database(id).await;
        }

        if let Ok(state) = resolve::<crate::utils::builder::hash_state::ApplicationState>().await {
            state.cancel_by_id(crate::utils::builder::custom_type::IdType::DatabaseId(id));
        }

        let _ = self.repo_deploy.cancel_queued_for_database(id).await;
        let _ = self.repo_deploy.request_cancel_database_deployment(id).await;

        let app_name = db_record.as_ref().map(|d| d.app_name.clone());
        let server_id = db_record.as_ref().and_then(|d| d.server_id);
        let db_ref = self.db.clone();
        let repo_postgres = self.repo_postgres.clone();
        let repo_mysql = self.repo_mysql.clone();
        let repo_mariadb = self.repo_mariadb.clone();
        let repo_mongo = self.repo_mongo.clone();
        let repo_redis = self.repo_redis.clone();
        let repo_libsql = self.repo_libsql.clone();
        let cache = self.cache.clone();

        // Spawn Docker container cleanup asynchronously so HTTP handler returns INSTANTLY (<5ms)
        tokio::spawn(async move {
            if let Some(ref app_name) = app_name {
                let docker_cli = match server_id {
                    Some(sid) => {
                        if let Ok(executor) = crate::services::compose::remote::remote_executor(db_ref.as_ref(), sid).await {
                            crate::utils::docker::DockerCli::from_remote_executor(executor)
                        } else {
                            crate::utils::docker::DockerCli::new_local()
                        }
                    }
                    None => crate::utils::docker::DockerCli::new_local(),
                };

                let candidates = [
                    app_name.clone(),
                    format!("{}_db", app_name),
                    format!("{}-db", app_name),
                ];

                if let Ok(services) = tokio::time::timeout(
                    std::time::Duration::from_secs(4),
                    docker_cli
                        .services()
                        .list()
                        .filter(crate::utils::docker::query::ServiceFilter::name(format!("{}_", app_name)))
                        .run_json(),
                )
                .await
                .unwrap_or(Err(crate::utils::docker::error::DockerError::CommandFailed { code: None, stderr: "timeout".into() }))
                {
                    for s in services {
                        if &s.replicas != "0/0" {
                            let _ = tokio::time::timeout(
                                std::time::Duration::from_secs(3),
                                docker_cli.services().scale().service(&s.name, 0).run(),
                            )
                            .await;
                        }
                    }
                }

                for candidate in &candidates {
                    let _ = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        docker_cli.services().scale().service(candidate, 0).run(),
                    )
                    .await;
                    let _ = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        docker_cli.container(candidate).stop().run(),
                    )
                    .await;
                }
            }

            match kind {
                DatabaseKind::Postgres => { let _ = repo_postgres.update_status(id, final_st).await; }
                DatabaseKind::Mysql => { let _ = repo_mysql.update_status(id, final_st).await; }
                DatabaseKind::Mariadb => { let _ = repo_mariadb.update_status(id, final_st).await; }
                DatabaseKind::Mongo => { let _ = repo_mongo.update_status(id, final_st).await; }
                DatabaseKind::Redis => { let _ = repo_redis.update_status(id, final_st).await; }
                DatabaseKind::Libsql => { let _ = repo_libsql.update_status(id, final_st).await; }
            }
            cache.invalidate(&crate::core::cache::CacheKey::Database(id)).await;
        });

        Ok(true)
    }

    #[allow(dead_code)]
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
