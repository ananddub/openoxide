CREATE TABLE application_migrations (
    id TEXT NOT NULL PRIMARY KEY,
    organization_id BIGINT NOT NULL,
    service_id BIGINT NOT NULL,
    source_server_id BIGINT NOT NULL,
    target_server_id BIGINT NOT NULL,
    status TEXT NOT NULL,
    phase TEXT NOT NULL,
    progress BIGINT NOT NULL DEFAULT 0,
    cancel_requested BIGINT NOT NULL DEFAULT 0,
    error TEXT,
    log_path TEXT NOT NULL DEFAULT '',
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_application_migrations_service ON application_migrations(service_id);
CREATE INDEX idx_application_migrations_status ON application_migrations(status);
