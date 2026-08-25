CREATE TABLE dns_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL, -- 'CLOUDFLARE', 'ROUTE53', 'DIGITALOCEAN', 'HETZNER'
    credentials_json TEXT NOT NULL, -- JSON string containing provider API tokens/keys
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);
CREATE INDEX idx_dns_providers_org ON dns_providers(organization_id);
