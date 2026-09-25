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
            DnsProviderType::Hetzner => self.test_hetzner_token(&provider.credentials_json).await,
            DnsProviderType::DigitalOcean => {
                self.test_digitalocean_token(&provider.credentials_json)
                    .await
            }
            DnsProviderType::Godaddy => {
                self.test_godaddy_credentials(&provider.credentials_json)
                    .await
            }
            DnsProviderType::Porkbun => {
                self.test_porkbun_credentials(&provider.credentials_json)
                    .await
            }
            DnsProviderType::Infomaniak => {
                self.test_infomaniak_token(&provider.credentials_json).await
            }
            DnsProviderType::Ovh => self.test_ovh_credentials(&provider.credentials_json).await,
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
            DnsProviderType::Hetzner => self.test_hetzner_token(credentials_json).await,
            DnsProviderType::DigitalOcean => self.test_digitalocean_token(credentials_json).await,
            DnsProviderType::Godaddy => self.test_godaddy_credentials(credentials_json).await,
            DnsProviderType::Porkbun => self.test_porkbun_credentials(credentials_json).await,
            DnsProviderType::Infomaniak => self.test_infomaniak_token(credentials_json).await,
            DnsProviderType::Ovh => self.test_ovh_credentials(credentials_json).await,
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
            DnsProviderType::Hetzner => self.list_hetzner_zones(&provider.credentials_json).await,
            DnsProviderType::DigitalOcean => {
                self.list_digitalocean_zones(&provider.credentials_json)
                    .await
            }
            DnsProviderType::Godaddy => self.list_godaddy_zones(&provider.credentials_json).await,
            DnsProviderType::Porkbun => self.list_porkbun_zones(&provider.credentials_json).await,
            DnsProviderType::Infomaniak => {
                self.list_infomaniak_zones(&provider.credentials_json).await
            }
            DnsProviderType::Ovh => self.list_ovh_zones(&provider.credentials_json).await,
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
            DnsProviderType::Hetzner => {
                self.list_hetzner_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::DigitalOcean => {
                self.list_digitalocean_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::Godaddy => {
                self.list_godaddy_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::Porkbun => {
                self.list_porkbun_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::Infomaniak => {
                self.list_infomaniak_records(&provider.credentials_json, zone_id)
                    .await
            }
            DnsProviderType::Ovh => {
                self.list_ovh_records(&provider.credentials_json, zone_id)
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
            DnsProviderType::Hetzner => {
                self.upsert_hetzner_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::DigitalOcean => {
                self.upsert_digitalocean_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::Godaddy => {
                self.upsert_godaddy_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::Porkbun => {
                self.upsert_porkbun_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::Infomaniak => {
                self.upsert_infomaniak_record(&provider.credentials_json, &body)
                    .await
            }
            DnsProviderType::Ovh => {
                self.upsert_ovh_record(&provider.credentials_json, &body)
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
            DnsProviderType::Hetzner => {
                self.delete_hetzner_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            DnsProviderType::DigitalOcean => {
                self.delete_digitalocean_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            DnsProviderType::Godaddy => {
                self.delete_godaddy_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            DnsProviderType::Porkbun => {
                self.delete_porkbun_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            DnsProviderType::Infomaniak => {
                self.delete_infomaniak_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            DnsProviderType::Ovh => {
                self.delete_ovh_record(&provider.credentials_json, zone_id, record_id)
                    .await
            }
            other => {
                self.bridge_delete_record(other, &provider.credentials_json, zone_id, record_id)
                    .await
            }
        }
    }
}
