use std::collections::HashSet;
use std::task::{Context, Poll};
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
///
/// The coalescing state (pending table set + flush deadline) lives on the
/// struct itself, not inside a per-poll future: a stream's `poll_next` must
/// stay safe to call with fresh wakers across yields, and any state created
/// inside a per-poll future would be dropped on every `Pending` return,
/// silently losing table notifications.
pub struct TableWatchStream {
    receiver: mpsc::Receiver<TableChangeBatch>,
    resync: Option<TableChangeBatch>,
    /// Tables accumulated during the current coalescing window. `None`
    /// while the stream is idle (no window open).
    pending: Option<Vec<WatchedTable>>,
    /// Timer for the coalescing window. Armed when the first batch of a
    /// burst arrives; polled inline so its waker keeps the stream parked
    /// until the flush deadline elapses.
    sleep: std::pin::Pin<Box<tokio::time::Sleep>>,
}

impl TableWatchStream {
    pub fn new(receiver: mpsc::Receiver<TableChangeBatch>, resync: TableChangeBatch) -> Self {
        Self {
            receiver,
            resync: Some(resync),
            pending: None,
            sleep: Box::pin(tokio::time::sleep_until(
                tokio::time::Instant::now() + COALESCE_WINDOW,
            )),
        }
    }

    fn poll_event(&mut self, cx: &mut Context<'_>) -> Poll<Option<WatchTablesEvent>> {
        // One-time resync snapshot from subscribe time.
        if let Some(resync) = self.resync.take() {
            return Poll::Ready(Some(event(&resync)));
        }

        // Drain the notification queue into the pending set. The first
        // batch of a burst opens (or reopens) the coalescing window.
        loop {
            match self.receiver.poll_recv(cx) {
                Poll::Ready(Some(batch)) => {
                    let window_was_idle = self.pending.is_none();
                    self.pending.get_or_insert_with(Vec::new).extend(batch.tables);
                    if window_was_idle {
                        self.sleep
                            .as_mut()
                            .reset(tokio::time::Instant::now() + COALESCE_WINDOW);
                    }
                }
                // Queue empty (waker registered) or sender gone: stop draining.
                Poll::Pending | Poll::Ready(None) => break,
            }
        }

        // Window open: flush one deduplicated event when it elapses.
        if self.pending.is_some() {
            match self.sleep.as_mut().poll(cx) {
                Poll::Ready(()) => {
                    let tables = self.pending.take().unwrap_or_default();
                    return Poll::Ready(Some(WatchTablesEvent {
                        tables: dedupe(tables),
                    }));
                }
                Poll::Pending => return Poll::Pending,
            }
        }

        // Idle with nothing accumulated. If the sender side is gone the
        // panel is shutting down — end the stream so the client reconnect
        // backoff takes over instead of spinning on a dead queue.
        if self.receiver.is_closed() {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
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
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.poll_event(cx).map(|event| event.map(Ok))
    }
}
