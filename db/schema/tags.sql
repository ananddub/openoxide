CREATE TABLE tags (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);
