CREATE TABLE vault_providers (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    api_url TEXT NOT NULL,
    auth_token TEXT NOT NULL,
    namespace TEXT,
    config_json TEXT,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);
