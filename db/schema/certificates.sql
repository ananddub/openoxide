CREATE TABLE certificates (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    certificate_data TEXT NOT NULL,
    private_key TEXT NOT NULL,
    certificate_path TEXT NOT NULL,
    auto_renew BIGINT NOT NULL,
    server_id BIGINT,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
