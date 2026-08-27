CREATE TABLE notification_resource_bindings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL CHECK (resource_type IN ('SERVER', 'APPLICATION', 'COMPOSE', 'DATABASE')),
    resource_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    UNIQUE(notification_id, resource_type, resource_id)
) STRICT;
