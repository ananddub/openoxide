CREATE TABLE IF NOT EXISTS server_migrations (
    id TEXT PRIMARY KEY,
    source_server_id INTEGER NOT NULL REFERENCES servers(id) ON DELETE RESTRICT,
    target_server_id INTEGER NOT NULL REFERENCES servers(id) ON DELETE RESTRICT,
    status TEXT NOT NULL CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED', 'ROLLED_BACK')),
    application_ids TEXT NOT NULL DEFAULT '[]',
    build_application_ids TEXT NOT NULL DEFAULT '[]',
    compose_ids TEXT NOT NULL DEFAULT '[]',
    certificate_ids TEXT NOT NULL DEFAULT '[]',
    schedule_ids TEXT NOT NULL DEFAULT '[]',
    queued_applications INTEGER NOT NULL DEFAULT 0,
    queued_compose_projects INTEGER NOT NULL DEFAULT 0,
    error TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX IF NOT EXISTS idx_server_migrations_source_created
    ON server_migrations(source_server_id, created_at DESC);
