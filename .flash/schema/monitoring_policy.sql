CREATE TABLE monitoring_policy (
    organization_id INTEGER NOT NULL PRIMARY KEY,
    desired_agent_version TEXT,
    retention_days INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
