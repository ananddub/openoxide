CREATE TABLE backup_executions (
    id INTEGER NOT NULL PRIMARY KEY,
    backup_kind TEXT NOT NULL,
    operation TEXT NOT NULL,
    backup_id INTEGER,
    status TEXT NOT NULL,
    object_key TEXT,
    checksum_sha256 TEXT,
    size_bytes INTEGER,
    attempt INTEGER NOT NULL,
    error TEXT,
    started_at INTEGER NOT NULL,
    finished_at INTEGER
);
