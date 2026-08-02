use crate::core::config::Config;
use auto_di::singleton;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

#[singleton]
pub async fn connect(config: Arc<Config>) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(config.database_url.as_str())
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(15))
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(25)
        .connect_with(options)
        .await
        .expect("Failed to connect to SQLite database");

    sqlx::migrate!("./db/migrations/")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Performance Indexes for instant JWT auth check and fast query lookups
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_jwt_tokens_jti ON jwt_tokens(jti);",
        "CREATE INDEX IF NOT EXISTS idx_jwt_tokens_user ON jwt_tokens(user_id);",
        "CREATE INDEX IF NOT EXISTS idx_jwt_tokens_jti_bl ON jwt_tokens(jti, is_blacklist);",
        "CREATE INDEX IF NOT EXISTS idx_environments_project_id ON environments(project_id);",
    ];

    for idx in indexes {
        let _ = sqlx::query(idx).execute(&pool).await;
    }

    tracing::info!("Database connection established in WAL mode with indexes optimized.");
    pool
}
