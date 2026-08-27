CREATE TABLE monitoring_maintenance_windows (
    id INTEGER NOT NULL PRIMARY KEY,
    organization_id INTEGER NOT NULL,
    server_id INTEGER,
    starts_at INTEGER NOT NULL,
    ends_at INTEGER NOT NULL,
    reason TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
