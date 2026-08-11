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
    pool
}

fn install_hooks(handle: &mut sqlx::sqlite::LockedSqliteHandle<'_>) {
    let pending = Arc::new(Mutex::new(Vec::<String>::new()));
    let writes = Arc::clone(&pending);
    handle.set_preupdate_hook(move |change| {
        if let Ok(mut tables) = writes.lock() {
            tables.push(change.table.to_owned());
        }
    });
    let commits = Arc::clone(&pending);
    handle.set_commit_hook(move || {
        if let Ok(mut tables) = commits.lock() {
            tables.sort_unstable();
            tables.dedup();
            html_rt::publish_table_changes(std::mem::take(&mut *tables));
        }
        false
    });
    handle.set_rollback_hook(move || {
        if let Ok(mut tables) = pending.lock() {
            tables.clear();
        }
    });
}
