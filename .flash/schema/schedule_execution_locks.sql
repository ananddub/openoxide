CREATE TABLE schedule_execution_locks (
    schedule_id INTEGER NOT NULL PRIMARY KEY,
    owner_id TEXT NOT NULL,
    acquired_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);
