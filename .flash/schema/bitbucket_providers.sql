CREATE TABLE bitbucket_providers (
    id INTEGER NOT NULL PRIMARY KEY,
    bitbucket_username TEXT,
    bitbucket_email TEXT,
    app_password TEXT,
    api_token TEXT,
    bitbucket_workspace_name TEXT,
    git_provider_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
