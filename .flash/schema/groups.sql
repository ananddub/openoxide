CREATE TABLE groups (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    is_system INTEGER NOT NULL,
    organization_id INTEGER
);
