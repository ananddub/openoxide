CREATE TABLE organization_invites (
    id INTEGER NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    role TEXT,
    status TEXT,
    token TEXT NOT NULL,
    group_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    invited_by INTEGER NOT NULL,
    expired_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
