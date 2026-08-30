CREATE TABLE schedule_execution_locks (
    schedule_id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    owner_id TEXT NOT NULL,
    acquired_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL
);
