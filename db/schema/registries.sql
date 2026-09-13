CREATE TABLE registries (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    registry_name TEXT NOT NULL,
    image_prefix TEXT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    registry_url TEXT NOT NULL,
    registry_type TEXT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
