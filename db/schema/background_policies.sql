CREATE TABLE background_policies (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    panel_backup_cron TEXT NOT NULL,
    log_cleanup_cron TEXT NOT NULL,
    log_retention_days BIGINT NOT NULL,
    panel_backup_enabled BIGINT NOT NULL,
    log_cleanup_enabled BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
