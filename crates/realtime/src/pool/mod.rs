use std::sync::Arc;
use futures_util::Stream;
use sqlx::{FromRow, SqlitePool};

use crate::engine::RealtimeHandle;
use crate::query::evaluator::RealtimeTypedExt;
use crate::query::registry::LiveQueryRegistry;

/// Unified Developer-Friendly Realtime Database Pool.
///
/// Encapsulates `SqlitePool`, `RealtimeHandle`, and `LiveQueryRegistry` together
/// into a single struct so callers don't have to pass handles and registries around.
///
/// # Usage:
/// ```rust,ignore
/// // 1. Create once at startup:
/// let pool = RealtimePool::new(sqlite_pool).await?;
///
/// // 2. Watch anywhere cleanly in 1 line:
/// let mut stream = pool.watch_query::<User>("SELECT * FROM users").await?;
/// while let Some(users) = stream.next().await {
///     println!("Live users: {:?}", users);
/// }
/// ```
#[derive(Clone)]
pub struct RealtimePool {
    pool: SqlitePool,
    handle: Arc<RealtimeHandle>,
    registry: Arc<LiveQueryRegistry>,
}

impl RealtimePool {
    /// Initialize a `RealtimePool` from a pool whose connections already
    /// carry the CDC hooks (attached via `after_connect`). Idempotent —
    /// re-attaching to an already-hooked connection reuses its live handle.
    pub async fn new(pool: SqlitePool) -> Result<Self, String> {
        let mut conn = pool
            .acquire()
            .await
            .map_err(|e| format!("Failed to acquire connection for realtime init: {}", e))?;

        let mut raw_handle = conn
            .lock_handle()
            .await
            .map_err(|e| format!("Failed to lock handle: {}", e))?;

        let raw_ptr = raw_handle.as_raw_handle().as_ptr() as *mut std::ffi::c_void;
        let handle = RealtimeHandle::attach_if_needed(raw_ptr)?;
        drop(raw_handle);
        drop(conn);

        let registry = Arc::new(LiveQueryRegistry::new());

        Ok(Self {
            pool,
            handle,
            registry,
        })
    }

    /// Access the underlying `SqlitePool` for standard queries and mutations.
    pub fn inner(&self) -> &SqlitePool {
        &self.pool
    }

    /// Access the `RealtimeHandle` if low-level eventfd access is needed.
    pub fn handle(&self) -> &RealtimeHandle {
        &self.handle
    }

    /// Access the `LiveQueryRegistry` for diagnostics.
    pub fn registry(&self) -> &LiveQueryRegistry {
        &self.registry
    }

    /// 🎯 Watch any SQL query (including JOINs, subqueries, aggregates) and receive
    /// a type-safe live stream of `Vec<T>` updates whenever any dependent table changes!
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut stream = realtime_pool.watch_query::<Order>(
    ///     "SELECT * FROM orders WHERE status = 'PENDING'"
    /// ).await?;
    /// ```
    pub async fn watch_query<T>(
        &self,
        query: impl Into<String> + Send,
    ) -> Result<impl Stream<Item = Vec<T>>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static,
    {
        self.pool
            .watch_query_typed::<T>(&self.handle, self.registry.clone(), query)
            .await
    }
}
