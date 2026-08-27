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
, acknowledged_at INTEGER, acknowledged_by INTEGER, silenced_until INTEGER, resolved_at INTEGER, notification_correlation_id TEXT) STRICT;

CREATE INDEX idx_alert_events_org_created ON alert_events(organization_id, created_at DESC);

CREATE INDEX idx_alert_events_rule_target_created
    ON alert_events(alert_rule_id, target_key, created_at DESC);
