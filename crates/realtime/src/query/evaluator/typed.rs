use std::sync::Arc;
use futures_util::{Stream, StreamExt};
use sqlx::{FromRow, SqlitePool};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::c_ffi::{RealtimeOp, TypedChangeEvent};
use crate::engine::RealtimeHandle;
use crate::query::evaluator::debounce_window;
use crate::query::registry::LiveQueryRegistry;

/// Extension Trait for `SqlitePool` providing fully Type-Safe, Auto-Parsed
/// Realtime Streams for both Tables and Live Queries.
pub trait RealtimeTypedExt {
    /// Watch a single table and auto-parse each mutation into a typed struct `T`.
    ///
    /// On INSERT/UPDATE, fetches the new row from the table via `rowid` and
    /// deserializes it into `T` using `FromRow`. On DELETE, `data` is `None`.
    ///
    /// ```rust,ignore
    /// let mut stream = pool.watch_table_typed::<User>(&handle, "users");
    /// while let Some(event) = stream.next().await {
    ///     println!("Op: {:?} | User: {:?}", event.op, event.data);
    /// }
    /// ```
    fn watch_table_typed<T>(
        &self,
        handle: &RealtimeHandle,
        target_table: &str,
    ) -> impl Stream<Item = TypedChangeEvent<T>>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static;

    /// Watch an arbitrary SQL query (including JOINs, subqueries, aggregates)
    /// and re-evaluate it whenever any dependent table changes.
    ///
    /// Uses SQLite's native `EXPLAIN QUERY PLAN` to auto-discover table
    /// dependencies. For a JOIN like `SELECT * FROM users u JOIN orders o ON ...`,
    /// both `users` and `orders` are tracked — a mutation in either triggers
    /// re-evaluation.
    ///
    /// The stream emits the FULL result set (`Vec<T>`) after each re-evaluation,
    /// so the consumer always has a consistent snapshot.
    ///
    /// Features:
    /// - **Leading + trailing edge debounce** (50ms): burst writes produce at
    ///   most 2 evaluations (first + final), never stale data.
    /// - **Auto-cleanup**: registry entry is removed when the stream is dropped.
    ///
    /// ```rust,ignore
    /// let mut stream = pool.watch_query_typed::<Deployment>(
    ///     &handle, registry.clone(),
    ///     "SELECT * FROM deployments WHERE status = 'RUNNING'"
    /// ).await?;
    /// while let Some(deployments) = stream.next().await {
    ///     println!("Live: {:?}", deployments);
    /// }
    /// ```
    fn watch_query_typed<T>(
        &self,
        handle: Arc<RealtimeHandle>,
        registry: Arc<LiveQueryRegistry>,
        query: impl Into<String> + Send,
    ) -> impl std::future::Future<Output = Result<impl Stream<Item = Vec<T>>, sqlx::Error>> + Send
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static;
}

impl RealtimeTypedExt for SqlitePool {
    fn watch_table_typed<T>(
        &self,
        handle: &RealtimeHandle,
        target_table: &str,
    ) -> impl Stream<Item = TypedChangeEvent<T>>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static,
    {
        let target = target_table.to_string();
        let rx = handle.subscribe();
        let pool = self.clone();

        tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(move |res| {
            let target = target.clone();
            let pool = pool.clone();
            async move {
                let raw = res.ok()?;
                if raw.table != target {
                    return None;
                }

                // DELETE: data is gone, can't fetch
                if raw.op == RealtimeOp::Delete {
                    return Some(TypedChangeEvent {
                        op: raw.op,
                        table: raw.table,
                        rowid: raw.rowid,
                        data: None,
                    });
                }

                // INSERT/UPDATE: fetch the actual row and parse via FromRow (Bug #5 fix)
                let query = format!("SELECT * FROM \"{}\" WHERE rowid = ?", raw.table);
                let parsed_data: Option<T> = sqlx::query_as::<_, T>(
                    sqlx::AssertSqlSafe(query.as_str()),
                )
                .bind(raw.rowid)
                .fetch_optional(&pool)
                .await
                .ok()
                .flatten();

                Some(TypedChangeEvent {
                    op: raw.op,
                    table: raw.table,
                    rowid: raw.rowid,
                    data: parsed_data,
                })
            }
        })
    }

    async fn watch_query_typed<T>(
        &self,
        handle: Arc<RealtimeHandle>,
        registry: Arc<LiveQueryRegistry>,
        query: impl Into<String> + Send,
    ) -> Result<impl Stream<Item = Vec<T>>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static,
    {
        let sql = query.into();
        let pool = self.clone();

        // ── 1. Initial snapshot ───────────────────────────────────────────
        let (initial_rows, initial_hash) = run_query_with_hash::<T>(&pool, sql.as_str())
            .await
            .ok_or_else(|| sqlx::Error::RowNotFound)?;

        // ── 2. Register with REAL db pointer (Bug #1 fix) ────────────────
        //    `handle.db_ptr()` returns the actual `sqlite3*` as usize,
        //    NOT 0usize (NULL) like the broken version.
        let db_ptr = handle.db_ptr();
        let (sub_id, deps) = registry.register(db_ptr, &sql).await;

        tracing::info!(
            sub_id = sub_id,
            sql = sql.as_str(),
            tables = ?deps,
            "live query stream started"
        );

        let (out_tx, out_rx) = mpsc::channel::<Vec<T>>(32);
        let _ = out_tx.send(initial_rows).await;

        // ── 3. Reactor loop with leading + trailing edge debounce + 8-byte HASH Diff ─
        //
        // 🎯 Memory Optimization: We only store an 8-byte `u64` hash instead of
        // keeping the entire `Vec<T>` in memory! Whether the query returns 10 rows
        // or 1,000,000 rows, RAM stays fixed at 8 BYTES.
        let pool_clone = pool.clone();
        let sql_clone = sql.clone();
        let registry_clone = Arc::clone(&registry);
        let mut cdc_rx = handle.subscribe();
        let window = debounce_window();

        tokio::spawn(async move {
            let mut trailing_deadline: Option<tokio::time::Instant> = None;
            let mut dirty = false;
            let mut last_result_hash: u64 = initial_hash;

            loop {
                tokio::select! {
                    // ── Branch A: CDC event received ──────────────────────
                    result = cdc_rx.recv() => {
                        match result {
                            Ok(raw_ev) => {
                                if !registry_clone.is_affected(&raw_ev.table, sub_id).await {
                                    continue;
                                }

                                if trailing_deadline.is_none() {
                                    // Leading edge: run immediately
                                    if let Some((rows, hash)) = run_query_with_hash::<T>(&pool_clone, &sql_clone).await {
                                        // 🎯 8-byte Hash Diff Check: Only emit if content ACTUALLY changed!
                                        if hash != last_result_hash {
                                            last_result_hash = hash;
                                            if out_tx.send(rows).await.is_err() {
                                                break; // subscriber dropped
                                            }
                                        }
                                    }
                                    trailing_deadline = Some(
                                        tokio::time::Instant::now() + window
                                    );
                                    dirty = false;
                                } else {
                                    dirty = true;
                                }
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                                tracing::warn!(
                                    sub_id = sub_id,
                                    lagged = n,
                                    "CDC broadcast lagged, forcing re-eval"
                                );
                                dirty = true;
                            }
                        }
                    }

                    // ── Branch B: Trailing-edge timer fired ──────────────
                    _ = async {
                        match trailing_deadline {
                            Some(deadline) => tokio::time::sleep_until(deadline).await,
                            None => std::future::pending().await,
                        }
                    }, if trailing_deadline.is_some() => {
                        if dirty {
                            // Trailing edge: final re-eval with latest data
                            if let Some((rows, hash)) = run_query_with_hash::<T>(&pool_clone, &sql_clone).await {
                                if hash != last_result_hash {
                                    last_result_hash = hash;
                                    if out_tx.send(rows).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            dirty = false;
                        }
                        trailing_deadline = None;
                    }
                }
            }

            // ── 4. Cleanup on exit (Bug #3 fix) ─────────────────────────
            registry_clone.unregister(sub_id).await;
            tracing::info!(sub_id = sub_id, "live query stream stopped");
        });

        Ok(ReceiverStream::new(out_rx))
    }
}

/// Execute query, build typed result vector, and compute an 8-byte 64-bit hash
/// of the raw row contents for O(1) duplicate suppression with zero RAM overhead.
async fn run_query_with_hash<T>(pool: &SqlitePool, sql: &str) -> Option<(Vec<T>, u64)>
where
    T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + 'static,
{
    use std::hash::{DefaultHasher, Hasher};
    use sqlx::Row;

    match sqlx::query(sqlx::AssertSqlSafe(sql)).fetch_all(pool).await {
        Ok(raw_rows) => {
            let mut hasher = DefaultHasher::new();
            let mut parsed_items = Vec::with_capacity(raw_rows.len());

            for row in raw_rows {
                let col_count = row.len();
                for i in 0..col_count {
                    if let Ok(Some(text)) = row.try_get::<Option<String>, _>(i) {
                        hasher.write(text.as_bytes());
                    } else if let Ok(Some(num)) = row.try_get::<Option<i64>, _>(i) {
                        hasher.write_i64(num);
                    } else if let Ok(Some(flt)) = row.try_get::<Option<f64>, _>(i) {
                        hasher.write(&flt.to_ne_bytes());
                    } else if let Ok(Some(bytes)) = row.try_get::<Option<Vec<u8>>, _>(i) {
                        hasher.write(&bytes);
                    } else {
                        hasher.write_u8(0);
                    }
                }
                hasher.write_u8(0xFF); // row boundary

                // Parse into user struct
                if let Ok(item) = T::from_row(&row) {
                    parsed_items.push(item);
                }
            }

            let hash = hasher.finish();
            Some((parsed_items, hash))
        }
        Err(e) => {
            tracing::error!(error = %e, sql = sql, "live query re-evaluation failed");
            None
        }
    }
}

// TODO(P2): Singleflight / Query Deduplication
//
// Currently, if 500 users subscribe to the EXACT SAME query, 500 separate
// Tokio tasks run 500 identical SQL queries on every table change.
//
// Future optimization: group subscriptions by SQL hash, run query ONCE per
// unique SQL, and fan-out the result via `Arc<Vec<T>>` + broadcast channel.
//
// Architecture sketch:
//   sql_hash -> SharedEvaluator { broadcast::Sender<Arc<Vec<T>>>, ref_count }
//   On table change: for each unique SQL → run once → broadcast to all subs
//   On last subscriber drop: remove SharedEvaluator entry
