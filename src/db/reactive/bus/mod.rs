use std::sync::OnceLock;
use tokio::sync::broadcast;

use crate::db::reactive::types::{DbChangeEvent, DbChangeSet};

static DB_POOL: OnceLock<sqlx::SqlitePool> = OnceLock::new();

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

    pub fn publish(&self, changes: Vec<DbChangeEvent>) {
        if !changes.is_empty() {
            tracing::info!(
                tables = ?changes.iter().map(|c| c.table.as_str()).collect::<Vec<_>>(),
                "sqlite realtime change committed"
            );
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

pub(crate) async fn publish_when_visible(changes: Vec<DbChangeEvent>) {
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
