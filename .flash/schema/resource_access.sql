CREATE TABLE resource_access (
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER,
    org_id INTEGER,
    resource_type TEXT,
    resource_id INTEGER,
    created_at INTEGER
);
