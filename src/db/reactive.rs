use std::{
    future::Future,
    sync::{Arc, Mutex, OnceLock},
};

use sqlx::sqlite::{PreupdateHookResult, SqliteOperation};
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
            let mut tables = changes
                .iter()
                .map(|change| change.table.clone())
                .collect::<Vec<_>>();
            tables.sort_unstable();
            tables.dedup();
            html_rt::publish_table_changes(tables);
            auto_socket::notify_table_changes(changes.iter().map(|change| change.table.as_str()));
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

async fn publish_when_visible(changes: Vec<DbChangeEvent>) {
    if let Some(pool) = DB_POOL.get()
        && let Ok(mut connection) = pool.acquire().await
    {
        // A committed SQLite change is not guaranteed to be visible to another
        // connection while the commit hook itself is still running. Acquiring
        // the write lock ensures the originating transaction has fully exited.
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
    let pending = Arc::new(Mutex::new(Vec::<DbChangeEvent>::new()));
    let runtime = tokio::runtime::Handle::current();

    let preupdate_pending = Arc::clone(&pending);
    handle.set_preupdate_hook(move |result| {
        if let Ok(mut changes) = preupdate_pending.lock() {
            changes.push(change_event(result));
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

fn change_event(result: PreupdateHookResult<'_>) -> DbChangeEvent {
    DbChangeEvent {
        table: result.table.to_owned(),
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
