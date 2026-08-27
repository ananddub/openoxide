CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
