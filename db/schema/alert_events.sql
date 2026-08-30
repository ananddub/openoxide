CREATE TABLE alert_events (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    alert_rule_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    target_key TEXT NOT NULL,
    state TEXT NOT NULL,
    value REAL,
    threshold REAL,
    message TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    acknowledged_at BIGINT,
    acknowledged_by BIGINT,
    silenced_until BIGINT,
    resolved_at BIGINT,
    notification_correlation_id TEXT
);
