CREATE TABLE server_cleanup_executions (
    id INTEGER NOT NULL PRIMARY KEY,
    server_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    policy TEXT NOT NULL,
    stdout TEXT,
    stderr TEXT,
    started_at INTEGER NOT NULL,
    finished_at INTEGER
);
