CREATE TABLE notification_delivery_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    trigger_name TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'DELIVERED', 'FAILED')),
    attempt INTEGER NOT NULL DEFAULT 1,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    error TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    finished_at INTEGER
) STRICT;
CREATE INDEX idx_notification_attempts_org_created ON notification_delivery_attempts(organization_id, created_at DESC);
CREATE INDEX idx_notification_attempts_retry ON notification_delivery_attempts(status, attempt, created_at);

CREATE TABLE notification_resource_bindings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL CHECK (resource_type IN ('SERVER', 'APPLICATION', 'COMPOSE', 'DATABASE')),
    resource_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    UNIQUE(notification_id, resource_type, resource_id)
) STRICT;
