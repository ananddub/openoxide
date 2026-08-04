ALTER TABLE server_metrics ADD COLUMN server_id INTEGER NOT NULL DEFAULT 1 REFERENCES servers(id) ON DELETE CASCADE;
CREATE INDEX IF NOT EXISTS idx_server_metrics_server_timestamp ON server_metrics(server_id, timestamp DESC);

ALTER TABLE container_metrics ADD COLUMN server_id INTEGER NOT NULL DEFAULT 1 REFERENCES servers(id) ON DELETE CASCADE;
ALTER TABLE container_metrics ADD COLUMN application_id INTEGER;
ALTER TABLE container_metrics ADD COLUMN compose_id INTEGER;
CREATE INDEX IF NOT EXISTS idx_container_metrics_server_container_timestamp
    ON container_metrics(server_id, container_id, timestamp DESC);

CREATE TABLE monitoring_agents (
    server_id INTEGER PRIMARY KEY REFERENCES servers(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    last_seen_at INTEGER,
    agent_version TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE TABLE alert_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alert_rule_id INTEGER NOT NULL REFERENCES alert_rules(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    target_key TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('FIRING', 'RESOLVED', 'NO_DATA')),
    value REAL,
    threshold REAL,
    message TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE INDEX idx_alert_events_org_created ON alert_events(organization_id, created_at DESC);
