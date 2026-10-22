mod bus;
mod cdc_bridge;
mod org_index;
pub mod types;

pub use bus::{DbEventBus, event_bus, set_pool};
pub use types::{DbChangeEvent, DbChangeOperation, DbChangeSet};

use std::sync::{Arc, LazyLock, Mutex};

/// Strong holder for the CDC handle of the pool's current physical
/// connection. `after_connect` attaches one per connection; without a
/// holder the Arc dies when the hook returns and `RealtimeHandle::Drop`
/// aborts the just-spawned reactor — silently killing event capture on
/// every sqlx connection replacement.
static LIVE_HANDLE: LazyLock<Mutex<Option<Arc<sqlite_realtime::RealtimeHandle>>>> =
    LazyLock::new(|| Mutex::new(None));

/// Releases the current CDC handle (aborting its reactor and releasing its
/// SQLite hooks) BEFORE a replacement connection attaches. Dropping after
/// the new attach would be fatal: the allocator can hand the freed
/// connection's address to the new `sqlite3`, and the handle's cleanup keys
/// the C context by that pointer — it would release the NEW connection's
/// context and capture would die silently again.
pub fn release_realtime_handle() {
    let previous = LIVE_HANDLE
        .lock()
        .expect("live handle slot poisoned")
        .take();
    if previous.is_some() {
        tracing::info!("realtime: releasing previous CDC handle");
    }
    drop(previous);
}

/// Registers a CDC handle for a (possibly replacement) pooled connection:
/// keeps it strongly alive for the connection's lifetime and spawns the
/// event-forwarding task (deduped via the handle's internal OnceLock).
pub fn register_realtime_handle(handle: Arc<sqlite_realtime::RealtimeHandle>) {
    tracing::info!(
        db_ptr = handle.db_ptr(),
        event_fd = handle.get_fd(),
        "realtime: CDC handle registered for connection"
    );
    handle.spawn_event_forwarder(|event| async move {
        cdc_bridge::forward_event(event).await;
    });
    *LIVE_HANDLE.lock().expect("live handle slot poisoned") = Some(handle);
}

/// Re-discovers which tables carry an `organization_id` column so the CDC
/// bridge can skip the per-event lookup for org-less tables.
pub(crate) async fn refresh_org_columns(pool: &sqlx::SqlitePool) {
    org_index::refresh(pool).await;
}
