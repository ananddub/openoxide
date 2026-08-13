use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::organization::Organization;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateOrganizationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 2_048))]
    pub logo: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub slug: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchOrganizationDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 2_048))]
    pub logo: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct OrganizationResponseDto {
    pub id: i64,
    pub name: String,
    pub logo: Option<String>,
    pub slug: String,
    pub owner_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Organization> for OrganizationResponseDto {
    fn from(value: Organization) -> Self {
        Self {
            id: value.id.expect("persisted organization must have an id"),
            name: value.name,
            logo: value.logo,
            slug: value.slug,
            owner_id: value.owner_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
