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

CREATE TRIGGER github_providers_updated_at
AFTER UPDATE ON github_providers
FOR EACH ROW
BEGIN
	UPDATE github_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
