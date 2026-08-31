use std::collections::HashSet;
use std::time::Duration;

use buffa::EnumValue;
use connectrpc::ConnectError;
use futures::Stream;
use tokio::sync::mpsc;

use crate::api::connect::generated::openoxide::api::v1::{WatchTablesEvent, WatchedTable};

use super::types::TableChangeBatch;

/// Coalescing window. Batches arriving within this window merge into one
/// event (table sets union), trading at most this much notification latency
/// for far fewer client refetches during deploy-sized write bursts.
const COALESCE_WINDOW: Duration = Duration::from_millis(150);

/// Response body of `WatchTables`: adapts an organization's notification
/// queue into a ConnectRPC stream. The first event is always the resync
/// snapshot handed out at subscribe time; afterwards every batch drains the
/// queue until it stays quiet for one coalescing window, then flushes as a
/// single deduplicated event.
pub struct TableWatchStream {
    receiver: mpsc::Receiver<TableChangeBatch>,
    resync: Option<TableChangeBatch>,
}

impl TableWatchStream {
    pub fn new(receiver: mpsc::Receiver<TableChangeBatch>, resync: TableChangeBatch) -> Self {
        Self {
            receiver,
            resync: Some(resync),
        }
    }

    async fn next_event(&mut self) -> Option<WatchTablesEvent> {
        if let Some(resync) = self.resync.take() {
            return Some(event(&resync));
        }
        let mut tables = self.receiver.recv().await?.tables;
        // Drain batches landing inside the coalescing window so a write
        // burst yields one event instead of one event per commit.
        let deadline = tokio::time::Instant::now() + COALESCE_WINDOW;
        while let Ok(Some(batch)) = tokio::time::timeout_at(deadline, self.receiver.recv()).await {
            tables.extend(batch.tables);
        }
        Some(WatchTablesEvent {
            tables: dedupe(tables),
        })
    }
}

fn event(batch: &TableChangeBatch) -> WatchTablesEvent {
    WatchTablesEvent {
        tables: dedupe(batch.tables.clone()),
    }
}

fn dedupe(tables: Vec<WatchedTable>) -> Vec<EnumValue<WatchedTable>> {
    let mut seen = HashSet::new();
    tables
        .into_iter()
        .filter(|table| seen.insert(*table))
        .map(EnumValue::Known)
        .collect()
}

impl Stream for TableWatchStream {
    type Item = Result<WatchTablesEvent, ConnectError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let future = self.next_event();
        futures::pin_mut!(future);
        future.as_mut().poll(cx).map(|event| event.map(Ok))
    }
}
