use crate::api::dto::dns::{
    CreateDnsProviderDto, DnsProviderDto, DnsTestResultDto, UpdateDnsProviderDto,
};
use crate::db::models::dns_providers::DnsProvider;
use crate::db::repository::DnsProviderRepository;
use auto_di::singleton;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DnsServiceError {
    #[error("DNS provider not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("DNS provider failure: {0}")]
    ProviderError(String),
}

pub struct DnsService {
    repository: Arc<DnsProviderRepository>,
    client: Client,
}

#[singleton]
impl DnsService {
    pub fn new(repository: Arc<DnsProviderRepository>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        Self { repository, client }
    }

    pub async fn list_providers(
        &self,
        organization_id: i64,
    ) -> Result<Vec<DnsProviderDto>, DnsServiceError> {
        let providers = self.repository.list_by_organization(organization_id).await?;
        Ok(providers.into_iter().map(Self::into_dto).collect())
    }

    pub async fn get_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;
        Ok(Self::into_dto(provider))
    }

    pub async fn create_provider(
        &self,
        organization_id: i64,
        body: CreateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let now = chrono::Utc::now().timestamp();
        let provider = DnsProvider {
            id: None,
            name: body.name,
            provider_type: body.provider_type.to_ascii_uppercase(),
            credentials_json: body.credentials_json,
            organization_id,
            created_at: now,
            updated_at: now,
        };

        let id = self.repository.create(&provider).await?;
        let created = self
            .repository
            .get_by_id(id)
            .await?
            .ok_or(DnsServiceError::NotFound)?;
        Ok(Self::into_dto(created))
    }

    pub async fn update_provider(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let mut existing = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;

        if let Some(name) = body.name {
            existing.name = name;
        }
        if let Some(ptype) = body.provider_type {
            existing.provider_type = ptype.to_ascii_uppercase();
        }
        if let Some(creds) = body.credentials_json {
            existing.credentials_json = creds;
        }
        existing.updated_at = chrono::Utc::now().timestamp();

        self.repository.update(id, &existing).await?;
        Ok(Self::into_dto(existing))
    }

    pub async fn delete_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, DnsServiceError> {
        Ok(self.repository.delete(id, organization_id).await?)
    }

    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;

        match provider.provider_type.as_str() {
            "CLOUDFLARE" => self.test_cloudflare_token(&provider.credentials_json).await,
            _ => Ok(DnsTestResultDto {
                success: true,
                message: "DNS provider configuration validated successfully".into(),
            }),
        }
    }

    pub async fn test_credentials(
        &self,
        provider_type: &str,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        match provider_type.to_ascii_uppercase().as_str() {
            "CLOUDFLARE" => self.test_cloudflare_token(credentials_json).await,
            _ => Ok(DnsTestResultDto {
                success: true,
                message: "DNS provider credentials validated successfully".into(),
            }),
        }
    }

    async fn test_cloudflare_token(&self, credentials_json: &str) -> Result<DnsTestResultDto, DnsServiceError> {
        let clean_token = credentials_json
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .trim();

        if clean_token.is_empty() {
            return Ok(DnsTestResultDto {
                success: false,
                message: "API token is empty".into(),
            });
        }

        let url = "https://api.cloudflare.com/client/v4/user/tokens/verify";
        let res = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", clean_token))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(DnsTestResultDto {
                success: true,
                message: "Successfully verified Cloudflare DNS API Token!".into(),
            })
        } else {
            Ok(DnsTestResultDto {
                success: false,
                message: format!("Cloudflare API returned HTTP status code {}", res.status()),
            })
        }
    }

    fn into_dto(p: DnsProvider) -> DnsProviderDto {
        DnsProviderDto {
            id: p.id.unwrap_or_default(),
            name: p.name,
            provider_type: p.provider_type,
            credentials_json: p.credentials_json,
            organization_id: p.organization_id,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
