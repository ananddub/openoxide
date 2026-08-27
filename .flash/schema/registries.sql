CREATE TABLE registries (
    id INTEGER NOT NULL PRIMARY KEY,
    registry_name TEXT NOT NULL,
    image_prefix TEXT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    registry_url TEXT NOT NULL,
    registry_type TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
