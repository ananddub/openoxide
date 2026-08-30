CREATE TABLE mounts (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    mount_type TEXT NOT NULL,
    service_type TEXT NOT NULL,
    host_path TEXT,
    volume_name TEXT,
    file_path TEXT,
    content TEXT,
    mount_path TEXT NOT NULL,
    service_id BIGINT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_mounts_service_id ON mounts(service_id);
