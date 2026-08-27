CREATE TABLE ports (
    id INTEGER NOT NULL PRIMARY KEY,
    published_port INTEGER NOT NULL,
    target_port INTEGER NOT NULL,
    protocol TEXT NOT NULL,
    publish_mode TEXT NOT NULL,
    application_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
