CREATE TABLE preview_deployments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    base_application_id INTEGER NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
    preview_application_id INTEGER REFERENCES applications(id) ON DELETE SET NULL,
    provider_type TEXT NOT NULL,
    provider_id INTEGER NOT NULL,
    owner TEXT NOT NULL,
    repository TEXT NOT NULL,
    pull_request_number TEXT NOT NULL,
    source_branch TEXT NOT NULL,
    target_branch TEXT NOT NULL,
    commit_sha TEXT,
    author TEXT,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    domain TEXT NOT NULL,
    last_deployment_id INTEGER REFERENCES deployments(id) ON DELETE SET NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    CONSTRAINT preview_deployments_status_check
        CHECK (status IN ('QUEUED', 'ACTIVE', 'ERROR', 'CLOSED')),
    UNIQUE(base_application_id, provider_type, pull_request_number)
) STRICT;

CREATE UNIQUE INDEX idx_preview_deployments_application
    ON preview_deployments(preview_application_id)
    WHERE preview_application_id IS NOT NULL;

CREATE INDEX idx_preview_deployments_base_status
    ON preview_deployments(base_application_id, status);

CREATE INDEX idx_preview_deployments_pull_request
    ON preview_deployments(provider_type, owner, repository, pull_request_number);
