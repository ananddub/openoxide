-- Make the persisted alert contract match the Rust/OpenAPI enums. SQLite cannot
-- add CHECK constraints in-place, so rebuild the table while preserving rows.
PRAGMA foreign_keys = OFF;

CREATE TABLE alert_rules_strict (
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

INSERT INTO alert_rules_strict (
    id, name, target_type, target_id, metric_name, operator, threshold,
    duration_seconds, notification_channel, enabled, organization_id,
    created_at, updated_at
)
SELECT
    id, trim(name),
    CASE upper(trim(target_type))
        WHEN 'APP' THEN 'APPLICATION'
        WHEN 'DB' THEN 'DATABASE'
        ELSE upper(trim(target_type))
    END,
    target_id,
    CASE upper(trim(metric_name))
        WHEN 'CPU_PERCENT' THEN 'CPU'
        WHEN 'MEM' THEN 'MEMORY'
        WHEN 'MEMORY_PERCENT' THEN 'MEMORY'
        WHEN 'RAM' THEN 'MEMORY'
        WHEN 'DISK_PERCENT' THEN 'DISK'
        WHEN 'STORAGE' THEN 'DISK'
        ELSE upper(trim(metric_name))
    END,
    CASE upper(trim(operator))
        WHEN '>' THEN 'GT'
        WHEN 'GREATER_THAN' THEN 'GT'
        WHEN '>=' THEN 'GTE'
        WHEN 'GREATER_OR_EQUAL' THEN 'GTE'
        WHEN '<' THEN 'LT'
        WHEN 'LESS_THAN' THEN 'LT'
        WHEN '<=' THEN 'LTE'
        WHEN 'LESS_OR_EQUAL' THEN 'LTE'
        WHEN '=' THEN 'EQ'
        WHEN '==' THEN 'EQ'
        WHEN 'EQUAL' THEN 'EQ'
        ELSE upper(trim(operator))
    END,
    threshold, duration_seconds, 'SYSTEM',
    CASE WHEN enabled = 0 THEN 0 ELSE 1 END, organization_id,
    created_at, updated_at
FROM alert_rules;

DROP TABLE alert_rules;
ALTER TABLE alert_rules_strict RENAME TO alert_rules;

CREATE INDEX idx_alert_rules_target ON alert_rules(target_type, target_id);
CREATE INDEX idx_alert_rules_organization ON alert_rules(organization_id);
CREATE INDEX IF NOT EXISTS idx_alert_events_rule_target_created
    ON alert_events(alert_rule_id, target_key, created_at DESC);

PRAGMA foreign_keys = ON;
