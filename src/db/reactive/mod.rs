pub mod bus;
pub mod hooks;
pub mod org_index;
pub mod scope;
pub mod types;

pub use bus::{event_bus, set_pool, DbEventBus};
pub use hooks::install_hooks;
pub use org_index::OrgColumnIndex;
pub use scope::request_scope;
pub use types::{DbChangeEvent, DbChangeOperation, DbChangeSet};

/// Re-discovers which tables carry an `organization_id` column.
pub(crate) async fn refresh_org_columns(pool: &sqlx::SqlitePool) {
    OrgColumnIndex::refresh(pool).await;
}
