use crate::{controller::TodoController, models::Todo};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

pub async fn connect() -> SqlitePool {
    let url = std::env::var("TODO_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://crates/todo-test/todo-test.sqlite3".into());
    let options = SqliteConnectOptions::from_str(&url)
        .expect("invalid TODO_DATABASE_URL")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|connection, _| {
            Box::pin(async move {
                let mut handle = connection.lock_handle().await?;
                install_hooks(&mut handle);
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .expect("failed to open todo database");
    sqlx::query("CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, done INTEGER NOT NULL DEFAULT 0)")
        .execute(&pool).await.expect("failed to create todos table");
    spawn_live_refresh(pool.clone());
    pool
}

/// Keeps the `todos` live endpoint synchronized with committed SQLite writes.
/// One process-wide listener performs one refresh query per coalesced commit;
/// Socket.IO then fans that single payload out to every subscribed client.
fn spawn_live_refresh(pool: SqlitePool) {
    tokio::spawn(async move {
        let mut changes = html_rt::subscribe_table_changes();
        while let Ok(tables) = changes.recv().await {
            if !tables.iter().any(|table| table == "todos") {
                continue;
            }

            // Collapse a burst of commits before reading the final visible state.
            while let Ok(next) = changes.try_recv() {
                if !next.iter().any(|table| table == "todos") {
                    continue;
                }
            }

            let todos: Vec<Todo> =
                sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC")
                    .fetch_all(&pool)
                    .await
                    .unwrap_or_default();

            if let Ok(publisher) = TodoController::todos() {
                let _ = publisher.publish(todos).await;
            }
        }
    });
}

fn install_hooks(handle: &mut sqlx::sqlite::LockedSqliteHandle<'_>) {
    let pending = Arc::new(Mutex::new(Vec::<String>::new()));
    let runtime = tokio::runtime::Handle::current();
    let writes = Arc::clone(&pending);
    handle.set_preupdate_hook(move |change| {
        if let Ok(mut tables) = writes.lock() {
            tables.push(change.table.to_owned());
        }
    });
    let commits = Arc::clone(&pending);
    let commit_runtime = runtime.clone();
    handle.set_commit_hook(move || {
        let tables = if let Ok(mut tables) = commits.lock() {
            tables.sort_unstable();
            tables.dedup();
            std::mem::take(&mut *tables)
        } else {
            Vec::new()
        };

        if !tables.is_empty() {
            commit_runtime.spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                html_rt::publish_table_changes(tables);
            });
        }

        true
    });
    handle.set_rollback_hook(move || {
        if let Ok(mut tables) = pending.lock() {
            tables.clear();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Connection, SqliteConnection};
    use std::time::Duration;

    #[tokio::test]
    async fn committed_write_is_visible_before_reactive_notification() {
        let mut connection = SqliteConnection::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE todos (id INTEGER PRIMARY KEY, title TEXT NOT NULL, done INTEGER NOT NULL)",
        )
        .execute(&mut connection)
        .await
        .unwrap();

        {
            let mut handle = connection.lock_handle().await.unwrap();
            install_hooks(&mut handle);
        }

        let mut changes = html_rt::subscribe_table_changes();
        sqlx::query("INSERT INTO todos (id, title, done) VALUES (1, 'test', 0)")
            .execute(&mut connection)
            .await
            .unwrap();

        let tables = tokio::time::timeout(Duration::from_secs(1), changes.recv())
            .await
            .expect("reactive notification timed out")
            .expect("reactive channel closed");
        assert!(tables.iter().any(|table| table == "todos"));

        let done: i64 = sqlx::query_scalar("SELECT done FROM todos WHERE id = 1")
            .fetch_one(&mut connection)
            .await
            .unwrap();
        assert_eq!(done, 0);
    }
}
