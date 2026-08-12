use crate::core::config::Config;
use auto_di::singleton;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

#[singleton]
pub async fn connect(config: Arc<Config>) -> SqlitePool {
    if let Err(error) = apply_pending_panel_restore(&config.database_url).await {
        persist_failed_restore(&error).await;
        panic!("Failed to apply pending panel restore: {error}");
    }
    let options = SqliteConnectOptions::from_str(config.database_url.as_str())
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(15))
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(25)
        .after_connect(|connection, _metadata| {
            Box::pin(async move {
                let mut handle = connection.lock_handle().await?;
                crate::db::reactive::install_hooks(&mut handle);
                Ok(())
            })
        })
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

async fn persist_failed_restore(error: &str) {
    let paths = crate::utils::paths::openoxide_paths();
    let marker = format!("{}/backups/panel-restore.pending.json", paths.base);
    let Ok(bytes) = tokio::fs::read(&marker).await else {
        return;
    };
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return;
    };
    let Some(restore_id) = value.get("restore_id").and_then(|value| value.as_str()) else {
        return;
    };
    let directory = format!("{}/backups/restore-history", paths.base);
    let _ = tokio::fs::create_dir_all(&directory).await;
    let record = serde_json::json!({ "restore_id": restore_id, "status": "FAILED", "message": error, "updated_at": chrono::Utc::now().timestamp() });
    if let Ok(body) = serde_json::to_vec_pretty(&record) {
        let _ = tokio::fs::write(format!("{directory}/{restore_id}.json"), body).await;
    }
}

#[derive(serde::Deserialize)]
struct PendingPanelRestore {
    staging: String,
    restore_id: String,
}

async fn apply_pending_panel_restore(database_url: &str) -> Result<(), String> {
    let paths = crate::utils::paths::openoxide_paths();
    let marker = format!("{}/backups/panel-restore.pending.json", paths.base);
    let marker_bytes = match tokio::fs::read(&marker).await {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.to_string()),
    };
    let pending: PendingPanelRestore =
        serde_json::from_slice(&marker_bytes).map_err(|error| error.to_string())?;
    let staging = std::path::Path::new(&pending.staging);
    let staged_db = staging.join("db.sqlite3");
    if !staged_db.is_file() {
        return Err("pending restore snapshot is missing db.sqlite3".into());
    }
    let db_path = sqlite_file_path(database_url)?;
    if let Some(parent) = db_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|error| error.to_string())?;
    }
    if db_path.exists() {
        let recovery = format!(
            "{}.pre-restore-{}",
            db_path.to_string_lossy(),
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        tokio::fs::copy(&db_path, recovery)
            .await
            .map_err(|error| error.to_string())?;
    }
    let replacement = db_path.with_extension("restore.tmp");
    tokio::fs::copy(&staged_db, &replacement)
        .await
        .map_err(|error| error.to_string())?;
    for suffix in ["-wal", "-shm"] {
        let sidecar = format!("{}{}", db_path.to_string_lossy(), suffix);
        if let Err(error) = tokio::fs::remove_file(&sidecar).await
            && error.kind() != std::io::ErrorKind::NotFound
        {
            return Err(error.to_string());
        }
    }
    tokio::fs::rename(&replacement, &db_path)
        .await
        .map_err(|error| error.to_string())?;

    let staged_traefik = staging.join("traefik");
    if staged_traefik.is_dir() {
        let destination = format!("{}/traefik", paths.base);
        tokio::fs::create_dir_all(&destination)
            .await
            .map_err(|error| error.to_string())?;
        let executor =
            crate::utils::exec::CommandExecutor::Local(crate::utils::exec::LocalExecutor::new());
        crate::utils::os::OsCli::new(&executor)
            .dir(staged_traefik.to_string_lossy().as_ref())
            .copy_to(&destination)
            .contents_only(true)
            .run()
            .await
            .map_err(|error| error.to_string())?;
    }
    tokio::fs::remove_file(&marker)
        .await
        .map_err(|error| error.to_string())?;
    let history_dir = format!("{}/backups/restore-history", paths.base);
    tokio::fs::create_dir_all(&history_dir)
        .await
        .map_err(|error| error.to_string())?;
    let completed = serde_json::json!({
        "restore_id": pending.restore_id.clone(),
        "status": "APPLIED",
        "message": "panel database and configuration restore applied successfully",
        "updated_at": chrono::Utc::now().timestamp(),
    });
    tokio::fs::write(
        format!("{history_dir}/{}.json", pending.restore_id),
        serde_json::to_vec_pretty(&completed).map_err(|error| error.to_string())?,
    )
    .await
    .map_err(|error| error.to_string())?;
    tracing::info!(staging = %pending.staging, "pending panel restore applied");
    Ok(())
}

fn sqlite_file_path(database_url: &str) -> Result<std::path::PathBuf, String> {
    let raw = database_url
        .strip_prefix("sqlite://")
        .or_else(|| database_url.strip_prefix("sqlite:"))
        .ok_or_else(|| "panel restore only supports file-backed SQLite URLs".to_string())?;
    let raw = raw.split('?').next().unwrap_or(raw);
    if raw.is_empty() || raw == ":memory:" {
        return Err("panel restore requires a file-backed SQLite database".into());
    }
    Ok(std::path::PathBuf::from(raw))
}
