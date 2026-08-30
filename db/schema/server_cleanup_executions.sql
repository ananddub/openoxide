CREATE TABLE server_cleanup_executions (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    server_id BIGINT NOT NULL,
    status TEXT NOT NULL,
    policy TEXT NOT NULL,
    stdout TEXT,
    stderr TEXT,
    started_at BIGINT NOT NULL,
    finished_at BIGINT
);
