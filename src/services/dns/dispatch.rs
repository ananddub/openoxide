use super::{DnsService, DnsServiceError};
use crate::api::dto::dns::{
    DnsProviderType, DnsRecordDto, DnsTestResultDto, DnsZoneDto, UpsertDnsRecordDto,
};

impl DnsService {
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

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.test_cloudflare_token(&provider.credentials_json).await
            }
            DnsProviderType::Route53 => self.test_route53(&provider.credentials_json).await,
            _ => Err(Self::unsupported_provider(p_type)),
        }
    }

    pub async fn test_credentials(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        match provider_type {
            DnsProviderType::Cloudflare => self.test_cloudflare_token(credentials_json).await,
            DnsProviderType::Route53 => self.test_route53(credentials_json).await,
            _ => Err(Self::unsupported_provider(provider_type)),
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

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.list_cloudflare_zones(&provider.credentials_json).await
            }
            DnsProviderType::Route53 => self.list_route53_zones(&provider.credentials_json).await,
            _ => Err(Self::unsupported_provider(p_type)),
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

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.list_cloudflare_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::Route53 => {
                self.list_route53_records(&provider.credentials_json, zone_id)
                    .await
            }
            _ => Err(Self::unsupported_provider(p_type)),
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

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.upsert_cloudflare_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::Route53 => {
                self.upsert_route53_record(&provider.credentials_json, &body)
                    .await
            }
            _ => Err(Self::unsupported_provider(p_type)),
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

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.delete_cloudflare_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            DnsProviderType::Route53 => {
                self.delete_route53_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            _ => Err(Self::unsupported_provider(p_type)),
        }
    }
}
