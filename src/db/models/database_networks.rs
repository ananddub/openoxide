use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatabaseNetwork {
    pub id: i64,
    pub name: String,
    pub docker_network_name: String,
    pub description: Option<String>,
    pub external: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}
