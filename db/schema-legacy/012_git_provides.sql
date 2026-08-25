-- Git provider umbrella record
CREATE TABLE git_providers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- provider_type: github | gitlab | gitea | bitbucket
	provider_type TEXT NOT NULL DEFAULT 'GITHUB',
	-- Share provider across all users (single-tenant: always true)
	shared INTEGER NOT NULL DEFAULT 1,
	webhook_secret TEXT,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT git_provider_type_check CHECK (
		provider_type IN ('GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET')
	)
) STRICT;

CREATE INDEX IF NOT EXISTS idx_git_providers_webhook_secret
ON git_providers(webhook_secret)
WHERE webhook_secret IS NOT NULL;

-- GitHub App credentials
CREATE TABLE github_providers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	github_app_name TEXT,
	github_app_id INTEGER,
	github_client_id TEXT,
	github_client_secret TEXT,
	github_installation_id TEXT,
	github_private_key TEXT,
	github_webhook_secret TEXT,
	git_provider_id INTEGER NOT NULL REFERENCES git_providers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- GitLab OAuth credentials
CREATE TABLE gitlab_providers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	gitlab_url TEXT NOT NULL DEFAULT 'https://gitlab.com',
	gitlab_internal_url TEXT,
	application_id TEXT,
	redirect_uri TEXT,
	secret TEXT,
	access_token TEXT,
	refresh_token TEXT,
	group_name TEXT,
	expires_at INTEGER,
	git_provider_id INTEGER NOT NULL REFERENCES git_providers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Gitea OAuth credentials
CREATE TABLE gitea_providers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	gitea_url TEXT NOT NULL DEFAULT 'https://gitea.com',
	gitea_internal_url TEXT,
	redirect_uri TEXT,
	client_id TEXT,
	client_secret TEXT,
	access_token TEXT,
	refresh_token TEXT,
	expires_at INTEGER,
	scopes TEXT DEFAULT 'repo,repo:status,read:user,read:org',
	last_authenticated_at INTEGER,
	git_provider_id INTEGER NOT NULL REFERENCES git_providers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Bitbucket App Password credentials
CREATE TABLE bitbucket_providers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	bitbucket_username TEXT,
	bitbucket_email TEXT,
	app_password TEXT,
	api_token TEXT,
	bitbucket_workspace_name TEXT,
	git_provider_id INTEGER NOT NULL REFERENCES git_providers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Trigger Function
CREATE TRIGGER git_providers_updated_at
AFTER UPDATE ON git_providers
FOR EACH ROW
BEGIN
	UPDATE git_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER github_providers_updated_at
AFTER UPDATE ON github_providers
FOR EACH ROW
BEGIN
	UPDATE github_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER gitlab_providers_updated_at
AFTER UPDATE ON gitlab_providers
FOR EACH ROW
BEGIN
	UPDATE gitlab_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER gitea_providers_updated_at
AFTER UPDATE ON gitea_providers
FOR EACH ROW
BEGIN
	UPDATE gitea_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER bitbucket_providers_updated_at
AFTER UPDATE ON bitbucket_providers
FOR EACH ROW
BEGIN
	UPDATE bitbucket_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;