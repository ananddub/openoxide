CREATE TABLE alert_rules (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id BIGINT NOT NULL,
    metric_name TEXT NOT NULL,
    operator TEXT NOT NULL,
    threshold REAL NOT NULL,
    duration_seconds BIGINT NOT NULL,
    notification_channel TEXT NOT NULL,
    enabled BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
