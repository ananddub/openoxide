CREATE TABLE domains (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    host TEXT NOT NULL,
    https BIGINT NOT NULL,
    port BIGINT,
    path TEXT,
    internal_path TEXT,
    custom_entrypoint TEXT,
    service_name TEXT,
    custom_cert_resolver TEXT,
    strip_path BIGINT NOT NULL,
    middlewares TEXT NOT NULL,
    domain_type TEXT NOT NULL,
    certificate_type TEXT NOT NULL,
    service_id BIGINT REFERENCES services(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_domains_service_id ON domains(service_id);
