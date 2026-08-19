use crate::api::dto::vault::{
    CreateVaultProviderDto, UpdateVaultProviderDto, VaultProviderAssignmentDto, VaultProviderDto,
    VaultProviderType,
};
use crate::db::models::vault_providers::VaultProvider;
use crate::db::repository::VaultProviderRepository;
use auto_di::singleton;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

mod azure;
mod config;
mod dispatch;
mod doppler;
mod hashicorp;
mod infisical;
mod providers;
mod resolution;
mod scaleway;
use config::{encode_path, field as config_field};

pub const VAULT_MASK_TOKEN: &str = "••••••••";

#[derive(Debug, Error)]
pub enum VaultServiceError {
    #[error("Vault provider not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Vault provider failure: {0}")]
    ProviderError(String),
}

pub struct VaultService {
    repository: Arc<VaultProviderRepository>,
    client: Client,
}

#[singleton]
impl VaultService {
    pub fn new(repository: Arc<VaultProviderRepository>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        Self { repository, client }
    }

    pub async fn list_providers(
        &self,
        organization_id: i64,
    ) -> Result<Vec<VaultProviderDto>, VaultServiceError> {
        let providers = self
            .repository
            .list_by_organization(organization_id)
            .await?;
        Ok(providers.into_iter().map(Self::into_dto).collect())
    }

    pub async fn get_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<VaultProviderDto, VaultServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;
        Ok(Self::into_dto(provider))
    }

    pub async fn create_provider(
        &self,
        organization_id: i64,
        body: CreateVaultProviderDto,
    ) -> Result<VaultProviderDto, VaultServiceError> {
        let now = chrono::Utc::now().timestamp();
        let config_json = Self::merge_assignments(body.config_json, &body.assignments);
        let provider = VaultProvider {
            id: None,
            name: body.name,
            provider_type: body.provider_type.as_str().to_string(),
            api_url: body.api_url.trim_end_matches('/').to_string(),
            auth_token: body.auth_token,
            namespace: body.namespace,
            config_json,
            organization_id,
            created_at: now,
            updated_at: now,
        };

        let id = self.repository.create(&provider).await?;
        let created = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(VaultServiceError::NotFound)?;
        Ok(Self::into_dto(created))
    }

    pub async fn update_provider(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateVaultProviderDto,
    ) -> Result<VaultProviderDto, VaultServiceError> {
        let mut existing = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;

        if let Some(name) = body.name {
            existing.name = name;
        }
        if let Some(api_url) = body.api_url {
            existing.api_url = api_url.trim_end_matches('/').to_string();
        }
        if let Some(auth_token) = body.auth_token {
            if auth_token != VAULT_MASK_TOKEN && !auth_token.trim().is_empty() {
                existing.auth_token = auth_token;
            }
        }
        if let Some(namespace) = body.namespace {
            existing.namespace = Some(namespace);
        }
        if let Some(config_json) = body.config_json {
            let assignments = Self::read_assignments(existing.config_json.as_deref());
            existing.config_json = Self::merge_assignments(Some(config_json), &assignments);
        }
        if let Some(assignments) = body.assignments {
            existing.config_json = Self::merge_assignments(existing.config_json, &assignments);
        }
        existing.updated_at = chrono::Utc::now().timestamp();

        self.repository.update(id, &existing).await?;
        Ok(Self::into_dto(existing))
    }

    pub async fn delete_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, VaultServiceError> {
        Ok(self.repository.delete(id, organization_id).await?)
    }

    pub async fn get_assignments(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<VaultProviderAssignmentDto>, VaultServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;
        Ok(Self::read_assignments(provider.config_json.as_deref()))
    }

    pub async fn set_assignments(
        &self,
        id: i64,
        organization_id: i64,
        assignments: Vec<VaultProviderAssignmentDto>,
    ) -> Result<Vec<VaultProviderAssignmentDto>, VaultServiceError> {
        let mut provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;
        provider.config_json = Self::merge_assignments(provider.config_json, &assignments);
        provider.updated_at = chrono::Utc::now().timestamp();
        self.repository.update(id, &provider).await?;
        Ok(assignments)
    }

    fn into_dto(p: VaultProvider) -> VaultProviderDto {
        let assignments = Self::read_assignments(p.config_json.as_deref());
        let p_type: VaultProviderType = p
            .provider_type
            .parse()
            .unwrap_or(VaultProviderType::Hashicorp);

        VaultProviderDto {
            id: p.id.unwrap_or_default(),
            name: p.name,
            provider_type: p_type,
            api_url: p.api_url,
            auth_token: if p.auth_token.is_empty() {
                String::new()
            } else {
                VAULT_MASK_TOKEN.to_string()
            },
            namespace: p.namespace,
            config_json: p.config_json,
            organization_id: p.organization_id,
            created_at: p.created_at,
            updated_at: p.updated_at,
            assignments,
        }
    }

    fn read_assignments(raw: Option<&str>) -> Vec<VaultProviderAssignmentDto> {
        raw.and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok())
            .and_then(|value| serde_json::from_value(value["_assignments"].clone()).ok())
            .unwrap_or_default()
    }

    fn merge_assignments(
        raw: Option<String>,
        assignments: &[VaultProviderAssignmentDto],
    ) -> Option<String> {
        let mut value = raw
            .as_deref()
            .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok())
            .unwrap_or_else(|| serde_json::json!({}));
        if let Some(object) = value.as_object_mut() {
            object.insert(
                "_assignments".into(),
                serde_json::to_value(assignments).unwrap_or_else(|_| serde_json::json!([])),
            );
            Some(value.to_string())
        } else {
            Some(
                serde_json::json!({ "_provider_config": value, "_assignments": assignments })
                    .to_string(),
            )
        }
    }
}
