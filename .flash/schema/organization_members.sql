CREATE TABLE organization_members (
    id INTEGER NOT NULL PRIMARY KEY,
    role TEXT,
    user_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    group_id INTEGER
);
