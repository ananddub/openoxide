CREATE TABLE IF NOT EXISTS "alert_rules" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 255),
    target_type TEXT NOT NULL CHECK (target_type IN ('SERVER', 'APPLICATION', 'DATABASE', 'COMPOSE')),
    target_id INTEGER NOT NULL CHECK (target_id >= 0),
    metric_name TEXT NOT NULL CHECK (metric_name IN ('CPU', 'MEMORY', 'DISK')),
    operator TEXT NOT NULL CHECK (operator IN ('GT', 'GTE', 'LT', 'LTE', 'EQ')),
    threshold REAL NOT NULL CHECK (threshold >= 0.0 AND threshold <= 100.0),
    duration_seconds INTEGER NOT NULL DEFAULT 60 CHECK (duration_seconds BETWEEN 0 AND 86400),
    notification_channel TEXT NOT NULL DEFAULT 'SYSTEM' CHECK (notification_channel = 'SYSTEM'),
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;
CREATE INDEX idx_alert_rules_target ON alert_rules(target_type, target_id);
CREATE INDEX idx_alert_rules_organization ON alert_rules(organization_id);
