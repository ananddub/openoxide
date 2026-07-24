use std::fmt;

/// Represents a Traefik TLS configuration for a router.
///
/// Used to configure TLS termination on a router — either by enabling raw TLS
/// (`TlsConfig::enabled()`) or by pointing at a named cert resolver
/// (`TlsConfig::with_resolver("letsencrypt")`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsConfig {
    /// Optional cert resolver name (e.g. `"letsencrypt"`, `"cloudflare"`).
    /// When `None`, TLS is enabled without a specific resolver (manual / wildcard certs).
    pub cert_resolver: Option<String>,
}

impl TlsConfig {
    /// Enable TLS without a cert resolver (uses whatever cert Traefik has loaded for the domain).
    pub fn enabled() -> Self {
        Self {
            cert_resolver: None,
        }
    }

    /// Enable TLS with a specific cert resolver name.
    pub fn with_resolver(resolver: impl Into<String>) -> Self {
        Self {
            cert_resolver: Some(resolver.into()),
        }
    }
}

impl fmt::Display for TlsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.cert_resolver {
            Some(r) => write!(f, "certResolver={r}"),
            None => write!(f, "enabled"),
        }
    }
}

/// Represents a well-known cert resolver name used in Traefik's static config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertResolver {
    LetsEncrypt,
    Custom(String),
}

impl CertResolver {
    pub fn name(&self) -> &str {
        match self {
            CertResolver::LetsEncrypt => "letsencrypt",
            CertResolver::Custom(name) => name,
        }
    }
}

impl fmt::Display for CertResolver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<CertResolver> for TlsConfig {
    fn from(resolver: CertResolver) -> Self {
        TlsConfig::with_resolver(resolver.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enabled_has_no_resolver() {
        assert_eq!(TlsConfig::enabled().cert_resolver, None);
    }

    #[test]
    fn with_resolver_stores_name() {
        let tls = TlsConfig::with_resolver("letsencrypt");
        assert_eq!(tls.cert_resolver.as_deref(), Some("letsencrypt"));
    }

    #[test]
    fn cert_resolver_letsencrypt_name() {
        assert_eq!(CertResolver::LetsEncrypt.name(), "letsencrypt");
    }

    #[test]
    fn cert_resolver_custom_name() {
        assert_eq!(
            CertResolver::Custom("cloudflare".into()).name(),
            "cloudflare"
        );
    }

    #[test]
    fn from_cert_resolver_into_tls_config() {
        let tls: TlsConfig = CertResolver::LetsEncrypt.into();
        assert_eq!(tls.cert_resolver.as_deref(), Some("letsencrypt"));
    }
}
