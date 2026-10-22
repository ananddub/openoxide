use crate::core::cache::enum_state::{CacheEnum, CacheKey};
use auto_di::singleton;
use std::future::Future;

/// Compatibility facade for legacy service constructors.
///
/// Endpoint caching is owned by `auto_socket`. Legacy reads always execute
/// their initializer and legacy writes/invalidations intentionally do nothing.
#[derive(Clone, Default)]
pub struct AppStateCache;

#[singleton]
impl AppStateCache {
    pub fn new() -> Self {
        Self
    }

    pub async fn get(&self, _key: &CacheKey) -> Option<CacheEnum> {
        None
    }

    pub async fn try_get_with<F, E>(
        &self,
        _key: CacheKey,
        init: F,
    ) -> Result<CacheEnum, std::sync::Arc<E>>
    where
        F: Future<Output = Result<CacheEnum, E>>,
        E: Send + Sync + 'static,
    {
        init.await.map_err(std::sync::Arc::new)
    }

    pub async fn insert(&self, _key: CacheKey, _value: CacheEnum) {}

    pub async fn invalidate(&self, _key: &CacheKey) {}

    pub async fn invalidate_all(&self) {}

    pub async fn invalidate_tables(&self, _tables: &[String]) {}

    pub fn entry_count(&self) -> u64 {
        0
    }
}
