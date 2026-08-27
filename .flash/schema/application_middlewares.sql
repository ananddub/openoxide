CREATE TABLE application_middlewares (
    id INTEGER NOT NULL PRIMARY KEY,
    application_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    middleware_type TEXT NOT NULL,
    enabled INTEGER NOT NULL,
    config TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
