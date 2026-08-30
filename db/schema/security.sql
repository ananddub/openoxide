CREATE TABLE security (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    service_id BIGINT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL
);

CREATE INDEX idx_security_service_id ON security(service_id);
