use sqlite_realtime::{RealtimeOp, RealtimeRawEvent};

use super::bus::event_bus;
use super::types::{DbChangeEvent, DbChangeOperation};

/// Forwards committed change events from the realtime CDC engine
/// (crates/realtime) into the application-wide [event_bus] so the
/// WatchTables pump and any other bus consumers keep receiving changes.
/// The engine drains per-commit events in commit order; org scoping is
/// resolved by reading the row's `organization_id` after the commit
/// (unavailable in the raw update hook, which only carries table+rowid).
/// Spawned per live handle by [`super::register_realtime_handle`], so a
/// sqlx connection replacement re-arms forwarding with the new handle.
pub(super) async fn forward_event(event: RealtimeRawEvent) {
    // The pool is installed right after the first connection opens; CDC
    // forwards that fire earlier (startup migrations) have no subscribers
    // to notify, so they are dropped.
    let Some(pool) = super::bus::db_pool() else {
        return;
    };
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
    tracing::info!(
        table = %event.table,
        rowid = event.rowid,
        organization_id = ?organization_id,
        "cdc event forwarded to bus"
    );
    event_bus().publish(vec![DbChangeEvent {
        table: event.table,
        organization_id,
        old_row_id: (operation == DbChangeOperation::Delete).then_some(event.rowid),
        new_row_id: (operation != DbChangeOperation::Delete).then_some(event.rowid),
        operation,
    }]);
}

async fn lookup_organization_id(pool: &sqlx::SqlitePool, table: &str, rowid: i64) -> Option<i64> {
    let sql = sqlx::AssertSqlSafe(format!(
        "SELECT organization_id FROM \"{table}\" WHERE rowid = {rowid}"
    ));
    match sqlx::query_scalar::<_, i64>(sql).fetch_optional(pool).await {
        Ok(value) => value,
        Err(error) => {
            tracing::debug!(table, rowid, error = %error, "org lookup failed for cdc event");
            None
        }
    }
}
