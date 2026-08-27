CREATE TABLE schedule_executions (
    id INTEGER NOT NULL PRIMARY KEY,
    schedule_id INTEGER NOT NULL,
    organization_id INTEGER,
    trigger_kind TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt INTEGER NOT NULL,
    scheduled_at INTEGER NOT NULL,
    started_at INTEGER NOT NULL,
    finished_at INTEGER,
    message TEXT,
    stdout TEXT,
    stderr TEXT
);
