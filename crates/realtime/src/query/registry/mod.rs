use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

use crate::query::analyzer::analyze_query_tables;

/// Dynamic In-Memory Registry mapping table names to active query subscription IDs.
///
/// The registry is a pure dependency index — it does NOT own output channels.
/// Each subscription's output channel is managed by the evaluator that spawned it.
///
/// Architecture:
///   table_to_subs:  "users"  → {1, 3, 7}     (O(1) lookup per CDC event)
///                   "orders" → {1, 5}
///   sub_to_sql:     1 → "SELECT * FROM users JOIN orders ..."
///                   3 → "SELECT * FROM users WHERE ..."
pub struct LiveQueryRegistry {
    next_id: std::sync::atomic::AtomicU64,
    /// table_name → set of subscription IDs that depend on it
    table_to_subs: RwLock<HashMap<String, HashSet<u64>>>,
    /// sub_id → SQL query string (for debugging / dedup grouping)
    sub_to_sql: RwLock<HashMap<u64, String>>,
}

impl Default for LiveQueryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveQueryRegistry {
    pub fn new() -> Self {
        Self {
            next_id: std::sync::atomic::AtomicU64::new(1),
            table_to_subs: RwLock::new(HashMap::new()),
            sub_to_sql: RwLock::new(HashMap::new()),
        }
    }

    /// Register a live query and auto-discover its table dependencies
    /// using SQLite's native `EXPLAIN QUERY PLAN` engine.
    ///
    /// Returns `(subscription_id, dependent_tables)`.
    ///
    /// For a JOIN query like `SELECT * FROM users u JOIN orders o ON ...`,
    /// SQLite's optimizer emits separate plan rows (`SCAN users`, `SEARCH orders`),
    /// so `deps` will contain `{"users", "orders"}`. Both tables are then mapped
    /// to this subscription in `table_to_subs`.
    pub async fn register(
        &self,
        raw_sqlite_ptr: usize,
        sql: &str,
    ) -> (u64, HashSet<String>) {
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let deps = analyze_query_tables(raw_sqlite_ptr as *mut std::ffi::c_void, sql);

        tracing::debug!(
            sub_id = id,
            sql = sql,
            tables = ?deps,
            "live query registered"
        );

        {
            let mut sql_map = self.sub_to_sql.write().await;
            sql_map.insert(id, sql.to_string());
        }

        {
            let mut table_map = self.table_to_subs.write().await;
            for table in &deps {
                table_map.entry(table.clone()).or_default().insert(id);
            }
        }

        (id, deps)
    }

    /// O(1) check: does this subscription depend on the changed table?
    pub async fn is_affected(&self, changed_table: &str, sub_id: u64) -> bool {
        let table_map = self.table_to_subs.read().await;
        table_map
            .get(changed_table)
            .is_some_and(|subs| subs.contains(&sub_id))
    }

    /// Get all subscription IDs affected by a table change.
    pub async fn get_affected_sub_ids(&self, changed_table: &str) -> HashSet<u64> {
        let table_map = self.table_to_subs.read().await;
        table_map.get(changed_table).cloned().unwrap_or_default()
    }

    /// Unregister a subscription and clean up all its table mappings.
    /// Must be called when a subscriber drops to prevent memory leaks.
    pub async fn unregister(&self, id: u64) {
        {
            let mut sql_map = self.sub_to_sql.write().await;
            sql_map.remove(&id);
        }

        {
            let mut table_map = self.table_to_subs.write().await;
            for subs in table_map.values_mut() {
                subs.remove(&id);
            }
            // Prune empty entries to keep memory bounded
            table_map.retain(|_, subs| !subs.is_empty());
        }

        tracing::debug!(sub_id = id, "live query unregistered");
    }

    /// Number of active subscriptions (for diagnostics).
    pub async fn active_count(&self) -> usize {
        self.sub_to_sql.read().await.len()
    }
}
