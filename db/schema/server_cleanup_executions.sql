CREATE TABLE server_cleanup_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id INTEGER NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'RUNNING' CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED')),
    policy TEXT NOT NULL,
    stdout TEXT,
    stderr TEXT,
    started_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    finished_at INTEGER
);
CREATE INDEX idx_server_cleanup_executions_server
ON server_cleanup_executions(server_id, started_at DESC);
