CREATE TABLE volume_backups (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    volume_name TEXT NOT NULL,
    prefix TEXT NOT NULL,
    service_type TEXT NOT NULL,
    app_name TEXT NOT NULL,
    service_name TEXT,
    turn_off BIGINT NOT NULL,
    cron_expression TEXT NOT NULL,
    keep_latest_count BIGINT,
    enabled BIGINT NOT NULL,
    service_id BIGINT REFERENCES services(id) ON DELETE CASCADE,
    destination_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_volume_backups_service_id ON volume_backups(service_id);
