CREATE TABLE schedules (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    cron_expression TEXT NOT NULL,
    app_name TEXT NOT NULL,
    service_name TEXT,
    shell_type TEXT NOT NULL,
    schedule_type TEXT NOT NULL,
    command TEXT NOT NULL,
    script TEXT,
    timezone TEXT,
    enabled BIGINT NOT NULL,
    service_id BIGINT REFERENCES services(id) ON DELETE CASCADE,
    server_id BIGINT,
    organization_id BIGINT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    schedule_action TEXT NOT NULL
);

CREATE INDEX idx_schedules_service_id ON schedules(service_id);
