use thiserror::Error;

#[derive(Debug, Error)]
pub enum DnsError {
    #[error("unsupported DNS provider '{0}'")]
    UnsupportedProvider(String),

    #[error("invalid DNS credentials: {0}")]
    Credentials(String),

    /// A provider rejected the request. `message` is already prefixed with what the
    /// provider said, so the panel shows it verbatim.
    #[error("{message}")]
    Provider { message: String },

    #[error("DNS provider HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("record '{0}' is no longer listed by this provider — refresh the record list")]
    StaleRecord(String),

    #[error("'{0}' is part of a routing-policy record set — edit it in Route53")]
    RoutedRecord(String),
}

impl DnsError {
    /// Provider-facing errors carry the provider's own words, which is what an operator
    /// needs in the UI; transport errors would only leak a URL.
    pub fn provider(message: impl Into<String>) -> Self {
        Self::Provider {
            message: message.into(),
        }
    }

    pub fn is_stale_record(&self) -> bool {
        matches!(self, Self::StaleRecord(_))
    }
}
