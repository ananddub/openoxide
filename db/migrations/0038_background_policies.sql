CREATE TABLE IF NOT EXISTS background_policies (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    panel_backup_cron TEXT NOT NULL DEFAULT '0 3 * * *',
    log_cleanup_cron TEXT NOT NULL DEFAULT '0 0 * * *',
    log_retention_days INTEGER NOT NULL DEFAULT 30 CHECK (log_retention_days BETWEEN 1 AND 3650),
    panel_backup_enabled INTEGER NOT NULL DEFAULT 1 CHECK (panel_backup_enabled IN (0, 1)),
    log_cleanup_enabled INTEGER NOT NULL DEFAULT 1 CHECK (log_cleanup_enabled IN (0, 1)),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
INSERT OR IGNORE INTO background_policies (id) VALUES (1);
