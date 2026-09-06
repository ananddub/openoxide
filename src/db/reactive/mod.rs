mod bus;
mod cdc_bridge;
mod org_index;
mod scope;
pub mod types;

pub use bus::{event_bus, set_pool, DbEventBus};
pub use cdc_bridge::CdcBridge;
pub use org_index::OrgColumnIndex;
pub use scope::request_scope;
pub use sqlite_realtime::RealtimePool;
pub use types::{DbChangeEvent, DbChangeOperation, DbChangeSet};

use std::sync::OnceLock;

static REALTIME_POOL: OnceLock<sqlite_realtime::RealtimePool> = OnceLock::new();

/// Global handle to the realtime CDC pool for live query streams.
pub fn set_realtime(pool: sqlite_realtime::RealtimePool) {
    let _ = REALTIME_POOL.set(pool);
}

/// Panics only if called before database startup, which is a wiring bug.
pub fn realtime() -> &'static sqlite_realtime::RealtimePool {
    REALTIME_POOL
        .get()
        .expect("realtime pool accessed before database startup")
}

/// Registers a CDC handle for a (possibly replacement) pooled connection and
/// starts forwarding its committed events into the event bus. Called from
/// the pool's `after_connect` for every physical connection, so a sqlx
/// reconnect re-arms realtime capture instead of silently dropping it.
pub fn register_realtime_handle(handle: std::sync::Arc<sqlite_realtime::RealtimeHandle>) {
    static REGISTERED: OnceLock<()> = OnceLock::new();
    // The first connection's bridge is wired by connect(); skip re-spawning
    // for the same connection — attach_if_needed hands back the same handle.
    if REGISTERED.set(()).is_err() {
        // A replacement connection: spawn a fresh forwarder for its handle.
        tokio::spawn(async move {
            let mut events = handle.subscribe();
            while let Ok(event) = events.recv().await {
                cdc_bridge::forward_event(event).await;
            }
        });
    }
}

/// Re-discovers which tables carry an `organization_id` column.
pub(crate) async fn refresh_org_columns(pool: &sqlx::SqlitePool) {
    OrgColumnIndex::refresh(pool).await;
}
