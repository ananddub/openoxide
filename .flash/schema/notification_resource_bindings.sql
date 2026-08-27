CREATE TABLE notification_resource_bindings (
    id INTEGER NOT NULL PRIMARY KEY,
    notification_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
