use serde::{Deserialize, Serialize};

use crate::flash_gen::models::DatabaseNetworks;

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
