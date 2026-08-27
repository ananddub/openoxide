CREATE TABLE domains (
    id INTEGER NOT NULL PRIMARY KEY,
    host TEXT NOT NULL,
    https INTEGER NOT NULL,
    port INTEGER,
    path TEXT,
    internal_path TEXT,
    custom_entrypoint TEXT,
    service_name TEXT,
    custom_cert_resolver TEXT,
    strip_path INTEGER NOT NULL,
    middlewares TEXT NOT NULL,
    domain_type TEXT NOT NULL,
    certificate_type TEXT NOT NULL,
    application_id INTEGER,
    compose_id INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
