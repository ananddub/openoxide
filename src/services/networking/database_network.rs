use crate::db::queries::FlashQueries;
use crate::flash_gen::database_networks::{
    DatabaseNetworksCreateForServerParams, DatabaseNetworksCreateGlobalParams,
    DatabaseNetworksUpdateForServerParams, DatabaseNetworksUpdateGlobalParams,
};
use crate::flash_gen::models::DatabaseNetworks;
use auto_di::singleton;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateDatabaseNetworkDto {
    pub name: String,
    pub docker_network_name: String,
    pub description: Option<String>,
    pub external: Option<i64>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PatchDatabaseNetworkDto {
    pub name: Option<String>,
    pub docker_network_name: Option<String>,
    pub description: Option<String>,
    pub external: Option<i64>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseNetworkResponseDto {
    pub id: i64,
    pub name: String,
    pub docker_network_name: String,
    pub description: Option<String>,
    pub external: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<DatabaseNetworks> for DatabaseNetworkResponseDto {
    fn from(value: DatabaseNetworks) -> Self {
        Self {
            id: value.id,
            name: value.name,
            docker_network_name: value.docker_network_name,
            description: value.description,
            external: value.external,
            server_id: value.server_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NetworkDependencyCounts {
    pub applications: i64,
    pub compose_services: i64,
    pub databases: i64,
}

impl NetworkDependencyCounts {
    pub fn total(&self) -> i64 {
        self.applications + self.compose_services + self.databases
    }
}

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
                self.queries.database_networks_list_by_server(server_id).await
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
        let databases = self
            .queries
            .db_network_dependency_databases(&key)
            .await?;
        Ok(NetworkDependencyCounts {
            applications,
            compose_services,
            databases,
        })
    }
}
