use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateDestinationDto {
    pub name: String,
    pub provider: String,
    pub access_key: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub organization_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct PatchDestinationDto {
    pub name: Option<String>,
    pub provider: Option<String>,
    pub access_key: Option<String>,
    pub secret_access_key: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
}
