CREATE TABLE certificates (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    certificate_data TEXT NOT NULL,
    private_key TEXT NOT NULL,
    certificate_path TEXT NOT NULL,
    auto_renew INTEGER NOT NULL,
    server_id INTEGER,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
