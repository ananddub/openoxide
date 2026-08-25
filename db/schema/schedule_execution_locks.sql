CREATE TABLE schedule_execution_locks (
    schedule_id INTEGER PRIMARY KEY REFERENCES schedules(id) ON DELETE CASCADE,
    owner_id TEXT NOT NULL,
    acquired_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
) STRICT;
