//! Managed DNS providers.
//!
//! One [`DnsProvider`] trait, one thin client per provider, and one
//! [`DnsProviderKind`](crate::DnsProviderKind) list that the panel's database column, the
//! proto enum and the web catalog all key off. A provider either exists in all four or in
//! none — that is the whole point of putting the list here instead of in the panel.

pub mod config;
pub mod error;
pub mod http;
pub mod provider;
pub mod providers;
pub mod records;
pub mod registry;
pub mod types;

pub use config::{CredentialField, DnsProviderKind, ProviderConfig};
pub use error::DnsError;
pub use http::Http;
pub use provider::DnsProvider;
pub use registry::open;
pub use types::{Record, RecordInput, RecordTarget, Zone};
