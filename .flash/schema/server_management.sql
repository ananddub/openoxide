CREATE TABLE server_management (
    server_id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    maintenance_mode BIGINT NOT NULL,
    maintenance_message TEXT,
    labels TEXT NOT NULL,
    cleanup_policy TEXT NOT NULL,
    gpu_enabled BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
