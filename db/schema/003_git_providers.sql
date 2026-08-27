CREATE TABLE git_providers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- provider_type: github | gitlab | gitea | bitbucket
	provider_type TEXT NOT NULL DEFAULT 'GITHUB',
	-- Share provider across all users (single-tenant: always true)
	shared INTEGER NOT NULL DEFAULT 1,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL, webhook_secret TEXT,
	CONSTRAINT git_provider_type_check CHECK (
		provider_type IN ('GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET')
	)
) STRICT;

CREATE INDEX idx_git_providers_webhook_secret
ON git_providers(webhook_secret)
WHERE webhook_secret IS NOT NULL;

CREATE TRIGGER git_providers_updated_at
AFTER UPDATE ON git_providers
FOR EACH ROW
BEGIN
	UPDATE git_providers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
