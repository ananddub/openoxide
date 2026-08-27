CREATE TABLE dns_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    credentials_json TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
