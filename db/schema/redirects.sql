CREATE TABLE redirects (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	regex TEXT NOT NULL,
	replacement TEXT NOT NULL,
	permanent INTEGER NOT NULL DEFAULT 0,
	unique_config_key INTEGER,
	application_id INTEGER NOT NULL REFERENCES applications(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;
CREATE INDEX idx_redirects_application_id ON redirects(application_id);
CREATE TRIGGER redirects_updated_at
AFTER UPDATE ON redirects
FOR EACH ROW
BEGIN
	UPDATE redirects
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
