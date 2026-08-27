CREATE TABLE server_migrations (
    id TEXT NOT NULL PRIMARY KEY,
    source_server_id INTEGER NOT NULL,
    target_server_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    application_ids TEXT NOT NULL,
    build_application_ids TEXT NOT NULL,
    compose_ids TEXT NOT NULL,
    certificate_ids TEXT NOT NULL,
    schedule_ids TEXT NOT NULL,
    queued_applications INTEGER NOT NULL,
    queued_compose_projects INTEGER NOT NULL,
    error TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
