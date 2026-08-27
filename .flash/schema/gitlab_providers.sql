CREATE TABLE gitlab_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    gitlab_url TEXT NOT NULL,
    gitlab_internal_url TEXT,
    application_id TEXT,
    redirect_uri TEXT,
    secret TEXT,
    access_token TEXT,
    refresh_token TEXT,
    group_name TEXT,
    expires_at INTEGER,
    git_provider_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
