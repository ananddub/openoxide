use super::{
    DnsProviderType, DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto,
    UpsertDnsRecordDto,
};

impl DnsService {
    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.test_cloudflare_token(&provider.credentials_json).await
            }
            DnsProviderType::Route53 => self.test_route53(&provider.credentials_json).await,
            other => {
                self.bridge_test_credentials(other, &provider.credentials_json)
                    .await
            }
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
            other => self.bridge_test_credentials(other, credentials_json).await,
        }
    }

    pub async fn list_zones(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

        let p_type = Self::parse_provider_type(&provider.provider_type)?;

        match p_type {
            DnsProviderType::Cloudflare => {
                self.list_cloudflare_zones(&provider.credentials_json).await
            }
            DnsProviderType::Route53 => self.list_route53_zones(&provider.credentials_json).await,
            other => {
                self.bridge_list_zones(other, &provider.credentials_json)
                    .await
            }
        }
    }

    pub async fn list_records(
        &self,
        id: i64,
        organization_id: i64,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

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
            other => {
                self.bridge_list_records(other, &provider.credentials_json, zone_id)
                    .await
            }
        }
    }

    pub async fn upsert_record(
        &self,
        id: i64,
        organization_id: i64,
        body: UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

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
            other => {
                self.bridge_upsert_record(other, &provider.credentials_json, &body)
                    .await
            }
        }
    }

    pub async fn delete_record(
        &self,
        id: i64,
        organization_id: i64,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let provider = self.provider_for_organization(id, organization_id).await?;

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
            other => {
                self.bridge_delete_record(other, &provider.credentials_json, zone_id, record_id)
                    .await
            }
        }
    }
}
