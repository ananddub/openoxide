use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::dto::database_network::DatabaseNetworkResponseDto;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpdateApplicationNetworksDto {
    pub network_ids: Vec<i64>,
    pub detach_rustploy_network: bool,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationNetworksResponseDto {
    pub application_id: i64,
    pub detach_rustploy_network: bool,
    pub networks: Vec<DatabaseNetworkResponseDto>,
}
