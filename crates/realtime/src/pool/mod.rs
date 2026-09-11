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
    handle: Arc<std::sync::RwLock<Option<Arc<RealtimeHandle>>>>,
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
            handle: Arc::new(std::sync::RwLock::new(Some(handle))),
            registry,
        })
    }

    /// Access the underlying `SqlitePool` for standard queries and mutations.
    pub fn inner(&self) -> &SqlitePool {
        &self.pool
    }

    /// The handle attached to the pool's CURRENT physical connection, or
    /// `None` while a connection replacement is in flight.
    pub fn handle(&self) -> Option<Arc<RealtimeHandle>> {
        self.handle
            .read()
            .expect("realtime handle lock poisoned")
            .clone()
    }

    /// Installs the handle for the pool's current physical connection.
    /// Called from `after_connect` when a recreated connection re-attaches
    /// the CDC engine, so live query streams follow the pool instead of
    /// polling a dead connection's eventfd.
    pub fn set_handle(&self, handle: Arc<RealtimeHandle>) {
        *self.handle.write().expect("realtime handle lock poisoned") = Some(handle);
    }

    /// Detaches and returns the current handle so the caller can drop it
    /// BEFORE a replacement connection attaches. Ordering matters: SQLite
    /// frees the old connection and the allocator may hand the same address
    /// to the new one — a handle dropped after that attach would clean up
    /// (by reused pointer) the NEW connection's C context and silently
    /// kill event capture.
    pub fn take_handle(&self) -> Option<Arc<RealtimeHandle>> {
        self.handle
            .write()
            .expect("realtime handle lock poisoned")
            .take()
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
        let handle = match self.handle() {
            Some(handle) => handle,
            None => {
                // Replacement attach in flight or pool built without one —
                // resolve the handle from the CURRENT physical connection
                // (attach_if_needed reuses the live handle for it).
                let mut conn = self.pool.acquire().await?;
                let mut raw_handle = conn.lock_handle().await?;
                let raw_ptr =
                    raw_handle.as_raw_handle().as_ptr() as *mut std::ffi::c_void;
                drop(raw_handle);
                drop(conn);
                RealtimeHandle::attach_if_needed(raw_ptr)
                    .map_err(|error| sqlx::Error::Configuration(error.into()))?
            }
        };
        self.pool
            .watch_query_typed::<T>(handle, self.registry.clone(), query)
            .await
    }
}
