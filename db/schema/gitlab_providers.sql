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
CREATE TRIGGER gitlab_providers_updated_at
AFTER UPDATE ON gitlab_providers
FOR EACH ROW
BEGIN
	UPDATE gitlab_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
