use super::{DnsProviderType, DnsService, DnsServiceError};

impl DnsService {
    pub(super) fn parse_provider_type(value: &str) -> Result<DnsProviderType, DnsServiceError> {
        DnsProviderType::from_code(value).ok_or_else(|| {
            DnsServiceError::ProviderError(format!("Unknown DNS provider type '{value}'"))
        })
    }

    /// Parsing the row's credentials through the provider's own config type is the
    /// validation: the required fields, their names and the values they accept are declared
    /// once, in `crates/dns`, and the create form collects exactly that list.
    pub(super) fn validate_provider_config(
        provider_type: DnsProviderType,
        credentials_json: &str,
    ) -> Result<(), DnsServiceError> {
        dns::ProviderConfig::from_json(provider_type, credentials_json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wire_codes_are_the_supported_set() {
        for kind in DnsProviderType::ALL {
            assert_eq!(DnsProviderType::from_code(kind.as_str()), Some(kind));
        }
        // A row from the old 62-provider list must not resolve to anything.
        assert_eq!(DnsProviderType::from_code("HETZNER"), None);
        assert_eq!(DnsProviderType::from_code("digitalocean"), None);
    }

    #[test]
    fn a_missing_required_field_is_named() {
        let error =
            DnsService::validate_provider_config(DnsProviderType::Porkbun, r#"{"apiKey":"pk-1"}"#)
                .unwrap_err()
                .to_string();
        assert!(error.contains("secretApiKey"), "{error}");
        assert!(
            DnsService::validate_provider_config(
                DnsProviderType::Porkbun,
                r#"{"apiKey":"pk-1","secretApiKey":"sk-1"}"#,
            )
            .is_ok()
        );
    }

    #[test]
    fn an_optional_field_still_has_to_be_understood() {
        // OVH's endpoint picks the host, so an invented one cannot be saved to be found later
        // by a request that never reaches it.
        let error = DnsService::validate_provider_config(
            DnsProviderType::Ovh,
            r#"{"endpoint":"ovh-xx","applicationKey":"a","applicationSecret":"b","consumerKey":"c"}"#,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("ovh-xx"), "{error}");
    }
}
