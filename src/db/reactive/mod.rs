mod bus;
mod cdc_bridge;
mod org_index;
mod scope;
pub mod types;

pub use bus::{event_bus, set_pool, DbEventBus};
pub use org_index::OrgColumnIndex;
pub use scope::request_scope;
pub use sqlite_realtime::RealtimePool;
pub use types::{DbChangeEvent, DbChangeOperation, DbChangeSet};

use std::sync::{Arc, LazyLock, Mutex, OnceLock};

static REALTIME_POOL: OnceLock<sqlite_realtime::RealtimePool> = OnceLock::new();

/// Strong holder for the CDC handle of the pool's current physical
/// connection. `after_connect` attaches one per connection; without a
/// holder the Arc dies when the hook returns and `RealtimeHandle::Drop`
/// aborts the just-spawned reactor — silently killing event capture on
/// every sqlx connection replacement.
static LIVE_HANDLE: LazyLock<Mutex<Option<Arc<sqlite_realtime::RealtimeHandle>>>> =
    LazyLock::new(|| Mutex::new(None));

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

/// Releases the current CDC handle (aborting its reactor and releasing its
/// SQLite hooks) BEFORE a replacement connection attaches. Dropping after
/// the new attach would be fatal: the allocator can hand the freed
/// connection's address to the new `sqlite3`, and the handle's cleanup keys
/// the C context by that pointer — it would release the NEW connection's
/// context and capture would die silently again.
pub fn release_realtime_handle() {
    let slot = LIVE_HANDLE
        .lock()
        .expect("live handle slot poisoned")
        .take();
    let pooled = REALTIME_POOL.get().and_then(|pool| pool.take_handle());
    if slot.is_some() || pooled.is_some() {
        tracing::info!("realtime: releasing previous CDC handle(s)");
    }
    drop(slot);
    drop(pooled);
}

/// Registers a CDC handle for a (possibly replacement) pooled connection:
/// keeps it strongly alive for the connection's lifetime, spawns the
/// event-forwarding task (deduped via the handle's internal OnceLock), and
/// moves the global pool's live-query handle to it.
pub fn register_realtime_handle(handle: Arc<sqlite_realtime::RealtimeHandle>) {
    tracing::info!(
        db_ptr = handle.db_ptr(),
        event_fd = handle.get_fd(),
        "realtime: CDC handle registered for connection"
    );
    handle.spawn_event_forwarder(|event| async move {
        cdc_bridge::forward_event(event).await;
    });
    *LIVE_HANDLE
        .lock()
        .expect("live handle slot poisoned") = Some(Arc::clone(&handle));
    if let Some(pool) = REALTIME_POOL.get() {
        pool.set_handle(handle);
    }
}

/// Re-discovers which tables carry an `organization_id` column.
pub(crate) async fn refresh_org_columns(pool: &sqlx::SqlitePool) {
    OrgColumnIndex::refresh(pool).await;
}
