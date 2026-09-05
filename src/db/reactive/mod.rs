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

/// Re-discovers which tables carry an `organization_id` column.
pub(crate) async fn refresh_org_columns(pool: &sqlx::SqlitePool) {
    OrgColumnIndex::refresh(pool).await;
}
