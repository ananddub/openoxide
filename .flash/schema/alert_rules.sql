CREATE TABLE alert_rules (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id INTEGER NOT NULL,
    metric_name TEXT NOT NULL,
    operator TEXT NOT NULL,
    threshold REAL NOT NULL,
    duration_seconds INTEGER NOT NULL,
    notification_channel TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
