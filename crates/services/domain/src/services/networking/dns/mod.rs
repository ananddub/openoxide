use crate::db::queries::FlashQueries;
use crate::flash_gen::{
    dns_providers::{DnsProvidersCreateParams, DnsProvidersUpdateParams},
    models::DnsProviders,
};
use auto_di::singleton;
use dns::DnsError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

mod dispatch;
mod validation;

#[cfg(test)]
mod service_tests;

/// The panel's provider list *is* the crate's: one enum owns the wire code stored in
/// `dns_providers.provider_type`, the credential schema every form and validation step reads,
/// and the client that serves the row. Keeping it here rather than in a second list is what
/// stops the database, the proto enum and the web catalog from drifting apart again.
pub type DnsProviderType = dns::DnsProviderKind;

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsProviderDto {
    pub id: i64,
    pub name: String,
    pub provider_type: DnsProviderType,
    pub credentials_json: String,
    pub organization_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDnsProviderDto {
    pub name: String,
    pub provider_type: DnsProviderType,
    pub credentials_json: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDnsProviderDto {
    pub name: Option<String>,
    pub provider_type: Option<DnsProviderType>,
    pub credentials_json: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DnsTestResultDto {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsZoneDto {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsRecordDto {
    pub id: String,
    pub zone_id: String,
    pub record_type: String, // "A", "AAAA", "CNAME", "TXT", "MX"
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertDnsRecordDto {
    pub zone_id: String,
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub ttl: Option<u32>,
    pub proxied: Option<bool>,
}

/// One row the UI wants changed. Every provider keys rows differently — a Cloudflare
/// record id, a Route53 `type:name` pair, an rrset id or just name+type+value — so the
/// whole identity travels with the request and each path uses the strongest hint it
/// understands. Matching on a bare name is never enough: one name holds several types
/// and one type holds several values.
#[derive(Debug, Clone, Default)]
pub struct DnsRecordTargetDto {
    pub zone_id: String,
    pub id: String,
    pub name: String,
    pub record_type: String,
    pub content: String,
}

#[derive(Debug)]
pub struct UpdateDnsRecordDto {
    pub target: DnsRecordTargetDto,
    pub next: UpsertDnsRecordDto,
}

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

/// A provider or credential failure keeps the crate's own words, which the panel shows
/// verbatim; the handler maps it to `invalid_argument` like any other `ProviderError`.
impl From<DnsError> for DnsServiceError {
    fn from(error: DnsError) -> Self {
        Self::ProviderError(error.to_string())
    }
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
        providers
            .into_iter()
            .map(Self::into_dto)
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_provider(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let provider = match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => provider,
            Ok(_) | Err(sqlx::Error::RowNotFound) => return Err(DnsServiceError::NotFound),
            Err(error) => return Err(error.into()),
        };
        Self::into_dto(provider)
    }

    pub async fn create_provider(
        &self,
        organization_id: i64,
        body: CreateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        Self::validate_provider_config(body.provider_type, &body.credentials_json)?;
        let now = chrono::Utc::now().timestamp();

        self.queries
            .dns_providers_create(&DnsProvidersCreateParams {
                name: body.name,
                provider_type: body.provider_type.as_str().to_string(),
                credentials_json: body.credentials_json,
                organization_id,
                created_at: now,
                updated_at: now,
            })
            .await?;
        let id = self.queries.dns_providers_get_last_insert_rowid().await?;
        let created = self.queries.dns_providers_get_by_id(id).await?;
        Self::into_dto(created)
    }

    pub async fn update_provider(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateDnsProviderDto,
    ) -> Result<DnsProviderDto, DnsServiceError> {
        let mut existing = match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => provider,
            Ok(_) | Err(sqlx::Error::RowNotFound) => return Err(DnsServiceError::NotFound),
            Err(error) => return Err(error.into()),
        };

        if let Some(name) = body.name {
            existing.name = name;
        }
        if let Some(ptype) = body.provider_type {
            existing.provider_type = ptype.as_str().to_string();
        }
        if let Some(creds) = body.credentials_json {
            let merged = merge_masked_credentials(&existing.credentials_json, &creds)?;
            let provider_type = Self::parse_provider_type(&existing.provider_type)?;
            Self::validate_provider_config(provider_type, &merged)?;
            existing.credentials_json = merged;
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
        Self::into_dto(existing)
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
    ) -> Result<DnsProviders, DnsServiceError> {
        match self.queries.dns_providers_get_by_id(id).await {
            Ok(provider) if provider.organization_id == organization_id => Ok(provider),
            Ok(_) | Err(sqlx::Error::RowNotFound) => Err(DnsServiceError::NotFound),
            Err(error) => Err(error.into()),
        }
    }

    /// A stored type outside the supported set is a row this build cannot serve — reporting
    /// it beats dressing it up as the first provider in the list.
    pub(crate) fn into_dto(p: DnsProviders) -> Result<DnsProviderDto, DnsServiceError> {
        let kind = Self::parse_provider_type(&p.provider_type)?;
        Ok(DnsProviderDto {
            id: p.id,
            name: p.name,
            provider_type: kind,
            credentials_json: mask_credentials(kind, &p.credentials_json),
            organization_id: p.organization_id,
            created_at: p.created_at,
            updated_at: p.updated_at,
        })
    }
}

// The edit form sends back masked secrets as-is; keep the stored value for
// every field that still carries the mask.
fn merge_masked_credentials(
    existing_json: &str,
    incoming_json: &str,
) -> Result<String, DnsServiceError> {
    let Ok(mut incoming) = serde_json::from_str::<serde_json::Value>(incoming_json) else {
        return Ok(incoming_json.to_string());
    };
    let (Some(obj), Ok(existing)) = (
        incoming.as_object_mut(),
        serde_json::from_str::<serde_json::Value>(existing_json),
    ) else {
        return Ok(incoming_json.to_string());
    };
    for (key, value) in obj.iter_mut() {
        if value.as_str() == Some(DNS_SECRET_MASK) {
            match existing
                .get(key)
                .filter(|v| v.as_str() != Some(DNS_SECRET_MASK))
            {
                Some(stored) => *value = stored.clone(),
                None => {
                    return Err(DnsServiceError::ProviderError(format!(
                        "DNS credentials field '{key}' must be re-entered"
                    )));
                }
            }
        }
    }
    Ok(serde_json::to_string(&incoming).unwrap_or_else(|_| incoming_json.to_string()))
}

/// The keys to hide come from the provider's own credential schema, so a secret cannot be
/// masked by one layer and left visible by another; non-secret fields (an access-key id, the
/// OVH endpoint) stay readable so an edit form can prefill them.
fn mask_credentials(kind: DnsProviderType, json_str: &str) -> String {
    if let Ok(mut val) = serde_json::from_str::<serde_json::Value>(json_str)
        && let Some(obj) = val.as_object_mut()
    {
        for key in kind.secret_keys() {
            if obj.contains_key(key) {
                obj.insert(
                    key.to_string(),
                    serde_json::Value::String(DNS_SECRET_MASK.to_string()),
                );
            }
        }
        return serde_json::to_string(&val).unwrap_or_else(|_| json_str.to_string());
    }
    DNS_SECRET_MASK.to_string()
}
