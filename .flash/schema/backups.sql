CREATE TABLE backups (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    app_name TEXT NOT NULL,
    schedule TEXT NOT NULL,
    enabled BIGINT NOT NULL,
    database_name TEXT NOT NULL,
    prefix TEXT NOT NULL,
    service_name TEXT,
    keep_latest_count BIGINT,
    backup_type TEXT NOT NULL,
    database_type TEXT,
    metadata TEXT,
    service_id BIGINT REFERENCES services(id) ON DELETE CASCADE,
    destination_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_backups_service_id ON backups(service_id);
