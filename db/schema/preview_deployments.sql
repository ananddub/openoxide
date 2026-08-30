CREATE TABLE preview_deployments (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    base_service_id BIGINT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    preview_service_id BIGINT REFERENCES services(id) ON DELETE SET NULL,
    provider_type TEXT NOT NULL,
    provider_id BIGINT NOT NULL,
    owner TEXT NOT NULL,
    repository TEXT NOT NULL,
    pull_request_number TEXT NOT NULL,
    source_branch TEXT NOT NULL,
    target_branch TEXT NOT NULL,
    commit_sha TEXT,
    author TEXT,
    status TEXT NOT NULL,
    domain TEXT NOT NULL,
    last_deployment_id BIGINT REFERENCES deployments(id) ON DELETE SET NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    source_owner TEXT,
    source_repository TEXT
);

CREATE INDEX idx_preview_deployments_base_service_id ON preview_deployments(base_service_id);
CREATE INDEX idx_preview_deployments_preview_service_id ON preview_deployments(preview_service_id);
