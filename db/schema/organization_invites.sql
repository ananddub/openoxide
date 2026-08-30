CREATE TABLE organization_invites (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL,
    role TEXT,
    status TEXT,
    token TEXT NOT NULL,
    group_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    invited_by BIGINT NOT NULL,
    expired_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);
