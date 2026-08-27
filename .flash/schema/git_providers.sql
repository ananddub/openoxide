CREATE TABLE git_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    shared INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    webhook_secret TEXT
);
