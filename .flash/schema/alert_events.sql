CREATE TABLE alert_events (
    id INTEGER NOT NULL PRIMARY KEY,
    alert_rule_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    target_key TEXT NOT NULL,
    state TEXT NOT NULL,
    value REAL,
    threshold REAL,
    message TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    acknowledged_at INTEGER,
    acknowledged_by INTEGER,
    silenced_until INTEGER,
    resolved_at INTEGER,
    notification_correlation_id TEXT
);
