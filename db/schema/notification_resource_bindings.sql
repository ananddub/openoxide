CREATE TABLE notification_resource_bindings (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    notification_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);
