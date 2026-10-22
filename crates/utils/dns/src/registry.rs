use reqwest::Client;

use crate::config::DnsProviderKind;
use crate::error::DnsError;
use crate::provider::DnsProvider;
use crate::providers::{Cloudflare, Hostinger, Infomaniak, Ovh, Porkbun, Route53, Spaceship};

/// Builds the client for one provider kind.
///
/// The panel calls this once per request with the shared HTTP client; what the provider
/// needs to know about the row (its credentials) travels with each method call as a
/// [`ProviderConfig`](crate::ProviderConfig).
pub fn open(kind: DnsProviderKind, client: &Client) -> Result<Box<dyn DnsProvider>, DnsError> {
    let provider: Box<dyn DnsProvider> = match kind {
        DnsProviderKind::Cloudflare => Box::new(Cloudflare::new(client)),
        DnsProviderKind::Route53 => Box::new(Route53::new()),
        DnsProviderKind::Hostinger => Box::new(Hostinger::new(client)),
        DnsProviderKind::Porkbun => Box::new(Porkbun::new(client)),
        DnsProviderKind::Infomaniak => Box::new(Infomaniak::new(client)),
        DnsProviderKind::Ovh => Box::new(Ovh::new(client)),
        DnsProviderKind::Spaceship => Box::new(Spaceship::new(client)),
    };
    Ok(provider)
}
