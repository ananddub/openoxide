CREATE TABLE monitoring_policy (
    organization_id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    desired_agent_version TEXT,
    retention_days BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
