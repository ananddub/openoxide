CREATE TABLE monitoring_maintenance_windows (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    organization_id BIGINT NOT NULL,
    server_id BIGINT,
    starts_at BIGINT NOT NULL,
    ends_at BIGINT NOT NULL,
    reason TEXT NOT NULL,
    created_at BIGINT NOT NULL
);
