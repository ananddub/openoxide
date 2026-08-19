use super::{DnsService, DnsServiceError};
use crate::api::dto::dns::{DnsRecordDto, UpsertDnsRecordDto};
use aws_sdk_route53::types::{
    Change, ChangeAction, ChangeBatch, ResourceRecord, ResourceRecordSet, RrType,
};

impl DnsService {
    fn route53_record(body: &UpsertDnsRecordDto) -> Result<ResourceRecordSet, DnsServiceError> {
        let kind = match body.record_type.as_str() {
            "A" => RrType::A,
            "CNAME" => RrType::Cname,
            _ => {
                return Err(DnsServiceError::ProviderError(
                    "Only A and CNAME records are supported".into(),
                ));
            }
        };
        ResourceRecordSet::builder()
            .name(if body.name.ends_with('.') {
                body.name.clone()
            } else {
                format!("{}.", body.name)
            })
            .r#type(kind)
            .ttl(body.ttl.unwrap_or(300) as i64)
            .resource_records(
                ResourceRecord::builder()
                    .value(&body.content)
                    .build()
                    .map_err(|e| DnsServiceError::ProviderError(e.to_string()))?,
            )
            .build()
            .map_err(|e| DnsServiceError::ProviderError(e.to_string()))
    }

    pub(super) async fn upsert_route53_record(
        &self,
        credentials_json: &str,
        body: &UpsertDnsRecordDto,
    ) -> Result<DnsRecordDto, DnsServiceError> {
        let set = Self::route53_record(body)?;
        let change = Change::builder()
            .action(ChangeAction::Upsert)
            .resource_record_set(set)
            .build()
            .map_err(|e| DnsServiceError::ProviderError(e.to_string()))?;
        let batch = ChangeBatch::builder()
            .changes(change)
            .build()
            .map_err(|e| DnsServiceError::ProviderError(e.to_string()))?;
        Self::route53_client(credentials_json)?
            .change_resource_record_sets()
            .hosted_zone_id(&body.zone_id)
            .change_batch(batch)
            .send()
            .await
            .map_err(|e| DnsServiceError::ProviderError(format!("Route53 upsert failed: {e}")))?;
        Ok(DnsRecordDto {
            id: format!("{}:{}", body.record_type, body.name),
            zone_id: body.zone_id.clone(),
            record_type: body.record_type.clone(),
            name: body.name.clone(),
            content: body.content.clone(),
            ttl: body.ttl.or(Some(300)),
            proxied: None,
        })
    }

    pub(super) async fn delete_route53_record(
        &self,
        credentials_json: &str,
        zone_id: &str,
        record_id: &str,
    ) -> Result<bool, DnsServiceError> {
        let (kind, name) = record_id
            .split_once(':')
            .ok_or_else(|| DnsServiceError::ProviderError("Invalid Route53 record id".into()))?;
        let records = self.list_route53_records(credentials_json, zone_id).await?;
        let existing = records
            .into_iter()
            .find(|r| r.record_type == kind && r.name == name)
            .ok_or(DnsServiceError::NotFound)?;
        let body = UpsertDnsRecordDto {
            zone_id: zone_id.into(),
            record_type: existing.record_type,
            name: existing.name,
            content: existing.content,
            ttl: existing.ttl,
            proxied: None,
        };
        let set = Self::route53_record(&body)?;
        let change = Change::builder()
            .action(ChangeAction::Delete)
            .resource_record_set(set)
            .build()
            .map_err(|e| DnsServiceError::ProviderError(e.to_string()))?;
        let batch = ChangeBatch::builder()
            .changes(change)
            .build()
            .map_err(|e| DnsServiceError::ProviderError(e.to_string()))?;
        Self::route53_client(credentials_json)?
            .change_resource_record_sets()
            .hosted_zone_id(zone_id)
            .change_batch(batch)
            .send()
            .await
            .map_err(|e| DnsServiceError::ProviderError(format!("Route53 delete failed: {e}")))?;
        Ok(true)
    }
}
