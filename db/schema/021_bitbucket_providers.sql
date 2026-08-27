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

CREATE TRIGGER bitbucket_providers_updated_at
AFTER UPDATE ON bitbucket_providers
FOR EACH ROW
BEGIN
	UPDATE bitbucket_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
