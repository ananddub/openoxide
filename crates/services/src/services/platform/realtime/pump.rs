use std::sync::Arc;

use crate::db::reactive::{DbChangeSet, event_bus};

use super::registry::TableRegistry;
use super::service::SubscriptionLedger;
use super::types::{TableBatcher, TableChangeBatch};

/// Forwards committed database changes to the per-organization subscriber
/// queues. One pump runs for the lifetime of the panel; broadcast lag is
/// recovered by pushing full-table resync batches instead of missed
/// change-sets, so subscribers never rely on pump death to resync.
pub struct ChangePump {
    registry: Arc<TableRegistry>,
    ledger: Arc<SubscriptionLedger>,
    batcher: TableBatcher,
}

impl ChangePump {
    pub fn new(registry: Arc<TableRegistry>, ledger: Arc<SubscriptionLedger>) -> Self {
        Self {
            registry,
            ledger,
            batcher: TableBatcher::new(),
        }
    }

    /// Consumes the global event bus until the panel shuts down. Never
    /// returns under normal operation.
    pub async fn run(self) {
        let mut receiver = event_bus().subscribe();
        loop {
            match receiver.recv().await {
                Ok(change_set) => self.dispatch(change_set),
                Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
                    // Missed change-sets are unrecoverable; the only safe
                    // signal is a full-table resync to every subscriber.
                    tracing::warn!(
                        missed,
                        "db event bus lagged, issuing full resync to subscribers"
                    );
                    self.ledger.resync(self.registry.all());
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    }

    fn dispatch(&self, change_set: DbChangeSet) {
        let batches = self.route(&change_set);
        tracing::debug!(
            table = %change_set.changes[0].table,
            batches = batches.len(),
            "change pump dispatched"
        );
        for batch in batches {
            self.ledger.deliver(batch);
        }
    }

    /// Splits one commit into per-organization batches. A single commit can
    /// touch rows of several organizations (e.g. cross-org admin actions);
    /// each affected org gets its own notification. Rows without a resolvable
    /// organization are treated as global and fan out to every subscriber.
    fn route(&self, change_set: &DbChangeSet) -> Vec<TableChangeBatch> {
        let mut batches = Vec::new();
        let mut global_tables = Vec::new();
        let mut per_org: std::collections::HashMap<i64, Vec<_>> = std::collections::HashMap::new();

        for change in &change_set.changes {
            let table = self.registry.resolve(&change.table);
            match change.organization_id {
                Some(organization_id) => per_org.entry(organization_id).or_default().push(table),
                None => global_tables.push(table),
            }
        }

        for (organization_id, tables) in per_org {
            if let Some(batch) = self
                .batcher
                .batch(organization_id, tables, self.registry.all())
            {
                batches.push(batch);
            }
        }
        if let Some(global) = self.batcher.batch(0, global_tables, self.registry.all()) {
            batches.push(global);
        }
        batches
    }
}

/// Queue capacity per subscriber. Enough to absorb a coalescing window of
/// bursts without dropping; overflow still self-heals via client reconnect.
pub const SUBSCRIBER_QUEUE: usize = 64;
