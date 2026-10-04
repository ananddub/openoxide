use dns::{DnsError, DnsProvider, ProviderConfig, Record, RecordInput, RecordTarget, Zone};
use reqwest::Client;

use super::{
    DnsProviderType, DnsRecordDto, DnsRecordTargetDto, DnsService, DnsServiceError,
    DnsTestResultDto, DnsZoneDto, UpdateDnsRecordDto, UpsertDnsRecordDto,
};
use crate::flash_gen::models::DnsProviders;

/// The row's stored type and credentials decide everything: which client runs and what it
/// sends. No provider is named in this file.
async fn open_for(
    row: &DnsProviders,
    http: &Client,
) -> Result<(Box<dyn DnsProvider>, ProviderConfig), DnsServiceError> {
    let kind = DnsService::parse_provider_type(&row.provider_type)?;
    let credentials = ProviderConfig::from_json(kind, &row.credentials_json)?;
    Ok((dns::open(kind, http)?, credentials))
}

/// One credential probe against a stored type, so a bad key reads as an answered test rather
/// than a failed request — the panel shows the provider's own reason in the result card.
async fn probe(
    kind: DnsProviderType,
    credentials_json: &str,
    http: &Client,
) -> Result<(), DnsError> {
    let credentials = ProviderConfig::from_json(kind, credentials_json)?;
    dns::open(kind, http)?.test_connection(&credentials).await
}

impl DnsService {
    pub async fn test_connection(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        let row = self.provider_for_organization(id, organization_id).await?;
        let kind = Self::parse_provider_type(&row.provider_type)?;
        Ok(Self::verdict(
            probe(kind, &row.credentials_json, &self.client).await,
        ))
    }

    pub async fn test_credentials(
        &self,
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        Ok(Self::verdict(
            probe(provider_type, credentials_json, &self.client).await,
        ))
    }

    fn verdict(outcome: Result<(), DnsError>) -> DnsTestResultDto {
        match outcome {
            Ok(()) => DnsTestResultDto {
                success: true,
                message: "Credentials verified successfully!".into(),
            },
            Err(error) => DnsTestResultDto {
                success: false,
                message: error.to_string(),
            },
        }
    }

    pub async fn list_zones(
        &self,
        id: i64,
        organization_id: i64,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let row = self.provider_for_organization(id, organization_id).await?;
        let (provider, credentials) = open_for(&row, &self.client).await?;
        Ok(provider
            .list_zones(&credentials)
            .await?
            .into_iter()
            .map(DnsZoneDto::from)
            .collect())
    }

    pub async fn list_records(
        &self,
        id: i64,
        organization_id: i64,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let row = self.provider_for_organization(id, organization_id).await?;
        let (provider, credentials) = open_for(&row, &self.client).await?;
        Ok(provider
            .list_records(&credentials, zone_id)
            .await?
            .into_iter()
            .map(DnsRecordDto::from)
            .collect())
    }

    pub async fn upsert_record(
        &self,
        id: i64,
        organization_id: i64,
        body: UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let row = self.provider_for_organization(id, organization_id).await?;
        let (provider, credentials) = open_for(&row, &self.client).await?;
        Ok(provider
            .create_record(&credentials, &RecordInput::from(&body))
            .await?
            .into())
    }

    pub async fn update_record(
        &self,
        id: i64,
        organization_id: i64,
        body: UpdateDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let row = self.provider_for_organization(id, organization_id).await?;
        let (provider, credentials) = open_for(&row, &self.client).await?;
        Ok(provider
            .update_record(
                &credentials,
                &RecordTarget::from(&body.target),
                &RecordInput::from(&body.next),
            )
            .await?
            .into())
    }

    pub async fn delete_record(
        &self,
        id: i64,
        organization_id: i64,
        target: DnsRecordTargetDto,
    ) -> Result<bool, DnsServiceError> {
        let row = self.provider_for_organization(id, organization_id).await?;
        let (provider, credentials) = open_for(&row, &self.client).await?;
        provider
            .delete_record(&credentials, &RecordTarget::from(&target))
            .await?;
        Ok(true)
    }
}

impl From<Zone> for DnsZoneDto {
    fn from(zone: Zone) -> Self {
        Self {
            id: zone.id,
            name: zone.name,
            status: zone.status,
        }
    }
}

impl From<Record> for DnsRecordDto {
    fn from(record: Record) -> Self {
        Self {
            id: record.id,
            zone_id: record.zone_id,
            record_type: record.record_type,
            name: record.name,
            content: record.content,
            ttl: record.ttl,
            proxied: record.proxied,
        }
    }
}

impl From<&UpsertDnsRecordDto> for RecordInput {
    fn from(body: &UpsertDnsRecordDto) -> Self {
        Self {
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: body.name.clone(),
            content: body.content.clone(),
            ttl: body.ttl,
            proxied: body.proxied,
        }
    }
}

impl From<&DnsRecordTargetDto> for RecordTarget {
    fn from(target: &DnsRecordTargetDto) -> Self {
        Self {
            zone_id: target.zone_id.clone(),
            id: target.id.clone(),
            name: target.name.clone(),
            record_type: target.record_type.clone(),
            content: target.content.clone(),
        }
    }
}
