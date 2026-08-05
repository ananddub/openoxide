CREATE TABLE IF NOT EXISTS monitoring_policy (
    organization_id INTEGER PRIMARY KEY REFERENCES organization(id) ON DELETE CASCADE,
    desired_agent_version TEXT,
    retention_days INTEGER NOT NULL DEFAULT 7 CHECK (retention_days BETWEEN 1 AND 3650),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

ALTER TABLE alert_events ADD COLUMN acknowledged_at INTEGER;
ALTER TABLE alert_events ADD COLUMN acknowledged_by INTEGER;
ALTER TABLE alert_events ADD COLUMN silenced_until INTEGER;
ALTER TABLE alert_events ADD COLUMN resolved_at INTEGER;
ALTER TABLE alert_events ADD COLUMN notification_correlation_id TEXT;

CREATE TABLE IF NOT EXISTS monitoring_maintenance_windows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
    starts_at INTEGER NOT NULL,
    ends_at INTEGER NOT NULL CHECK (ends_at > starts_at),
    reason TEXT NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;
CREATE INDEX IF NOT EXISTS idx_monitoring_windows_scope ON monitoring_maintenance_windows(organization_id, starts_at, ends_at);
