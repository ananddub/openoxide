use async_trait::async_trait;

use crate::config::ProviderConfig;
use crate::error::DnsError;
use crate::types::{Record, RecordInput, RecordTarget, Zone};

/// Everything the panel needs from a DNS provider.
///
/// Credentials travel per call rather than in the constructor, because one client serves
/// every stored provider row of that type. Implementations are stateless apart from their
/// base URL and HTTP client.
#[async_trait]
pub trait DnsProvider: Send + Sync {
    /// Proves the credentials work without writing anything.
    async fn test_connection(&self, config: &ProviderConfig) -> Result<(), DnsError>;

    async fn list_zones(&self, config: &ProviderConfig) -> Result<Vec<Zone>, DnsError>;

    async fn list_records(
        &self,
        config: &ProviderConfig,
        zone_id: &str,
    ) -> Result<Vec<Record>, DnsError>;

    async fn create_record(
        &self,
        config: &ProviderConfig,
        input: &RecordInput,
    ) -> Result<Record, DnsError>;

    async fn update_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
        next: &RecordInput,
    ) -> Result<Record, DnsError>;

    async fn delete_record(
        &self,
        config: &ProviderConfig,
        target: &RecordTarget,
    ) -> Result<(), DnsError>;
}

/// Finds the row a [`RecordTarget`] points at in a provider's own listing.
///
/// A bare name is never enough to identify a row: one name holds several types and one type
/// holds several values, so every field the caller filled in has to match. A miss means the
/// row is gone rather than a bad request, because the list came from the provider itself.
pub(crate) fn find_row(
    rows: &[Record],
    target: &RecordTarget,
    matches: impl Fn(&Record, &RecordTarget) -> bool,
) -> Result<usize, DnsError> {
    let found: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| matches(row, target))
        .map(|(index, _)| index)
        .collect();
    match found.as_slice() {
        [] => Err(DnsError::StaleRecord(target.name.clone())),
        [one] => Ok(*one),
        several => Err(DnsError::provider(format!(
            "{} records match '{}' — delete them one at a time",
            several.len(),
            target.name
        ))),
    }
}

/// The default row matcher: the id wins when the provider has one, otherwise every field the
/// caller supplied has to agree.
pub(crate) fn row_matches(row: &Record, target: &RecordTarget) -> bool {
    if target.has_id() {
        return row.id == target.id;
    }
    row.name == target.name
        && (target.record_type.trim().is_empty()
            || row
                .record_type
                .eq_ignore_ascii_case(target.record_type.trim()))
        && (target.content.trim().is_empty() || row.content == target.content)
}
