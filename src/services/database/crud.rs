use super::{
    DatabaseKind, DatabaseRecord, DatabaseService,
    queries::{generate_app_name, random_secret, slug_value},
};
use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto};

use crate::core::cache::{CacheEnum, CacheKey};

impl DatabaseService {
    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<DatabaseRecord>> {
        self.repo_postgres
            .list_all_by_environment(environment_id)
            .await
    }

    pub async fn get_by_id(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<DatabaseRecord> {
        let key = CacheKey::Database(id);
        let res = self
            .cache
            .try_get_with(key, async {
                let record = match kind {
                    DatabaseKind::Postgres => self.repo_postgres.get_by_id(id).await?,
                    DatabaseKind::Mysql => self.repo_mysql.get_by_id(id).await?,
                    DatabaseKind::Mariadb => self.repo_mariadb.get_by_id(id).await?,
                    DatabaseKind::Mongo => self.repo_mongo.get_by_id(id).await?,
                    DatabaseKind::Redis => self.repo_redis.get_by_id(id).await?,
                    DatabaseKind::Libsql => self.repo_libsql.get_by_id(id).await?,
                };
                Ok::<_, sqlx::Error>(CacheEnum::Database(record))
            })
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        if let CacheEnum::Database(record) = res {
            Ok(record)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }

    pub async fn create(
        &self,
        kind: DatabaseKind,
        input: CreateDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        let app_name = generate_app_name(&input.name, kind.as_str());
        let image = input
            .docker_image
            .clone()
            .unwrap_or_else(|| kind.default_image().into());
        let db_name = input
            .database_name
            .clone()
            .unwrap_or_else(|| slug_value(&input.name));
        let db_user = input
            .database_user
            .clone()
            .unwrap_or_else(|| "openoxide".into());
        let db_password = input
            .database_password
            .clone()
            .unwrap_or_else(random_secret);
        let root_password = input
            .database_root_password
            .clone()
            .unwrap_or_else(random_secret);

        let id = match kind {
            DatabaseKind::Postgres => {
                self.repo_postgres
                    .create(&input, &app_name, &image, &db_name, &db_user, &db_password)
                    .await?
            }
            DatabaseKind::Mysql => {
                self.repo_mysql
                    .create(
                        &input,
                        &app_name,
                        &image,
                        &db_name,
                        &db_user,
                        &db_password,
                        &root_password,
                    )
                    .await?
            }
            DatabaseKind::Mariadb => {
                self.repo_mariadb
                    .create(
                        &input,
                        &app_name,
                        &image,
                        &db_name,
                        &db_user,
                        &db_password,
                        &root_password,
                    )
                    .await?
            }
            DatabaseKind::Mongo => {
                self.repo_mongo
                    .create(&input, &app_name, &image, &db_user, &db_password)
                    .await?
            }
            DatabaseKind::Redis => {
                self.repo_redis
                    .create(&input, &app_name, &image, &db_password)
                    .await?
            }
            DatabaseKind::Libsql => {
                self.repo_libsql
                    .create(&input, &app_name, &image, &db_user, &db_password)
                    .await?
            }
        };

        self.get_by_id(kind, id).await
    }

    pub async fn patch(
        &self,
        kind: DatabaseKind,
        id: i64,
        input: PatchDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        match kind {
            DatabaseKind::Postgres => self.repo_postgres.update(id, &input).await?,
            DatabaseKind::Mysql => self.repo_mysql.update(id, &input).await?,
            DatabaseKind::Mariadb => self.repo_mariadb.update(id, &input).await?,
            DatabaseKind::Mongo => self.repo_mongo.update(id, &input).await?,
            DatabaseKind::Redis => self.repo_redis.update(id, &input).await?,
            DatabaseKind::Libsql => self.repo_libsql.update(id, &input).await?,
        }

        self.cache.invalidate(&CacheKey::Database(id)).await;
        let record = self.get_by_id(kind, id).await?;

        if let Some(new_password) = &input.database_password {
            if !new_password.is_empty() {
                let db_ref = self.db.clone();
                let pwd = new_password.clone();
                let app_name = record.app_name.clone();
                let server_id = record.server_id;
                let user_name = record.database_user.clone().unwrap_or_else(|| "openoxide".to_string());
                tokio::spawn(async move {
                    let docker = match server_id {
                        Some(sid) => {
                            if let Ok(executor) = crate::services::compose::remote::remote_executor(db_ref.as_ref(), sid).await {
                                crate::utils::docker::DockerCli::from_remote_executor(executor)
                            } else {
                                crate::utils::docker::DockerCli::new_local()
                            }
                        }
                        None => crate::utils::docker::DockerCli::new_local(),
                    };

                    let target_container = format!("{}_db", app_name);
                    let escaped_pwd = pwd.replace('\'', "''");
                    match kind {
                        DatabaseKind::Postgres => {
                            let sql = format!("ALTER USER \"{}\" WITH PASSWORD '{}';", user_name, escaped_pwd);
                            let _ = docker.containers().exec(&target_container).run(["psql", "-U", &user_name, "-d", "postgres", "-c", &sql]).await;
                            let _ = docker.containers().exec(&target_container).run(["psql", "-U", "postgres", "-c", &sql]).await;
                        }
                        DatabaseKind::Mysql | DatabaseKind::Mariadb => {
                            let sql = format!("ALTER USER '{}'@'%' IDENTIFIED BY '{}'; FLUSH PRIVILEGES;", user_name, escaped_pwd);
                            let _ = docker.containers().exec(&target_container).run(["mysql", "-u", "root", "-e", &sql]).await;
                        }
                        DatabaseKind::Redis => {
                            let _ = docker.containers().exec(&target_container).run(["redis-cli", "CONFIG", "SET", "requirepass", &pwd]).await;
                        }
                        _ => {}
                    }
                });
            }
        }

        let needs_redeploy = input.external_port.is_some()
            || input.docker_image.is_some()
            || input.memory_limit.is_some()
            || input.cpu_limit.is_some()
            || input.replicas.is_some()
            || input.network_ids.is_some();

        if needs_redeploy {
            let operations = self.operations.clone();
            tokio::spawn(async move {
                let _ = operations.deploy(kind, id).await;
            });
        }

        Ok(record)
    }

    pub async fn delete(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.get_by_id(kind, id).await?;
        let dependencies = self.repo_dependencies.database(kind.as_str(), id).await?;
        if dependencies.blocks_delete() {
            return Err(sqlx::Error::Protocol(format!(
                "database has active dependencies: active_deployments={}, enabled_backups={}",
                dependencies.active_deployments, dependencies.enabled_backups
            )));
        }
        match kind {
            DatabaseKind::Postgres => self.repo_postgres.delete(id).await?,
            DatabaseKind::Mysql => self.repo_mysql.delete(id).await?,
            DatabaseKind::Mariadb => self.repo_mariadb.delete(id).await?,
            DatabaseKind::Mongo => self.repo_mongo.delete(id).await?,
            DatabaseKind::Redis => self.repo_redis.delete(id).await?,
            DatabaseKind::Libsql => self.repo_libsql.delete(id).await?,
        }
        self.cache.invalidate(&CacheKey::Database(id)).await;
        Ok(())
    }

    pub async fn dependencies(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<crate::repository::ResourceDependencyCounts> {
        self.get_by_id(kind, id).await?;
        self.repo_dependencies.database(kind.as_str(), id).await
    }
}
