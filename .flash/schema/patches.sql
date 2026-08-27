CREATE TABLE patches (
    id INTEGER NOT NULL PRIMARY KEY,
    patch_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    content TEXT NOT NULL,
    application_id INTEGER,
    compose_id INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
