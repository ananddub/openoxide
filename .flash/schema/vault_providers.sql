CREATE TABLE vault_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    api_url TEXT NOT NULL,
    auth_token TEXT NOT NULL,
    namespace TEXT,
    config_json TEXT,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
