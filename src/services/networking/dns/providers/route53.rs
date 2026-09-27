use super::super::{
    DnsRecordDto, DnsService, DnsServiceError, DnsTestResultDto, DnsZoneDto,
    extract_credential_field,
};
use aws_credential_types::Credentials;
use aws_sdk_route53::{Client as Route53Client, types::RrType};

impl DnsService {
    pub(in crate::services::networking::dns) fn route53_client(
        credentials_json: &str,
    ) -> Result<Route53Client, DnsServiceError> {
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

    pub(in crate::services::networking::dns) async fn test_route53(
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

    pub(in crate::services::networking::dns) async fn list_route53_zones(
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

    pub(in crate::services::networking::dns) async fn list_route53_records(
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
                let ttl = record.ttl().map(|v| v as u32);
                // One row per value. Route53 stores an rrset as a group, but the panel
                // deletes and edits single values, so a collapsed row could not be
                // pointed at unambiguously.
                for value in record.resource_records() {
                    out.push(DnsRecordDto {
                        id: format!("{kind}:{name}:{}", value.value()),
                        zone_id: zone_id.into(),
                        record_type: kind.clone(),
                        name: name.clone(),
                        content: plain_value(&kind, value.value()),
                        ttl,
                        proxied: None,
                    });
                }
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

/// Route53 returns TXT and CAA values wrapped in double quotes; the panel shows and
/// accepts the bare string, so the quotes are stripped on the way out and re-added by
/// `api_value` on the way in.
pub(in crate::services::networking::dns) fn plain_value(kind: &str, value: &str) -> String {
    if matches!(kind, "TXT" | "CAA") {
        let trimmed = value.trim();
        if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
            return trimmed[1..trimmed.len() - 1].replace("\\\"", "\"");
        }
    }
    value.to_string()
}

/// Inverse of `plain_value` — Route53 rejects an unquoted TXT/CAA value.
pub(in crate::services::networking::dns) fn api_value(kind: &str, value: &str) -> String {
    let trimmed = value.trim();
    if matches!(kind, "TXT" | "CAA")
        && !(trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"'))
    {
        return format!("\"{}\"", trimmed.replace('\\', "\\\\").replace('"', "\\\""));
    }
    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::{api_value, plain_value};

    /// The list call quotes TXT and CAA; the form shows and accepts the bare string, so
    /// a row read back has to encode to exactly what Route53 already stores.
    #[test]
    fn route53_value_quotes_round_trip() {
        for kind in ["TXT", "CAA"] {
            let stored = "\"v=spf1 -all\"";
            assert_eq!(plain_value(kind, &stored), "v=spf1 -all");
            assert_eq!(api_value(kind, "v=spf1 -all"), stored);
            assert_eq!(api_value(kind, &stored), stored);
        }
        assert_eq!(plain_value("A", "192.0.2.1"), "192.0.2.1");
        assert_eq!(api_value("A", "192.0.2.1"), "192.0.2.1");
        assert_eq!(api_value("TXT", r#"a"b"#), r#""a\"b""#);
    }
}
