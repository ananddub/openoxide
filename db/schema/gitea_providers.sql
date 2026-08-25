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
CREATE TRIGGER gitea_providers_updated_at
AFTER UPDATE ON gitea_providers
FOR EACH ROW
BEGIN
	UPDATE gitea_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
