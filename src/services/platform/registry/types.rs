use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateRegistryDto {
    pub registry_name: String,
    pub image_prefix: Option<String>,
    pub username: String,
    pub password: String,
    pub registry_url: String,
    pub registry_type: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchRegistryDto {
    pub registry_name: Option<String>,
    pub image_prefix: Option<Option<String>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub registry_url: Option<String>,
    pub registry_type: Option<String>,
}
