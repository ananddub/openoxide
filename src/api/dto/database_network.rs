use crate::db::models::database_networks::DatabaseNetwork;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateDatabaseNetworkDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub docker_network_name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub external: Option<i64>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchDatabaseNetworkDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub docker_network_name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub external: Option<i64>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
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

impl From<DatabaseNetwork> for DatabaseNetworkResponseDto {
    fn from(value: DatabaseNetwork) -> Self {
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
