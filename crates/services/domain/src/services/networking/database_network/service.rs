use crate::db::queries::FlashQueries;
use crate::flash_gen::database_networks::{
    DatabaseNetworksCreateForServerParams, DatabaseNetworksCreateGlobalParams,
    DatabaseNetworksUpdateForServerParams, DatabaseNetworksUpdateGlobalParams,
};
use crate::flash_gen::models::DatabaseNetworks;
use auto_di::singleton;
use std::sync::Arc;

use super::types::{CreateDatabaseNetworkDto, NetworkDependencyCounts, PatchDatabaseNetworkDto};

pub struct DatabaseNetworkService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl DatabaseNetworkService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn list(&self) -> sqlx::Result<Vec<DatabaseNetworks>> {
        self.queries.database_networks_list().await
    }

    pub async fn list_by_server(
        &self,
        server_id: Option<i64>,
    ) -> sqlx::Result<Vec<DatabaseNetworks>> {
        match server_id {
            Some(server_id) => {
                self.queries
                    .database_networks_list_by_server(server_id)
                    .await
            }
            None => self.queries.database_networks_list_global().await,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<DatabaseNetworks> {
        self.queries.database_networks_get_by_id(id).await
    }

    pub async fn create(&self, input: CreateDatabaseNetworkDto) -> sqlx::Result<DatabaseNetworks> {
        let new_id = match input.server_id {
            None => {
                self.queries
                    .database_networks_create_global(&DatabaseNetworksCreateGlobalParams {
                        name: input.name,
                        docker_network_name: input.docker_network_name,
                        description: input.description.unwrap_or_default(),
                        external: input.external.unwrap_or(1),
                    })
                    .await?
            }
            Some(server_id) => {
                self.queries
                    .database_networks_create_for_server(&DatabaseNetworksCreateForServerParams {
                        name: input.name,
                        docker_network_name: input.docker_network_name,
                        description: input.description.unwrap_or_default(),
                        external: input.external.unwrap_or(1),
                        server_id,
                    })
                    .await?
            }
        };
        self.get_by_id(new_id).await
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchDatabaseNetworkDto,
    ) -> sqlx::Result<DatabaseNetworks> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let docker_network_name = input
            .docker_network_name
            .unwrap_or(current.docker_network_name);
        let description = input.description.or(current.description);
        let external = input.external.unwrap_or(current.external);
        let server_id = input.server_id.or(current.server_id);
        match server_id {
            None => {
                self.queries
                    .database_networks_update_global(&DatabaseNetworksUpdateGlobalParams {
                        name,
                        docker_network_name,
                        description: description.unwrap_or_default(),
                        external,
                        id,
                    })
                    .await?;
            }
            Some(server_id) => {
                self.queries
                    .database_networks_update_for_server(&DatabaseNetworksUpdateForServerParams {
                        name,
                        docker_network_name,
                        description: description.unwrap_or_default(),
                        external,
                        server_id,
                        id,
                    })
                    .await?;
            }
        }
        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let dependencies = self.dependencies(id).await?;
        if dependencies.total() > 0 {
            return Err(sqlx::Error::Protocol(format!(
                "database network is in use: applications={}, compose_services={}, databases={}",
                dependencies.applications, dependencies.compose_services, dependencies.databases
            )));
        }
        self.queries.database_networks_delete(id).await
    }

    pub async fn dependencies(&self, id: i64) -> sqlx::Result<NetworkDependencyCounts> {
        self.get_by_id(id).await?;
        // Bind the network id as TEXT — the same comparison the old
        // repository used against the JSON array of network-id strings.
        let key = id.to_string();
        let applications = self
            .queries
            .db_network_dependency_applications(&key)
            .await?;
        let compose_services = self
            .queries
            .db_network_dependency_compose_services(&key)
            .await?;
        let databases = self.queries.db_network_dependency_databases(&key).await?;
        Ok(NetworkDependencyCounts {
            applications,
            compose_services,
            databases,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fixture;

    async fn service(tag: &str) -> (DatabaseNetworkService, sqlx::SqlitePool) {
        let (queries, pool) = fixture(tag).await;
        (DatabaseNetworkService { queries }, pool)
    }

    fn network(name: &str, server_id: Option<i64>) -> CreateDatabaseNetworkDto {
        CreateDatabaseNetworkDto {
            name: name.into(),
            docker_network_name: format!("ox-{name}"),
            description: None,
            external: None,
            server_id,
        }
    }

    async fn services_for(pool: &sqlx::SqlitePool, network_id: i64) {
        sqlx::raw_sql(
            "INSERT INTO organization (name, slug, owner_id, created_at, updated_at)
                 VALUES ('org', 'org', 1, 0, 0);
             INSERT INTO projects (name, env_var, organization_id, created_at, updated_at)
                 VALUES ('site', '', 1, 0, 0);
             INSERT INTO environments (name, env_var, is_default, project_id, created_at, updated_at)
                 VALUES ('production', '', 1, 1, 0, 0);",
        )
        .execute(pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO services
                 (id, service_type, name, app_name, app_status, replicas, network_ids,
                  detach_rustploy_network, environment_id, created_at, updated_at)
                 VALUES (1, 'APPLICATION', 'app', 'app', 'IDLE', 1, json_array(?), 0, 1, 0, 0)",
        )
        .bind(network_id.to_string())
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn create_defaults_external_and_scopes_global_vs_server_rows() {
        let (service, _pool) = service("network-create").await;

        let global = service.create(network("shared", None)).await.unwrap();
        assert_eq!(global.server_id, None);
        assert_eq!(global.external, 1, "external defaults to true");
        assert_eq!(global.description.as_deref(), Some(""));
        assert!(global.created_at > 0);

        let scoped = service.create(network("edge", Some(5))).await.unwrap();
        assert_eq!(scoped.server_id, Some(5));

        assert_eq!(service.list().await.unwrap().len(), 2);
        let globals = service.list_by_server(None).await.unwrap();
        assert_eq!(globals.len(), 1);
        assert_eq!(globals[0].name, "shared");
        let ones = service.list_by_server(Some(5)).await.unwrap();
        assert_eq!(ones.len(), 1);
        assert_eq!(ones[0].name, "edge");
    }

    #[tokio::test]
    async fn patch_keeps_unsent_fields_and_scope() {
        let (service, _pool) = service("network-patch").await;
        let scoped = service.create(network("edge", Some(5))).await.unwrap();

        let patched = service
            .patch(
                scoped.id,
                PatchDatabaseNetworkDto {
                    name: Some("edge-2".into()),
                    docker_network_name: None,
                    description: Some("changed".into()),
                    external: Some(0),
                    server_id: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(patched.name, "edge-2");
        assert_eq!(patched.docker_network_name, "ox-edge");
        assert_eq!(patched.description.as_deref(), Some("changed"));
        assert_eq!(patched.external, 0);
        assert_eq!(
            patched.server_id,
            Some(5),
            "an absent server_id must not move the network back to global"
        );
        assert!(patched.updated_at >= scoped.updated_at);
    }

    #[tokio::test]
    async fn delete_removes_an_unused_network() {
        let (service, _pool) = service("network-delete").await;
        let created = service.create(network("shared", None)).await.unwrap();

        service.delete(created.id).await.unwrap();

        assert!(service.get_by_id(created.id).await.is_err());
        assert!(service.list().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn delete_refuses_a_network_referenced_by_a_service() {
        let (service, pool) = service("network-used").await;
        let created = service.create(network("shared", None)).await.unwrap();
        services_for(&pool, created.id).await;

        let dependencies = service.dependencies(created.id).await.unwrap();
        assert_eq!(dependencies.applications, 1);
        assert_eq!(dependencies.compose_services, 0);
        assert_eq!(dependencies.databases, 0);

        let error = service.delete(created.id).await.unwrap_err();
        assert!(
            error.to_string().contains("database network is in use"),
            "{error}"
        );
        assert!(
            service.get_by_id(created.id).await.is_ok(),
            "a refused delete must keep the network"
        );
    }
}
