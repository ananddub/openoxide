use super::{
    DatabaseKind, DatabaseOperation, DatabaseRecord, DatabaseService,
    queries::{
        ResourceDependencyCounts, build_engine_config, db_type_value, dependency_counts,
        generate_app_name, patch_engine_config, random_secret, slug_value,
        update_service_identity,
    },
};
use serde::Deserialize;

use crate::core::cache::{CacheEnum, CacheKey};
use crate::flash_gen::database_extra::{
    DbCreateDatabaseServiceForServerParams, DbCreateDatabaseServiceGlobalParams,
    DbCreateServiceDatabaseNoPortParams,
};
use crate::flash_gen::service_database::CreateServiceDatabaseParams;
use crate::flash_gen::services::{UpdateServiceCommandParams, UpdateServiceNetworksParams};

#[derive(Debug, Deserialize)]
pub struct CreateDatabaseDto {
    pub name: String,
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub docker_image: Option<String>,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub database_root_password: Option<String>,
    pub external_port: Option<i64>,
    pub external_grpc_port: Option<i64>,
    pub external_admin_port: Option<i64>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env_var: Option<String>,
    pub replica_sets: Option<i64>,
    pub sqld_node: Option<String>,
    pub sqld_primary_url: Option<String>,
    pub enable_namespaces: Option<i64>,
    pub network_ids: Option<Vec<String>>,
    pub detach_rustploy_network: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchDatabaseDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub docker_image: Option<String>,
    pub database_name: Option<String>,
    pub database_user: Option<String>,
    pub database_password: Option<String>,
    pub database_root_password: Option<String>,
    pub external_port: Option<i64>,
    pub external_grpc_port: Option<i64>,
    pub external_admin_port: Option<i64>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env_var: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: Option<i64>,
    pub server_id: Option<i64>,
    pub replica_sets: Option<i64>,
    pub sqld_node: Option<String>,
    pub sqld_primary_url: Option<String>,
    pub enable_namespaces: Option<i64>,
    pub network_ids: Option<Vec<String>>,
    pub detach_rustploy_network: Option<i64>,
}

pub(crate) fn serialize_json_string_vec(
    value: Option<&Vec<String>>,
) -> sqlx::Result<Option<String>> {
    value
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))
}

pub(crate) fn parse_json_string_vec(value: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(value).unwrap_or_default()
}

impl DatabaseService {
    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<DatabaseRecord>> {
        Ok(self
            .queries
            .list_databases_by_environment_full(environment_id)
            .await?
            .into_iter()
            .map(DatabaseRecord::from)
            .collect())
    }

    pub async fn get_by_id(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<DatabaseRecord> {
        let key = CacheKey::Database(id);
        let res = self
            .cache
            .try_get_with(key, async {
                let record = DatabaseRecord::from(self.queries.database_get_by_id(id).await?);
                if record.kind != kind {
                    return Err(sqlx::Error::RowNotFound);
                }
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
        let db_name = match kind {
            DatabaseKind::Postgres | DatabaseKind::Mysql | DatabaseKind::Mariadb => {
                Some(input.database_name.clone().unwrap_or_else(|| slug_value(&input.name)))
            }
            _ => None,
        };
        let db_user = match kind {
            DatabaseKind::Redis => None,
            _ => Some(input.database_user.clone().unwrap_or_else(|| "openoxide".into())),
        };
        let db_password = input
            .database_password
            .clone()
            .unwrap_or_else(random_secret);
        let root_password = input
            .database_root_password
            .clone()
            .unwrap_or_else(random_secret);
        let engine_config = build_engine_config(kind, &input, &root_password);

        let now = chrono::Utc::now().timestamp();
        let network_ids = serialize_json_string_vec(input.network_ids.as_ref())?
            .unwrap_or_else(|| "[]".into());
        let detach_rustploy_network = input.detach_rustploy_network.unwrap_or(0);

        let service_id = match input.server_id {
            None => {
                self.queries
                    .db_create_database_service_global(
                        &DbCreateDatabaseServiceGlobalParams {
                            service_type: "DATABASE".into(),
                            name: input.name.clone(),
                            app_name: app_name.clone(),
                            description: input.description.clone().unwrap_or_default(),
                            env_var: input.env_var.clone().unwrap_or_default(),
                            command: input.command.clone().unwrap_or_default(),
                            args: serialize_json_string_vec(input.args.as_ref())?
                                .unwrap_or_default(),
                            docker_image: image,
                            replicas: 1,
                            network_ids,
                            detach_rustploy_network,
                            environment_id: input.environment_id,
                            created_at: now,
                            updated_at: now,
                        },
                    )
                    .await?
            }
            Some(server_id) => {
                self.queries
                    .db_create_database_service_for_server(
                        &DbCreateDatabaseServiceForServerParams {
                            service_type: "DATABASE".into(),
                            name: input.name.clone(),
                            app_name: app_name.clone(),
                            description: input.description.clone().unwrap_or_default(),
                            env_var: input.env_var.clone().unwrap_or_default(),
                            command: input.command.clone().unwrap_or_default(),
                            args: serialize_json_string_vec(input.args.as_ref())?
                                .unwrap_or_default(),
                            docker_image: image,
                            replicas: 1,
                            network_ids,
                            detach_rustploy_network,
                            environment_id: input.environment_id,
                            server_id,
                            created_at: now,
                            updated_at: now,
                        },
                    )
                    .await?
            }
        };

        let db_type = db_type_value(kind);
        match input.external_port {
            Some(external_port) => {
                self.queries
                    .create_service_database(&CreateServiceDatabaseParams {
                        service_id,
                        db_type,
                        external_port,
                        db_name: db_name.clone().unwrap_or_default(),
                        db_user: db_user.clone().unwrap_or_default(),
                        db_password,
                        engine_config,
                    })
                    .await?;
            }
            None => {
                self.queries
                    .db_create_service_database_no_port(
                        &DbCreateServiceDatabaseNoPortParams {
                            service_id,
                            db_type,
                            db_name: db_name.unwrap_or_default(),
                            db_user: db_user.unwrap_or_default(),
                            db_password,
                            engine_config,
                        },
                    )
                    .await?;
            }
        }

        self.get_by_id(kind, service_id).await
    }

    pub async fn patch(
        &self,
        kind: DatabaseKind,
        id: i64,
        input: PatchDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        let current = self.get_by_id(kind, id).await?;

        let name = input.name.as_deref().or(Some(current.name.as_str()));
        let description = input
            .description
            .as_deref()
            .or(current.description.as_deref());
        let command = input.command.as_deref().or(current.command.as_deref());
        let args = serialize_json_string_vec(input.args.as_ref())?
            .or(current.args.clone());
        let env_var = input.env_var.as_deref().or(current.env_var.as_deref());
        let memory_reservation = input
            .memory_reservation
            .as_deref()
            .or(current.memory_reservation.as_deref());
        let memory_limit = input
            .memory_limit
            .as_deref()
            .or(current.memory_limit.as_deref());
        let cpu_reservation = input
            .cpu_reservation
            .as_deref()
            .or(current.cpu_reservation.as_deref());
        let cpu_limit = input.cpu_limit.as_deref().or(current.cpu_limit.as_deref());
        let replicas = input.replicas.unwrap_or(current.replicas);
        let server_id = input.server_id.or(current.server_id);
        let network_ids = serialize_json_string_vec(input.network_ids.as_ref())?
            .unwrap_or_else(|| current.network_ids.clone());
        let detach_rustploy_network = input
            .detach_rustploy_network
            .unwrap_or(current.detach_rustploy_network);
        let external_port = input.external_port.or(current.external_port);

        update_service_identity(&self.queries, id, name, description).await?;
        if let Some(docker_image) = input.docker_image.as_deref() {
            self.queries
                .update_service_docker_image(docker_image, id)
                .await?;
        }
        self.queries
            .update_service_command(&UpdateServiceCommandParams {
                command: command.unwrap_or_default().to_string(),
                args: args.clone().unwrap_or_default(),
                id,
            })
            .await?;
        self.queries
            .update_service_env_var(env_var.unwrap_or_default(), id)
            .await?;
        if let Some(value) = memory_reservation {
            self.queries
                .db_update_service_memory_reservation(value, id)
                .await?;
        }
        if let Some(value) = memory_limit {
            self.queries
                .db_update_service_memory_limit(value, id)
                .await?;
        }
        if let Some(value) = cpu_reservation {
            self.queries
                .db_update_service_cpu_reservation(value, id)
                .await?;
        }
        if let Some(value) = cpu_limit {
            self.queries.db_update_service_cpu_limit(value, id).await?;
        }
        self.queries.db_update_service_replicas(replicas, id).await?;
        if let Some(server_id) = server_id {
            self.queries.update_service_server(server_id, id).await?;
        }
        self.queries
            .update_service_networks(&UpdateServiceNetworksParams {
                network_ids,
                detach_rustploy_network,
                id,
            })
            .await?;

        if let Some(value) = input.database_name.as_deref() {
            self.queries.db_update_database_name(value, id).await?;
        }
        if let Some(value) = input.database_user.as_deref() {
            self.queries.db_update_database_user(value, id).await?;
        }
        if let Some(value) = input.database_password.as_deref() {
            self.queries.db_update_database_password(value, id).await?;
        }
        if let Some(external_port) = external_port {
            self.queries
                .update_database_external_port(external_port, id)
                .await?;
        }
        patch_engine_config(&self.queries, kind, id, &input).await?;

        self.cache.invalidate(&CacheKey::Database(id)).await;
        let record = self.get_by_id(kind, id).await?;

        if let Some(new_password) = &input.database_password {
            if !new_password.is_empty() {
                let pwd = new_password.clone();
                let app_name = record.app_name.clone();
                let server_id = record.server_id;
                let user_name = record
                    .database_user
                    .clone()
                    .unwrap_or_else(|| "openoxide".to_string());
                tokio::spawn(async move {
                    let Ok(executor) =
                        crate::services::compose::remote::agent_executor_for(server_id).await
                    else {
                        tracing::warn!(database = %app_name, "could not resolve Agent for password update");
                        return;
                    };
                    let docker = crate::utils::docker::DockerCli::from_executor(executor);

                    let containers = docker
                        .containers()
                        .ps()
                        .all()
                        .list()
                        .await
                        .unwrap_or_default();
                    let target_id = containers
                        .iter()
                        .find(|c| {
                            let name = c.names.trim_start_matches('/').to_lowercase();
                            name.starts_with(&format!("{}_db", app_name).to_lowercase())
                                || name.contains(&app_name.to_lowercase())
                        })
                        .map(|c| c.id.clone())
                        .unwrap_or_else(|| format!("{}_db", app_name));

                    let escaped_pwd = pwd.replace('\'', "''");
                    match kind {
                        DatabaseKind::Postgres => {
                            let sql = format!(
                                "ALTER USER \"{}\" WITH PASSWORD '{}';",
                                user_name, escaped_pwd
                            );
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["psql", "-U", &user_name, "-d", "postgres", "-c", &sql])
                                .await;
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["psql", "-U", "postgres", "-c", &sql])
                                .await;
                        }
                        DatabaseKind::Mysql | DatabaseKind::Mariadb => {
                            let sql = format!(
                                "ALTER USER '{}'@'%' IDENTIFIED BY '{}'; FLUSH PRIVILEGES;",
                                user_name, escaped_pwd
                            );
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["mysql", "-u", "root", "-e", &sql])
                                .await;
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["mariadb", "-u", "root", "-e", &sql])
                                .await;
                        }
                        DatabaseKind::Redis => {
                            // Apply password in live Redis server and rewrite configuration
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["redis-cli", "CONFIG", "SET", "requirepass", &pwd])
                                .await;
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["redis-cli", "-a", &pwd, "CONFIG", "REWRITE"])
                                .await;
                        }
                        DatabaseKind::Mongo => {
                            let script = format!(
                                "try {{ db.getSiblingDB('admin').changeUserPassword('{}', '{}'); }} catch(e) {{ db.changeUserPassword('{}', '{}'); }}",
                                user_name, escaped_pwd, user_name, escaped_pwd
                            );
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["mongosh", "--eval", &script])
                                .await;
                            let _ = docker
                                .containers()
                                .exec(&target_id)
                                .run(["mongo", "--eval", &script])
                                .await;
                        }
                        _ => {}
                    }
                });
            }
        }

        let needs_redeploy = input.external_port.is_some()
            || input.external_grpc_port.is_some()
            || input.external_admin_port.is_some()
            || input.docker_image.is_some()
            || input.database_password.is_some()
            || input.database_root_password.is_some()
            || input.database_user.is_some()
            || input.database_name.is_some()
            || input.replica_sets.is_some()
            || input.sqld_node.is_some()
            || input.sqld_primary_url.is_some()
            || input.enable_namespaces.is_some()
            || input.command.is_some()
            || input.args.is_some()
            || input.env_var.is_some()
            || input.memory_limit.is_some()
            || input.memory_reservation.is_some()
            || input.cpu_limit.is_some()
            || input.cpu_reservation.is_some()
            || input.replicas.is_some()
            || input.network_ids.is_some()
            || input.detach_rustploy_network.is_some()
            || input.server_id.is_some();

        if needs_redeploy {
            tokio::spawn(async move {
                if let Ok(svc) = auto_di::resolve::<DatabaseService>().await {
                    let _ = svc.run_operation(kind, id, DatabaseOperation::Deploy).await;
                }
            });
        }

        Ok(record)
    }

    pub async fn delete(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.get_by_id(kind, id).await?;
        let dependencies = dependency_counts(&self.queries, id).await?;
        if dependencies.blocks_delete() {
            return Err(sqlx::Error::Protocol(format!(
                "database has active dependencies: active_deployments={}, enabled_backups={}",
                dependencies.active_deployments, dependencies.enabled_backups
            )));
        }
        // ON DELETE CASCADE removes the service_database row (and linked
        // deployments) alongside the services row.
        self.queries.delete_service(id).await?;
        self.cache.invalidate(&CacheKey::Database(id)).await;
        Ok(())
    }

    pub async fn dependencies(
        &self,
        kind: DatabaseKind,
        id: i64,
    ) -> sqlx::Result<ResourceDependencyCounts> {
        self.get_by_id(kind, id).await?;
        dependency_counts(&self.queries, id).await
    }
}
