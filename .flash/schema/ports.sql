CREATE TABLE ports (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    published_port BIGINT NOT NULL,
    target_port BIGINT NOT NULL,
    protocol TEXT NOT NULL,
    publish_mode TEXT NOT NULL,
    service_id BIGINT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL
);

CREATE INDEX idx_ports_service_id ON ports(service_id);
