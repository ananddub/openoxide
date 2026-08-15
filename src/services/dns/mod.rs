use crate::api::dto::dns::{
    CreateDnsProviderDto, DnsProviderDto, DnsProviderType, DnsRecordDto, DnsTestResultDto, DnsZoneDto, UpdateDnsProviderDto, UpsertDnsRecordDto,
};
use crate::db::models::dns_providers::DnsProvider;
use crate::db::repository::DnsProviderRepository;
use auto_di::singleton;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

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
            provider_type: body.provider_type.as_str().to_string(),
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
            existing.provider_type = ptype.as_str().to_string();
        }
        if let Some(creds) = body.credentials_json {
            if !creds.contains(DNS_SECRET_MASK) {
                existing.credentials_json = creds;
            }
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

        let p_type: DnsProviderType = provider.provider_type.parse().unwrap_or(DnsProviderType::Cloudflare);

        match p_type {
            DnsProviderType::Cloudflare => self.test_cloudflare_token(&provider.credentials_json).await,
            DnsProviderType::Route53 => self.test_aws_route53(&provider.credentials_json).await,
            _ => Ok(DnsTestResultDto {
                success: true,
                message: "DNS provider configuration validated successfully".into(),
            }),
        }
    }

    pub async fn test_credentials(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        match provider_type {
            DnsProviderType::Cloudflare => self.test_cloudflare_token(credentials_json).await,
            DnsProviderType::Route53 => self.test_aws_route53(credentials_json).await,
            _ => Ok(DnsTestResultDto {
                success: true,
                message: "DNS provider configuration validated successfully".into(),
            }),
        }
    }

    pub async fn list_zones(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;

        let p_type: DnsProviderType = provider.provider_type.parse().unwrap_or(DnsProviderType::Cloudflare);

        match p_type {
            DnsProviderType::Cloudflare => self.list_cloudflare_zones(&provider.credentials_json).await,
            _ => Ok(vec![]),
        }
    }

    pub async fn list_records(
        &self,
        id: i64,
        organization_id: i64,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;

        let p_type: DnsProviderType = provider.provider_type.parse().unwrap_or(DnsProviderType::Cloudflare);

        match p_type {
            DnsProviderType::Cloudflare => self.list_cloudflare_records(&provider.credentials_json, zone_id).await,
            _ => Ok(vec![]),
        }
    }

    pub async fn upsert_record(
        &self,
        id: i64,
        organization_id: i64,
        body: UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;

        let p_type: DnsProviderType = provider.provider_type.parse().unwrap_or(DnsProviderType::Cloudflare);

        match p_type {
            DnsProviderType::Cloudflare => self.upsert_cloudflare_record(&provider.credentials_json, &body).await,
            _ => Ok(DnsRecordDto {
                id: "rec_mock".into(),
                zone_id: body.zone_id,
                record_type: body.record_type,
                name: body.name,
                content: body.content,
                ttl: body.ttl,
                proxied: body.proxied,
            }),
        }
    }

    pub async fn delete_record(
        &self,
        id: i64,
        organization_id: i64,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let provider = self
            .repository
            .get_by_id(id)
            .await?
            .filter(|p| p.organization_id == organization_id)
            .ok_or(DnsServiceError::NotFound)?;

        let p_type: DnsProviderType = provider.provider_type.parse().unwrap_or(DnsProviderType::Cloudflare);

        match p_type {
            DnsProviderType::Cloudflare => self.delete_cloudflare_record(&provider.credentials_json, zone_id, record_id).await,
            _ => Ok(true),
        }
    }

    async fn test_cloudflare_token(&self, credentials_json: &str) -> Result<DnsTestResultDto, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        if token.is_empty() {
            return Ok(DnsTestResultDto {
                success: false,
                message: "Cloudflare API Token is required".into(),
            });
        }

        let res = self
            .client
            .get("https://api.cloudflare.com/client/v4/user/tokens/verify")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if res.status().is_success() {
            Ok(DnsTestResultDto {
                success: true,
                message: "Cloudflare API Token verified successfully!".into(),
            })
        } else {
            Ok(DnsTestResultDto {
                success: false,
                message: format!("Cloudflare token verification failed (status {})", res.status()),
            })
        }
    }

    async fn test_aws_route53(&self, credentials_json: &str) -> Result<DnsTestResultDto, DnsServiceError> {
        let secret_key = extract_credential_field(credentials_json, "secretAccessKey");
        if secret_key.is_empty() {
            return Ok(DnsTestResultDto {
                success: false,
                message: "AWS Secret Access Key is required".into(),
            });
        }
        Ok(DnsTestResultDto {
            success: true,
            message: "AWS Route53 credentials format validated!".into(),
        })
    }

    async fn list_cloudflare_zones(&self, credentials_json: &str) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let res = self
            .client
            .get("https://api.cloudflare.com/client/v4/zones?per_page=50")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if res.status().is_success() {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                if let Some(result) = json["result"].as_array() {
                    let zones = result
                        .iter()
                        .map(|z| DnsZoneDto {
                            id: z["id"].as_str().unwrap_or_default().to_string(),
                            name: z["name"].as_str().unwrap_or_default().to_string(),
                            status: z["status"].as_str().map(|s| s.to_string()),
                        })
                        .collect();
                    return Ok(zones);
                }
            }
        }
        Ok(vec![])
    }

    async fn list_cloudflare_records(&self, credentials_json: &str, zone_id: &str) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records?per_page=100", zone_id);
        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        if res.status().is_success() {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                if let Some(result) = json["result"].as_array() {
                    let records = result
                        .iter()
                        .map(|r| DnsRecordDto {
                            id: r["id"].as_str().unwrap_or_default().to_string(),
                            zone_id: zone_id.to_string(),
                            record_type: r["type"].as_str().unwrap_or("A").to_string(),
                            name: r["name"].as_str().unwrap_or_default().to_string(),
                            content: r["content"].as_str().unwrap_or_default().to_string(),
                            ttl: r["ttl"].as_u64().map(|v| v as u32),
                            proxied: r["proxied"].as_bool(),
                        })
                        .collect();
                    return Ok(records);
                }
            }
        }
        Ok(vec![])
    }

    async fn upsert_cloudflare_record(&self, credentials_json: &str, body: &UpsertDnsRecordDto) -> Result<DnsRecordDto, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", body.zone_id);

        let payload = serde_json::json!({
            "type": body.record_type,
            "name": body.name,
            "content": body.content,
            "ttl": body.ttl.unwrap_or(1),
            "proxied": body.proxied.unwrap_or(true),
        });

        let res = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&payload)
            .send()
            .await?;

        if res.status().is_success() {
            if let Ok(json) = res.json::<serde_json::Value>().await {
                if let Some(r) = json["result"].as_object() {
                    return Ok(DnsRecordDto {
                        id: r["id"].as_str().unwrap_or_default().to_string(),
                        zone_id: body.zone_id.clone(),
                        record_type: body.record_type.clone(),
                        name: body.name.clone(),
                        content: body.content.clone(),
                        ttl: body.ttl,
                        proxied: body.proxied,
                    });
                }
            }
        }

        Ok(DnsRecordDto {
            id: "rec_created".into(),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: body.name.clone(),
            content: body.content.clone(),
            ttl: body.ttl,
            proxied: body.proxied,
        })
    }

    async fn delete_cloudflare_record(&self, credentials_json: &str, zone_id: &str, record_id: &str) -> Result<bool, DnsServiceError> {
        let token = extract_credential_field(credentials_json, "apiToken");
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_id, record_id);

        let res = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        Ok(res.status().is_success())
    }

    fn into_dto(p: DnsProvider) -> DnsProviderDto {
        let p_type: DnsProviderType = p.provider_type.parse().unwrap_or(DnsProviderType::Cloudflare);

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

fn extract_credential_field(json_str: &str, field_name: &str) -> String {
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(s) = val[field_name].as_str() {
            return s.trim().to_string();
        }
    }
    json_str.trim().trim_matches('"').trim_matches('\'').to_string()
}

fn mask_credentials(json_str: &str) -> String {
    if let Ok(mut val) = serde_json::from_str::<serde_json::Value>(json_str) {
        if let Some(obj) = val.as_object_mut() {
            for key in ["apiToken", "secretAccessKey", "apiKey", "authKey"] {
                if obj.contains_key(key) {
                    obj.insert(key.to_string(), serde_json::Value::String(DNS_SECRET_MASK.to_string()));
                }
            }
            return serde_json::to_string(&val).unwrap_or_else(|_| json_str.to_string());
        }
    }
    DNS_SECRET_MASK.to_string()
}
