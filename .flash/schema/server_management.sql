CREATE TABLE server_management (
    server_id INTEGER NOT NULL PRIMARY KEY,
    maintenance_mode INTEGER NOT NULL,
    maintenance_message TEXT,
    labels TEXT NOT NULL,
    cleanup_policy TEXT NOT NULL,
    gpu_enabled INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
