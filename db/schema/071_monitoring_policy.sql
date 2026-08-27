CREATE TABLE monitoring_policy (
    organization_id INTEGER PRIMARY KEY REFERENCES organization(id) ON DELETE CASCADE,
    desired_agent_version TEXT,
    retention_days INTEGER NOT NULL DEFAULT 7 CHECK (retention_days BETWEEN 1 AND 3650),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;
