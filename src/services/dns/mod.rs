use crate::api::dto::dns::{
    CreateDnsProviderDto, DnsProviderDto, DnsProviderType, UpdateDnsProviderDto,
};
use crate::db::models::dns_providers::DnsProvider;
use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    dns_providers::{DnsProvidersCreateParams, DnsProvidersUpdateParams},
    models::DnsProviders,
};
use auto_di::singleton;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

mod cloudflare;
mod cloudflare_response;
mod dispatch;
mod route53;
mod route53_records;
mod validation;

pub const DNS_SECRET_MASK: &str = "********";

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
    queries: Arc<FlashQueries>,
    client: Client,
}

#[singleton]
impl DnsService {
    pub fn new(queries: Arc<FlashQueries>) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        Self { queries, client }
    }

    pub async fn list_providers(
        &self,
        organization_id: i64,
    ) -> Result<Vec<DnsProviderDto>, DnsServiceError> {
        let providers = self
            .queries
            .dns_providers_list_by_organization(organization_id)
            .await?;
        Ok(providers
            .into_iter()
            .map(dns_provider_from_model)
            .map(Self::into_dto)
            .collect())
    }

    pub async fn get_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let provider = match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => {
                dns_provider_from_model(provider)
            }
            Ok(_) | Err(sqlx::Error::RowNotFound) => return Err(DnsServiceError::NotFound),
            Err(error) => return Err(error.into()),
        };
        Ok(Self::into_dto(provider))
    }

    pub async fn create_provider(
        &self,
        organization_id: i64,
        body: CreateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        Self::validate_provider_config(body.provider_type, &body.credentials_json)?;
        let now = chrono::Utc::now().timestamp();
        let provider = DnsProvider {
            id: None,
            name: body.name,
            provider_type: body.provider_type.as_str().to_string(),
            credentials_json: body.credentials_json,
            organization_id,
            created_at: now,
            updated_at: now,
        };

        self.queries
            .dns_providers_create(&DnsProvidersCreateParams {
                name: provider.name.clone(),
                provider_type: provider.provider_type.clone(),
                credentials_json: provider.credentials_json.clone(),
                organization_id: provider.organization_id,
                created_at: provider.created_at,
                updated_at: provider.updated_at,
            })
            .await?;
        let id = self.queries.dns_providers_get_last_insert_rowid().await?;
        let created = dns_provider_from_model(self.queries.dns_providers_get_by_id(id).await?);
        Ok(Self::into_dto(created))
    }

    pub async fn update_provider(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let mut existing = match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => {
                dns_provider_from_model(provider)
            }
            Ok(_) | Err(sqlx::Error::RowNotFound) => return Err(DnsServiceError::NotFound),
            Err(error) => return Err(error.into()),
        };

        if let Some(name) = body.name {
            existing.name = name;
        }
        if let Some(ptype) = body.provider_type {
            Self::ensure_supported_provider(ptype)?;
            existing.provider_type = ptype.as_str().to_string();
        }
        if let Some(creds) = body.credentials_json {
            if !creds.contains(DNS_SECRET_MASK) {
                let provider_type = Self::parse_provider_type(&existing.provider_type)?;
                Self::validate_provider_config(provider_type, &creds)?;
                existing.credentials_json = creds;
            }
        }
        existing.updated_at = chrono::Utc::now().timestamp();

        self.queries
            .dns_providers_update(&DnsProvidersUpdateParams {
                name: existing.name.clone(),
                provider_type: existing.provider_type.clone(),
                credentials_json: existing.credentials_json.clone(),
                updated_at: existing.updated_at,
                id,
                organization_id,
            })
            .await?;
        Ok(Self::into_dto(existing))
    }

    pub async fn delete_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<bool, DnsServiceError> {
        match self.queries.dns_providers_delete(id, organization_id).await {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    async fn provider_for_organization(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsProvider, DnsServiceError> {
        match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => {
                Ok(dns_provider_from_model(provider))
            }
            Ok(_) | Err(sqlx::Error::RowNotFound) => Err(DnsServiceError::NotFound),
            Err(error) => Err(error.into()),
        }
    }

    fn into_dto(p: DnsProvider) -> DnsProviderDto {
        let p_type: DnsProviderType = p
            .provider_type
            .parse()
            .unwrap_or(DnsProviderType::Cloudflare);

        DnsProviderDto {
            id: p.id.unwrap_or_default(),
            name: p.name,
            provider_type: p_type,
            credentials_json: mask_credentials(&p.credentials_json),
            organization_id: p.organization_id,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

fn dns_provider_from_model(provider: DnsProviders) -> DnsProvider {
    DnsProvider {
        id: Some(provider.id),
        name: provider.name,
        provider_type: provider.provider_type,
        credentials_json: provider.credentials_json,
        organization_id: provider.organization_id,
        created_at: provider.created_at,
        updated_at: provider.updated_at,
    }
}

fn extract_credential_field(json_str: &str, field_name: &str) -> String {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(s) = val[field_name].as_str() {
            return s.trim().to_string();
        }
    }
    json_str
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn mask_credentials(json_str: &str) -> String {
    if let Ok(mut val) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(obj) = val.as_object_mut() {
            for key in ["apiToken", "secretAccessKey", "apiKey", "authKey"] {
                if obj.contains_key(key) {
                    obj.insert(
                        key.to_string(),
                        serde_json::Value::String(DNS_SECRET_MASK.to_string()),
                    );
                }
            }
            return serde_json::to_string(&val).unwrap_or_else(|_| json_str.to_string());
        }
    }
    DNS_SECRET_MASK.to_string()
}
