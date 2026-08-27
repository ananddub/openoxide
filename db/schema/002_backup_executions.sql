CREATE TABLE backup_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    backup_kind TEXT NOT NULL CHECK (backup_kind IN ('DATABASE', 'VOLUME', 'PANEL', 'COMPOSE_CONFIG')),
    operation TEXT NOT NULL CHECK (operation IN ('BACKUP', 'RESTORE')),
    backup_id INTEGER,
    status TEXT NOT NULL DEFAULT 'RUNNING' CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED')),
    object_key TEXT,
    checksum_sha256 TEXT,
    size_bytes INTEGER,
    attempt INTEGER NOT NULL DEFAULT 1,
    error TEXT,
    started_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    finished_at INTEGER
);

CREATE INDEX idx_backup_executions_job
ON backup_executions(backup_kind, backup_id, started_at DESC);

CREATE INDEX idx_backup_executions_status
ON backup_executions(status, started_at DESC);
