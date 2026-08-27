CREATE TABLE github_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    github_app_name TEXT,
    github_app_id INTEGER,
    github_client_id TEXT,
    github_client_secret TEXT,
    github_installation_id TEXT,
    github_private_key TEXT,
    github_webhook_secret TEXT,
    git_provider_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
