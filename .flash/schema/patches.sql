CREATE TABLE patches (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    patch_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    enabled BIGINT NOT NULL,
    content TEXT NOT NULL,
    service_id BIGINT REFERENCES services(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_patches_service_id ON patches(service_id);
