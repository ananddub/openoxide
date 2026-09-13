use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::flash_gen::models::Registries;
use crate::flash_gen::registries::RegistriesUsageReferencesRow;

pub fn map_sqlx_error(error: sqlx::Error) -> ConnectError {
    match &error {
        sqlx::Error::RowNotFound => ConnectError::not_found("registry not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        // Validation and usage-conflict failures surface as Protocol errors;
        // the client supplied the offending value either way.
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "registry database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn registry_to_resource(dto: RegistryResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id.parse::<i64>().unwrap_or(0),
        kind: "registry".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, serde::Deserialize)]
pub struct RegistryTagsQuery {
    pub repository: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct TestRegistryDto {
    /// Saved-registry test: only `id` is sent and the stored credentials are
    /// used. Raw test (create dialog): url/username/password, nothing stored.
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub registry_url: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct RotateRegistryCredentialsDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RegistryRepositoriesDto {
    pub repositories: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RegistryTagsDto {
    pub repository: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RegistryUsageDto {
    pub resource_id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub usage_kind: String,
}

impl From<RegistriesUsageReferencesRow> for RegistryUsageDto {
    fn from(value: RegistriesUsageReferencesRow) -> Self {
        Self {
            resource_id: value.resource_id.map(|id| id.to_string()).unwrap_or_default(),
            resource_name: value.resource_name,
            resource_type: value.resource_type,
            usage_kind: value.usage_kind,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
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

impl From<Registries> for RegistryResponseDto {
    fn from(val: Registries) -> Self {
        Self {
            id: val.id.to_string(),
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
