use super::{DnsService, DnsServiceError};
use crate::api::dto::dns::DnsProviderType;

impl DnsService {
    pub(super) fn parse_provider_type(value: &str) -> Result<DnsProviderType, DnsServiceError> {
        value.parse().map_err(|_| {
            DnsServiceError::ProviderError(format!("Unknown DNS provider type '{value}'"))
        })
    }

    pub(super) fn ensure_supported_provider(
        provider_type: DnsProviderType,
    ) -> Result<(), DnsServiceError> {
        match provider_type {
            DnsProviderType::Cloudflare | DnsProviderType::Route53 => Ok(()),
            _ => Err(Self::unsupported_provider(provider_type)),
        }
    }

    pub(super) fn unsupported_provider(provider_type: DnsProviderType) -> DnsServiceError {
        DnsServiceError::ProviderError(format!(
            "DNS provider '{}' is not implemented",
            provider_type.as_str()
        ))
    }

    pub(super) fn validate_provider_config(
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<(), DnsServiceError> {
        Self::ensure_supported_provider(provider_type)?;
        let config: serde_json::Value =
            serde_json::from_str(credentials_json).map_err(|error| {
                DnsServiceError::ProviderError(format!("Invalid DNS credentials JSON: {error}"))
            })?;

        let required = match provider_type {
            DnsProviderType::Cloudflare => &["apiToken"][..],
            DnsProviderType::Route53 => &["accessKeyId", "secretAccessKey"][..],
            _ => unreachable!(),
        };
        for field in required {
            if config[*field]
                .as_str()
                .map(str::trim)
                .unwrap_or_default()
                .is_empty()
            {
                return Err(DnsServiceError::ProviderError(format!(
                    "DNS credentials field '{field}' is required"
                )));
            }
        }
        Ok(())
    }
}
