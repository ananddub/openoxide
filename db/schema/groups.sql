CREATE TABLE groups (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    is_system BIGINT NOT NULL,
    organization_id BIGINT,
    is_active BIGINT NOT NULL DEFAULT 1
);
