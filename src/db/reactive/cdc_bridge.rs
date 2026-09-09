use tokio::sync::broadcast;

use sqlite_realtime::{RealtimeOp, RealtimePool};

use super::bus::event_bus;
use super::types::{DbChangeEvent, DbChangeOperation};

/// Forwards committed change events from the realtime CDC engine
/// (crates/realtime) into the application-wide [event_bus] so the
/// WatchTables pump and any other bus consumers keep receiving changes.
/// The engine drains per-commit events in commit order; org scoping is
/// resolved by reading the row's `organization_id` after the commit
/// (unavailable in the raw update hook, which only carries table+rowid).
pub struct CdcBridge {
    pool: sqlx::SqlitePool,
}

impl CdcBridge {
    /// Spawns the forwarding task for the lifetime of the process.
    pub fn spawn(realtime: &RealtimePool) {
        let pool = realtime.inner().clone();
        let mut events = realtime.handle().subscribe();
        tokio::spawn(async move {
            let bridge = Self { pool };
            loop {
                match events.recv().await {
                    Ok(event) => bridge.forward(event).await,
                    Err(broadcast::error::RecvError::Closed) => break,
                    // Missed events only mean a missed wake-up for
                    // table-level watchers; keep the bridge alive.
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
        });
    }

    async fn forward(&self, event: sqlite_realtime::RealtimeRawEvent) {
        let forwarded = forward_event_with_pool(&self.pool, event).await;
        tracing::info!(
            table = %forwarded.table,
            rowid = forwarded.new_row_id.unwrap_or(forwarded.old_row_id.unwrap_or(0)),
            organization_id = ?forwarded.organization_id,
            "cdc event forwarded to bus"
        );
    }

    async fn lookup_organization_id(&self, table: &str, rowid: i64) -> Option<i64> {
        lookup_organization_id(&self.pool, table, rowid).await
    }
}

/// Standalone forwarder used by replacement pooled connections (sqlx
/// reconnect) — same event mapping as the startup bridge.
pub(super) async fn forward_event(event: sqlite_realtime::RealtimeRawEvent) {
    let pool = event_bus_pool().await;
    forward_event_with_pool(&pool, event).await;
}

async fn forward_event_with_pool(
    pool: &sqlx::SqlitePool,
    event: sqlite_realtime::RealtimeRawEvent,
) -> DbChangeEvent {
    let operation = match event.op {
        RealtimeOp::Insert => DbChangeOperation::Insert,
        RealtimeOp::Update => DbChangeOperation::Update,
        RealtimeOp::Delete => DbChangeOperation::Delete,
        RealtimeOp::Unknown => DbChangeOperation::Unknown,
    };
    let organization_id = match operation {
        DbChangeOperation::Delete | DbChangeOperation::Unknown => None,
        _ => lookup_organization_id(pool, &event.table, event.rowid).await,
    };
    let change = DbChangeEvent {
        table: event.table,
        organization_id,
        old_row_id: (operation == DbChangeOperation::Delete).then_some(event.rowid),
        new_row_id: (operation != DbChangeOperation::Delete).then_some(event.rowid),
        operation,
    };
    event_bus().publish(vec![change.clone()]);
    change
}

async fn lookup_organization_id(pool: &sqlx::SqlitePool, table: &str, rowid: i64) -> Option<i64> {
    let sql = sqlx::AssertSqlSafe(format!(
        "SELECT organization_id FROM \"{table}\" WHERE rowid = {rowid}"
    ));
    match sqlx::query_scalar::<_, i64>(sql)
        .fetch_optional(pool)
        .await
    {
        Ok(value) => value,
        Err(error) => {
            tracing::debug!(table, rowid, error = %error, "org lookup failed for cdc event");
            None
        }
    }
}

/// The startup bridge holds its own pool clone; replacement-connection
/// forwarders resolve the global pool lazily (it's set by then — a
/// replacement connection only exists after startup).
async fn event_bus_pool() -> sqlx::SqlitePool {
    super::realtime().inner().clone()
}
