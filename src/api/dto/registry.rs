use crate::db::models::registries::Registry;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateRegistryDto {
    #[validate(length(min = 1, max = 255))]
    pub registry_name: String,
    pub image_prefix: Option<String>,
    pub username: String,
    pub password: String,
    pub registry_url: String,
    pub registry_type: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchRegistryDto {
    #[validate(length(min = 1, max = 255))]
    pub registry_name: Option<String>,
    pub image_prefix: Option<Option<String>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub registry_url: Option<String>,
    pub registry_type: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct TestRegistryDto {
    pub registry_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RegistryResponseDto {
    pub id: String,
    pub registry_name: String,
    pub image_prefix: Option<String>,
    pub username: String,
    pub password: String,
    pub registry_url: String,
    pub registry_type: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Registry> for RegistryResponseDto {
    fn from(val: Registry) -> Self {
        Self {
            id: val.id.unwrap_or_default().to_string(),
            registry_name: val.registry_name,
            image_prefix: val.image_prefix,
            username: val.username,
            password: val.password,
            registry_url: val.registry_url,
            registry_type: val.registry_type,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
