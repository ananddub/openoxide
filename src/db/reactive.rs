use std::sync::{Arc, Mutex, OnceLock};

use sqlx::sqlite::{PreupdateHookResult, SqliteOperation};
use tokio::sync::broadcast;

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
            let mut tables = changes
                .iter()
                .map(|change| change.table.clone())
                .collect::<Vec<_>>();
            tables.sort_unstable();
            tables.dedup();
            html_rt::publish_table_changes(tables);
            let _ = self.sender.send(DbChangeSet { changes });
        }
    }
}

pub fn event_bus() -> &'static DbEventBus {
    static BUS: OnceLock<DbEventBus> = OnceLock::new();
    BUS.get_or_init(|| DbEventBus::new(256))
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

        // SQLx maps `true` to SQLite's zero return value, which allows the
        // commit. The hook runs before sqlite3_step() has fully completed, so
        // defer publication until the committed rows are visible to readers.
        if !changes.is_empty() {
            commit_runtime.spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                event_bus().publish(changes);
            });
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
