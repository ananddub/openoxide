CREATE TABLE destinations (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider TEXT NOT NULL,
    access_key TEXT NOT NULL,
    secret_access_key TEXT NOT NULL,
    bucket TEXT NOT NULL,
    region TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
