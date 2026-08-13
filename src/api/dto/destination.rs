use crate::db::models::destinations::Destination;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateDestinationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub organization_id: i64,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TestDestinationDto {
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchDestinationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub provider: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct DestinationResponseDto {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Destination> for DestinationResponseDto {
    fn from(val: Destination) -> Self {
        Self {
            id: val.id.unwrap_or_default(),
            name: val.name,
            provider: val.provider,
            access_key: val.access_key,
            secret_access_key: val.secret_access_key,
            bucket: val.bucket,
            region: val.region,
            endpoint: val.endpoint,
            organization_id: val.organization_id,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
