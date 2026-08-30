CREATE TABLE resource_access (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id BIGINT,
    org_id BIGINT,
    resource_type TEXT,
    resource_id BIGINT,
    created_at BIGINT
);
