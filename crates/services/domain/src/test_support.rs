//! Shared harness for the domain crate's database-backed tests.
//!
//! Every fixture is a fresh SQLite file built by the same migrations the panel
//! runs, so a service is exercised against the real schema and its constraints
//! instead of a hand-copied `CREATE TABLE` that drifts.

use std::str::FromStr;
use std::sync::Arc;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use crate::db::queries::FlashQueries;

/// A migrated database plus the query layer every service takes.
///
/// File-backed rather than `:memory:`: the pool owns the single connection and
/// a recycle would silently hand the next statement an empty database.
pub(crate) async fn fixture(tag: &str) -> (Arc<FlashQueries>, sqlx::SqlitePool) {
    let path = std::env::temp_dir().join(format!(
        "openoxide-domain-{tag}-{}.sqlite3",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let url = format!("sqlite://{}", path.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(
            SqliteConnectOptions::from_str(&url)
                .unwrap()
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .foreign_keys(true),
        )
        .await
        .unwrap();
    sqlx::migrate!("../../../db/migrations/")
        .run(&pool)
        .await
        .expect("panel migrations should build the schema");
    (Arc::new(FlashQueries::new(Arc::new(pool.clone()))), pool)
}
