-- Environments group related services inside a project
CREATE TABLE environments (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	description TEXT,
	env_var TEXT NOT NULL DEFAULT '',
	is_default INTEGER NOT NULL DEFAULT 0,
	project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Trigger Function
CREATE TRIGGER environments_updated_at
AFTER UPDATE ON environments
FOR EACH ROW
BEGIN
	UPDATE environments
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;