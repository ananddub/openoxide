use std::sync::Arc;

use auto_di::singleton;
use dashmap::DashMap;
use tokio::sync::mpsc;

use crate::api::connect::generated::openoxide::api::v1::WatchedTable;

use super::pump::{ChangePump, SUBSCRIBER_QUEUE};
use super::registry::TableRegistry;
use super::types::TableChangeBatch;

/// Tracks live WatchTables streams per organization. The pump owns a shared
/// handle and pushes batches in; `watch` hands out ownership-aware
/// subscriptions whose drop removes them again.
#[derive(Default)]
pub struct SubscriptionLedger {
    next_key: std::sync::atomic::AtomicU64,
    entries: DashMap<u64, SubscriberEntry>,
}

pub struct SubscriberEntry {
    pub key: u64,
    pub organization_id: i64,
    pub sender: mpsc::Sender<TableChangeBatch>,
}

impl SubscriptionLedger {
    fn insert(&self, organization_id: i64, sender: mpsc::Sender<TableChangeBatch>) -> u64 {
        let key = self
            .next_key
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.entries.insert(
            key,
            SubscriberEntry {
                key,
                organization_id,
                sender,
            },
        );
        key
    }

    fn remove(&self, key: u64) {
        self.entries.remove(&key);
    }

    /// Delivers a batch to every subscriber of its organization. The global
    /// batch (organization 0) is delivered to subscribers of all
    /// organizations. Send failures mean the subscriber went away; the
    /// entry is reaped immediately.
    pub(super) fn deliver(&self, batch: TableChangeBatch) {
        let is_global = batch.organization_id == 0;
        let stale: Vec<u64> = self
            .entries
            .iter()
            .filter(|entry| is_global || entry.organization_id == batch.organization_id)
            .filter_map(|entry| {
                entry
                    .sender
                    .try_send(batch.clone())
                    .err()
                    .map(|_| entry.key)
            })
            .collect();
        for key in stale {
            self.remove(key);
        }
    }

    /// Pushes a full-table resync batch to every subscriber, one per unique
    /// organization. Used when the pump falls behind the broadcast channel
    /// and missed change-sets can no longer be recovered.
    pub(super) fn resync(&self, tables: Vec<WatchedTable>) {
        let organizations: std::collections::HashSet<i64> = self
            .entries
            .iter()
            .map(|entry| entry.organization_id)
            .collect();
        for organization_id in organizations {
            self.deliver(TableChangeBatch {
                organization_id,
                tables: tables.clone(),
            });
        }
    }
}

pub struct RealtimeService {
    registry: Arc<TableRegistry>,
    ledger: Arc<SubscriptionLedger>,
}

#[singleton]
impl RealtimeService {
    fn new() -> Self {
        let registry = Arc::new(TableRegistry::new());
        let ledger = Arc::new(SubscriptionLedger::default());
        tokio::spawn({
            let pump = ChangePump::new(Arc::clone(&registry), Arc::clone(&ledger));
            async move { pump.run().await }
        });
        Self { registry, ledger }
    }

    /// Registers a watch stream for an organization and returns the receive
    /// half plus the initial resync batch covering the subscribe gap.
    pub fn watch(&self, organization_id: i64) -> (mpsc::Receiver<TableChangeBatch>, TableChangeBatch) {
        let (sender, receiver) = mpsc::channel(SUBSCRIBER_QUEUE);
        self.ledger.insert(organization_id, sender);
        // Resync snapshot: everything may have changed between the client's
        // initial fetch and this stream opening.
        let resync = TableChangeBatch {
            organization_id,
            tables: self.registry.all(),
        };
        (receiver, resync)
    }
}
