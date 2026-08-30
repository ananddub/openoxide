CREATE TABLE application_middlewares (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    service_id BIGINT NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    middleware_type TEXT NOT NULL,
    enabled BIGINT NOT NULL,
    config TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX idx_application_middlewares_service_id ON application_middlewares(service_id);
