CREATE TABLE gitea_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    gitea_url TEXT NOT NULL,
    gitea_internal_url TEXT,
    redirect_uri TEXT,
    client_id TEXT,
    client_secret TEXT,
    access_token TEXT,
    refresh_token TEXT,
    expires_at INTEGER,
    scopes TEXT,
    last_authenticated_at INTEGER,
    git_provider_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
