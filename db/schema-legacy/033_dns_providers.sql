CREATE TABLE IF NOT EXISTS dns_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    credentials_json TEXT NOT NULL,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);
