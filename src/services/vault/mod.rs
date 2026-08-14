use crate::api::dto::vault::{
    CreateVaultProviderDto, UpdateVaultProviderDto, VaultProviderDto, VaultTestResultDto,
};
use crate::db::models::vault_providers::VaultProvider;
use crate::db::repository::VaultProviderRepository;
use auto_di::singleton;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

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
        let providers = self.repository.list_by_organization(organization_id).await?;
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
        let provider = VaultProvider {
            id: None,
            name: body.name,
            provider_type: body.provider_type.to_ascii_uppercase(),
            api_url: body.api_url.trim_end_matches('/').to_string(),
            auth_token: body.auth_token,
            namespace: body.namespace,
            config_json: body.config_json,
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
            existing.auth_token = auth_token;
        }
        if let Some(namespace) = body.namespace {
            existing.namespace = Some(namespace);
        }
        if let Some(config_json) = body.config_json {
            existing.config_json = Some(config_json);
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

    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<VaultTestResultDto, VaultServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(VaultServiceError::NotFound)?;

        match provider.provider_type.as_str() {
            "HASHICORP" => self.test_hashicorp(&provider).await,
            "INFISICAL" => self.test_infisical(&provider).await,
            "DOPPLER" => self.test_doppler(&provider).await,
            _ => Ok(VaultTestResultDto {
                success: true,
                message: "Provider type configured successfully".into(),
            }),
        }
    }

    async fn test_hashicorp(&self, provider: &VaultProvider) -> Result<VaultTestResultDto, VaultServiceError> {
        let url = format!("{}/v1/sys/health", provider.api_url);
        let mut req = self.client.get(&url).header("X-Vault-Token", &provider.auth_token);
        if let Some(ns) = &provider.namespace {
            req = req.header("X-Vault-Namespace", ns);
        }
        let res = req.send().await?;
        if res.status().is_success() || res.status().as_u16() == 473 || res.status().as_u16() == 429 {
            Ok(VaultTestResultDto {
                success: true,
                message: "Successfully connected to HashiCorp Vault".into(),
            })
        } else {
            Ok(VaultTestResultDto {
                success: false,
                message: format!("HashiCorp Vault returned status code {}", res.status()),
            })
        }
    }

    async fn test_infisical(&self, provider: &VaultProvider) -> Result<VaultTestResultDto, VaultServiceError> {
        let url = format!("{}/api/v1/status", provider.api_url);
        let res = self.client.get(&url).header("Authorization", format!("Bearer {}", provider.auth_token)).send().await?;
        if res.status().is_success() {
            Ok(VaultTestResultDto {
                success: true,
                message: "Successfully connected to Infisical Vault".into(),
            })
        } else {
            Ok(VaultTestResultDto {
                success: false,
                message: format!("Infisical returned status code {}", res.status()),
            })
        }
    }

    async fn test_doppler(&self, provider: &VaultProvider) -> Result<VaultTestResultDto, VaultServiceError> {
        let url = "https://api.doppler.com/v3/me";
        let res = self.client.get(url).basic_auth(&provider.auth_token, Option::<&str>::None).send().await?;
        if res.status().is_success() {
            Ok(VaultTestResultDto {
                success: true,
                message: "Successfully authenticated with Doppler".into(),
            })
        } else {
            Ok(VaultTestResultDto {
                success: false,
                message: format!("Doppler returned status code {}", res.status()),
            })
        }
    }

    fn into_dto(p: VaultProvider) -> VaultProviderDto {
        VaultProviderDto {
            id: p.id.unwrap_or_default(),
            name: p.name,
            provider_type: p.provider_type,
            api_url: p.api_url,
            auth_token: p.auth_token,
            namespace: p.namespace,
            config_json: p.config_json,
            organization_id: p.organization_id,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
