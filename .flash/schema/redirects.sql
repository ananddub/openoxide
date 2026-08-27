CREATE TABLE redirects (
    id INTEGER NOT NULL PRIMARY KEY,
    regex TEXT NOT NULL,
    replacement TEXT NOT NULL,
    permanent INTEGER NOT NULL,
    unique_config_key INTEGER,
    application_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
