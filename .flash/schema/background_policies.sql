CREATE TABLE background_policies (
    id INTEGER NOT NULL PRIMARY KEY,
    panel_backup_cron TEXT NOT NULL,
    log_cleanup_cron TEXT NOT NULL,
    log_retention_days INTEGER NOT NULL,
    panel_backup_enabled INTEGER NOT NULL,
    log_cleanup_enabled INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
