CREATE TABLE redirects (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    regex TEXT NOT NULL,
    replacement TEXT NOT NULL,
    permanent BIGINT NOT NULL,
    cfg_key BIGINT,
    service_id BIGINT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_redirects_service_id ON redirects(service_id);
