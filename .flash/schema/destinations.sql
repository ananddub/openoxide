CREATE TABLE destinations (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL,
    access_key TEXT NOT NULL,
    secret_access_key TEXT NOT NULL,
    bucket TEXT NOT NULL,
    region TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
