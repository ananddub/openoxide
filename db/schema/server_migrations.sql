CREATE TABLE server_migrations (
    id TEXT NOT NULL PRIMARY KEY,
    source_server_id BIGINT NOT NULL,
    target_server_id BIGINT NOT NULL,
    status TEXT NOT NULL,
    service_ids TEXT NOT NULL,
    build_service_ids TEXT NOT NULL,
    certificate_ids TEXT NOT NULL,
    schedule_ids TEXT NOT NULL,
    queued_services BIGINT NOT NULL,
    queued_total BIGINT NOT NULL,
    error TEXT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
