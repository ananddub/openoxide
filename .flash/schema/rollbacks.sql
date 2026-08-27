CREATE TABLE rollbacks (
    id INTEGER NOT NULL PRIMARY KEY,
    deployment_id INTEGER NOT NULL,
    version INTEGER NOT NULL,
    image TEXT,
    full_context TEXT,
    created_at INTEGER NOT NULL
);
