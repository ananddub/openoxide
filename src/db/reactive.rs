use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex, OnceLock, RwLock},
};

use sqlx::sqlite::{PreupdateHookResult, SqliteOperation};
use sqlx::{Decode, Row};
use tokio::sync::broadcast;

static DB_POOL: OnceLock<sqlx::SqlitePool> = OnceLock::new();

tokio::task_local! {
    static REQUEST_CHANGES: Arc<Mutex<Vec<DbChangeEvent>>>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbChangeOperation {
    Insert,
    Update,
    Delete,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbChangeEvent {
    pub table: String,
    pub organization_id: Option<i64>,
    pub old_row_id: Option<i64>,
    pub new_row_id: Option<i64>,
    pub operation: DbChangeOperation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbChangeSet {
    pub changes: Vec<DbChangeEvent>,
}

pub struct DbEventBus {
    sender: broadcast::Sender<DbChangeSet>,
}

impl DbEventBus {
    fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<DbChangeSet> {
        self.sender.subscribe()
    }

    fn publish(&self, changes: Vec<DbChangeEvent>) {
        if !changes.is_empty() {
            tracing::info!(tables = ?changes.iter().map(|c| c.table.as_str()).collect::<Vec<_>>(), "sqlite realtime change committed");
            let _ = self.sender.send(DbChangeSet { changes });
        }
    }
}

pub fn event_bus() -> &'static DbEventBus {
    static BUS: OnceLock<DbEventBus> = OnceLock::new();
    BUS.get_or_init(|| DbEventBus::new(256))
}

pub fn set_pool(pool: sqlx::SqlitePool) {
    let _ = DB_POOL.set(pool);
}

/// Re-discovers which tables carry an `organization_id` column. Called after
/// migrations so new tables are picked up without restarting the panel.
pub(crate) async fn refresh_org_columns(pool: &sqlx::SqlitePool) {
    OrgColumnIndex::refresh(pool).await;
}

async fn publish_when_visible(changes: Vec<DbChangeEvent>) {
    if let Some(pool) = DB_POOL.get()
        && let Ok(mut connection) = pool.acquire().await
    {
        if sqlx::query("BEGIN IMMEDIATE")
            .execute(&mut *connection)
            .await
            .is_ok()
        {
            let _ = sqlx::query("ROLLBACK").execute(&mut *connection).await;
        }
    }
    event_bus().publish(changes);
}

pub async fn request_scope<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    let changes = Arc::new(Mutex::new(Vec::new()));
    let result = REQUEST_CHANGES.scope(Arc::clone(&changes), future).await;
    let changes = changes
        .lock()
        .map(|mut changes| std::mem::take(&mut *changes))
        .unwrap_or_default();
    event_bus().publish(changes);
    result
}

pub(crate) fn install_hooks(handle: &mut sqlx::sqlite::LockedSqliteHandle<'_>) {
    let registry = OrgColumnIndex::global();
    let pending = Arc::new(Mutex::new(Vec::<DbChangeEvent>::new()));
    let runtime = tokio::runtime::Handle::current();

    let preupdate_pending = Arc::clone(&pending);
    handle.set_preupdate_hook(move |result| {
        if let Ok(mut changes) = preupdate_pending.lock() {
            changes.push(change_event(result, registry));
        }
    });

    let commit_pending = Arc::clone(&pending);
    let commit_runtime = runtime.clone();
    handle.set_commit_hook(move || {
        let changes = commit_pending
            .lock()
            .map(|mut changes| std::mem::take(&mut *changes))
            .unwrap_or_default();

        // SQLx maps `true` to SQLite's zero return value, which allows commit.
        // HTTP requests batch their changes until the handler (including its
        // cache invalidation) has completed. Background writes have no request
        // scope, so retain a small visibility defer for those only.
        if !changes.is_empty() {
            let queued = REQUEST_CHANGES
                .try_with(|request_changes| {
                    if let Ok(mut request_changes) = request_changes.lock() {
                        request_changes.extend(changes.iter().cloned());
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(false);
            if !queued {
                commit_runtime.spawn(publish_when_visible(changes));
            }
        }

        true
    });

    handle.set_rollback_hook(move || {
        if let Ok(mut changes) = pending.lock() {
            changes.clear();
        }
    });
}

fn change_event(result: PreupdateHookResult<'_>, registry: &OrgColumnIndex) -> DbChangeEvent {
    DbChangeEvent {
        table: result.table.to_owned(),
        organization_id: registry.organization_id(&result),
        old_row_id: result.get_old_row_id().ok(),
        new_row_id: result.get_new_row_id().ok(),
        operation: match result.operation {
            SqliteOperation::Insert => DbChangeOperation::Insert,
            SqliteOperation::Update => DbChangeOperation::Update,
            SqliteOperation::Delete => DbChangeOperation::Delete,
            _ => DbChangeOperation::Unknown,
        },
    }
}

/// Resolves the `organization_id` column index for each table so the
/// preupdate hook can route changes to an organization without issuing any
/// schema reads on the hot path.
///
/// The index is discovered asynchronously right after migrations run
/// (`refresh`) and shared by every pooled connection through `global()`.
/// Tables without the column (or before the first refresh) resolve to `None`;
/// such rows are not org-routable and the realtime layer treats them as
/// global.
struct OrgColumnIndex {
    columns: RwLock<HashMap<String, Option<i32>>>,
}

impl OrgColumnIndex {
    fn global() -> &'static Self {
        static INDEX: OnceLock<OrgColumnIndex> = OnceLock::new();
        INDEX.get_or_init(|| Self {
            columns: RwLock::new(HashMap::new()),
        })
    }

    pub(crate) async fn refresh(pool: &sqlx::SqlitePool) {
        let tables = sqlx::query("SELECT name FROM sqlite_master WHERE type = 'table'")
            .fetch_all(pool)
            .await;
        let Ok(tables) = tables else {
            return;
        };
        let mut columns = HashMap::new();
        for table in tables {
            let Ok(name) = table.try_get::<String, _>(0) else {
                continue;
            };
            // Table names come straight from sqlite_master, not user input;
            // quoting keeps even a hostile table name a literal identifier.
            let info = sqlx::query(sqlx::AssertSqlSafe(format!(
                "PRAGMA table_info(\"{name}\")"
            )))
            .fetch_all(pool)
            .await;
            let Ok(info) = info else {
                continue;
            };
            let index = info
                .iter()
                .enumerate()
                .find_map(|(index, column)| {
                    column
                        .try_get::<String, _>(1)
                        .is_ok_and(|column| column == "organization_id")
                        .then_some(index as i32)
                });
            columns.insert(name, index);
        }
        if let Ok(mut self_columns) = Self::global().columns.write() {
            *self_columns = columns;
        }
    }

    fn organization_id(&self, result: &PreupdateHookResult<'_>) -> Option<i64> {
        let index = *self.columns.read().ok()?.get(result.table)?;
        let index = index?;
        // Insert has no old row; Delete has no new row.
        let value = match result.operation {
            SqliteOperation::Delete => result.get_old_column_value(index),
            _ => result.get_new_column_value(index),
        };
        Option::<i64>::decode(value.ok()?).ok().flatten()
    }
}
