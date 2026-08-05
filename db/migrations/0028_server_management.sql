CREATE TABLE server_management (
    server_id INTEGER PRIMARY KEY REFERENCES servers(id) ON DELETE CASCADE,
    maintenance_mode INTEGER NOT NULL DEFAULT 0 CHECK (maintenance_mode IN (0, 1)),
    maintenance_message TEXT,
    labels TEXT NOT NULL DEFAULT '{}',
    cleanup_policy TEXT NOT NULL DEFAULT '{"containers":true,"images":true,"networks":true,"volumes":false,"packages":false}',
    gpu_enabled INTEGER NOT NULL DEFAULT 0 CHECK (gpu_enabled IN (0, 1)),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

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
