CREATE TABLE projects (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	-- Unique slug used in Docker service names (e.g. 'my-app')
	name TEXT NOT NULL UNIQUE,
	description TEXT,
	env_var TEXT NOT NULL DEFAULT '',
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

CREATE TRIGGER projects_updated_at
AFTER UPDATE ON projects
FOR EACH ROW
BEGIN
	UPDATE projects
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
