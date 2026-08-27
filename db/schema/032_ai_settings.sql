CREATE TABLE ai_settings (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	api_url TEXT NOT NULL,
	api_key TEXT NOT NULL,
	model TEXT NOT NULL,
	is_enabled INTEGER NOT NULL DEFAULT 1,
	-- Foreign keys
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT ai_enabled_check CHECK (is_enabled IN (0, 1))
) STRICT;

CREATE UNIQUE INDEX idx_ai_settings_org_name
ON ai_settings(organization_id, name COLLATE NOCASE);

CREATE INDEX idx_ai_settings_organization_id ON ai_settings(organization_id);

CREATE TRIGGER ai_settings_updated_at
AFTER UPDATE ON ai_settings
FOR EACH ROW
BEGIN
	UPDATE ai_settings
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
