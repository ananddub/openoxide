CREATE TABLE IF NOT EXISTS vault_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL, -- 'HASHICORP', 'INFISICAL', 'DOPPLER', 'AWS'
    api_url TEXT NOT NULL,       -- e.g. https://vault.example.com:8200
    auth_token TEXT NOT NULL,    -- Token / API key
    namespace TEXT,              -- Vault namespace (HashiCorp) / Project slug (Infisical)
    config_json TEXT,            -- Additional provider configuration (JSON)
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX IF NOT EXISTS idx_vault_providers_org ON vault_providers(organization_id);
