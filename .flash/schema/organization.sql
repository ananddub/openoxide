CREATE TABLE organization (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    logo TEXT,
    slug TEXT NOT NULL,
    owner_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
