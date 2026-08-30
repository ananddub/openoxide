CREATE TABLE backup_executions (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    backup_kind TEXT NOT NULL,
    operation TEXT NOT NULL,
    backup_id BIGINT,
    status TEXT NOT NULL,
    object_key TEXT,
    sha256_checksum TEXT,
    size_bytes BIGINT,
    attempt BIGINT NOT NULL,
    error TEXT,
    started_at BIGINT NOT NULL,
    finished_at BIGINT
);
