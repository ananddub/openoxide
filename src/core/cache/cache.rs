use crate::core::cache::enum_state::{CacheEnum, CacheKey};
use auto_di::singleton;
use moka::future::Cache;
use std::future::Future;
use std::hash::Hash;
use std::time::Duration;

/// Core high-performance in-memory cache wrapper powered by Moka.
/// Supports async operations, auto TTL expiration, and LRU capacity eviction.
#[derive(Clone)]
pub struct AppCache<K, V>
where
    K: Send + Sync + Hash + Eq + 'static,
    V: Send + Sync + Clone + 'static,
{
    inner: Cache<K, V>,
}

impl<K, V> AppCache<K, V>
where
    K: Send + Sync + Hash + Eq + 'static,
    V: Send + Sync + Clone + 'static,
{
    pub fn new() -> Self {
        let inner = Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_mins(10))
            .support_invalidation_closures()
            .build();

        Self { inner }
    }

    /// Retrieve a cached value if present and not expired.
    pub async fn get(&self, key: &K) -> Option<V> {
        self.inner.get(key).await
    }

    /// Insert or update a key-value pair in the cache.
    pub async fn insert(&self, key: K, value: V) {
        self.inner.insert(key, value).await;
    }

    /// Remove a specific key from the cache (invalidation).
    pub async fn invalidate(&self, key: &K) {
        self.inner.invalidate(key).await;
    }

    /// Retrieve a cached value or atomically execute the async init closure (coalesces concurrent requests to prevent Cache Stampede).
    pub async fn try_get_with<F, E>(&self, key: K, init: F) -> Result<V, std::sync::Arc<E>>
    where
        F: Future<Output = Result<V, E>>,
        E: Send + Sync + 'static,
    {
        self.inner.try_get_with(key, init).await
    }

    /// Clear all entries from the cache.
    pub async fn invalidate_all(&self) {
        self.inner.invalidate_all();
    }

    pub fn invalidate_entries_if(
        &self,
        predicate: impl Fn(&K, &V) -> bool + Send + Sync + 'static,
    ) {
        let _ = self.inner.invalidate_entries_if(predicate);
    }

    pub async fn run_pending_tasks(&self) {
        self.inner.run_pending_tasks().await;
    }

    /// Get current estimated entry count in cache.
    pub fn entry_count(&self) -> u64 {
        self.inner.entry_count()
    }
}

/// Global Application State Cache singleton registered with auto_di
#[derive(Clone)]
pub struct AppStateCache {
    inner: AppCache<CacheKey, CacheEnum>,
}

#[singleton]
impl AppStateCache {
    pub fn new() -> Self {
        Self {
            inner: AppCache::new(),
        }
    }

    pub async fn get(&self, key: &CacheKey) -> Option<CacheEnum> {
        self.inner.get(key).await
    }

    pub async fn try_get_with<F, E>(
        &self,
        key: CacheKey,
        init: F,
    ) -> Result<CacheEnum, std::sync::Arc<E>>
    where
        F: Future<Output = Result<CacheEnum, E>>,
        E: Send + Sync + 'static,
    {
        self.inner.try_get_with(key, init).await
    }

    pub async fn insert(&self, key: CacheKey, value: CacheEnum) {
        self.inner.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: &CacheKey) {
        self.inner.invalidate(key).await;
    }

    pub async fn invalidate_all(&self) {
        self.inner.invalidate_all().await;
    }

    pub async fn invalidate_tables(&self, tables: &[String]) {
        let tables = tables
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>();

        // Fixed cache keys use direct awaited invalidation. This avoids
        // predicate-maintenance timing for the common list endpoints.
        if tables.contains("servers") {
            self.invalidate(&CacheKey::ServersList).await;
        }
        if tables.contains("ssh_keys") {
            self.invalidate(&CacheKey::SshKeysList).await;
        }
        if tables.contains("destinations") {
            self.invalidate(&CacheKey::DestinationsList).await;
        }
        if tables.contains("registries") {
            self.invalidate(&CacheKey::RegistriesList).await;
        }
        if tables.contains("volume_backups") {
            self.invalidate(&CacheKey::VolumeBackups).await;
        }

        let invalidated_tables = tables.clone();
        self.inner.invalidate_entries_if(move |key, _| {
            use CacheKey::*;
            match key {
                Application(_) => invalidated_tables.contains("applications"),
                Compose(_) => invalidated_tables.contains("compose_projects"),
                Database(_) => [
                    "postgres_dbs",
                    "mysql_dbs",
                    "mariadb_dbs",
                    "mongo_dbs",
                    "redis_dbs",
                    "libsql_dbs",
                ]
                .iter()
                .any(|table| invalidated_tables.contains(*table)),
                ProjectsList(_) => invalidated_tables.contains("projects"),
                EnvironmentsProject(_) => invalidated_tables.contains("environments"),
                ServersList | SshKeysList | DestinationsList | RegistriesList | VolumeBackups => {
                    false
                }
                SchedulesCompose(_) | SchedulesApp(_) => invalidated_tables.contains("schedules"),
                DomainsCompose(_) | DomainsApp(_) => invalidated_tables.contains("domains"),
                SwarmInfo(_) | SwarmNodes(_) => false,
            }
        });
        self.inner.run_pending_tasks().await;
        tracing::info!(tables = ?tables, "live table caches invalidated");
    }

    pub fn entry_count(&self) -> u64 {
        self.inner.entry_count()
    }
}
