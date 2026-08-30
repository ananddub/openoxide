CREATE TABLE organization_members (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    role TEXT,
    user_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    group_id BIGINT
);
