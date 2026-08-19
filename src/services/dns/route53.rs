use super::{DnsService, DnsServiceError, extract_credential_field};
use crate::api::dto::dns::{DnsRecordDto, DnsTestResultDto, DnsZoneDto};
use aws_credential_types::Credentials;
use aws_sdk_route53::{Client as Route53Client, types::RrType};

impl DnsService {
    pub(super) fn route53_client(credentials_json: &str) -> Result<Route53Client, DnsServiceError> {
        let access_key = extract_credential_field(credentials_json, "accessKeyId");
        let secret_key = extract_credential_field(credentials_json, "secretAccessKey");
        if access_key.is_empty() || secret_key.is_empty() {
            return Err(DnsServiceError::ProviderError(
                "Route53 accessKeyId and secretAccessKey are required".into(),
            ));
        }
        let credentials = Credentials::new(access_key, secret_key, None, None, "rustploy-dns");
        let config = aws_sdk_route53::Config::builder()
            .region(aws_sdk_route53::config::Region::new("us-east-1"))
            .credentials_provider(credentials)
            .behavior_version_latest()
            .build();
        Ok(Route53Client::from_conf(config))
    }

    pub(super) async fn test_route53(
        &self,
        credentials_json: &str,
    ) -> Result<DnsTestResultDto, DnsServiceError> {
        Self::route53_client(credentials_json)?
            .list_hosted_zones()
            .max_items(1)
            .send()
            .await
            .map_err(|e| {
                DnsServiceError::ProviderError(format!("Route53 authentication failed: {e}"))
            })?;
        Ok(DnsTestResultDto {
            success: true,
            message: "Route53 credentials verified successfully".into(),
        })
    }

    pub(super) async fn list_route53_zones(
        &self,
        credentials_json: &str,
    ) -> Result<Vec<DnsZoneDto>, DnsServiceError> {
        let client = Self::route53_client(credentials_json)?;
        let mut zones = Vec::new();
        let mut marker: Option<String> = None;
        loop {
            let mut request = client.list_hosted_zones().max_items(100);
            if let Some(value) = marker.as_deref() {
                request = request.marker(value);
            }
            let response = request.send().await.map_err(|e| {
                DnsServiceError::ProviderError(format!("Route53 list zones failed: {e}"))
            })?;
            for zone in response.hosted_zones() {
                zones.push(DnsZoneDto {
                    id: zone.id().trim_start_matches("/hostedzone/").into(),
                    name: zone.name().trim_end_matches('.').into(),
                    status: None,
                });
            }
            marker = if response.is_truncated() {
                response.next_marker().map(str::to_string)
            } else {
                None
            };
            if marker.is_none() {
                break;
            }
        }
        Ok(zones)
    }

    pub(super) async fn list_route53_records(
        &self,
        credentials_json: &str,
        zone_id: &str,
    ) -> Result<Vec<DnsRecordDto>, DnsServiceError> {
        let client = Self::route53_client(credentials_json)?;
        let mut out = Vec::new();
        let mut next_name: Option<String> = None;
        let mut next_type: Option<RrType> = None;
        loop {
            let mut request = client.list_resource_record_sets().hosted_zone_id(zone_id);
            if let Some(name) = next_name.as_deref() {
                request = request.start_record_name(name);
            }
            if let Some(kind) = next_type.clone() {
                request = request.start_record_type(kind);
            }
            let response = request.send().await.map_err(|e| {
                DnsServiceError::ProviderError(format!("Route53 list records failed: {e}"))
            })?;
            for record in response.resource_record_sets() {
                let name = record.name().trim_end_matches('.').to_string();
                let kind = record.r#type().as_str().to_string();
                if record.resource_records().is_empty() {
                    continue;
                }
                out.push(DnsRecordDto {
                    id: format!("{kind}:{name}"),
                    zone_id: zone_id.into(),
                    record_type: kind,
                    name,
                    content: record
                        .resource_records()
                        .iter()
                        .map(|v| v.value())
                        .collect::<Vec<_>>()
                        .join(", "),
                    ttl: record.ttl().map(|v| v as u32),
                    proxied: None,
                });
            }
            next_name = response.next_record_name().map(str::to_string);
            next_type = response.next_record_type().cloned();
            if next_name.is_none() {
                break;
            }
        }
        Ok(out)
    }
}
