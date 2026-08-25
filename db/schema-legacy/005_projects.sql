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

CREATE TABLE tags (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	color TEXT NOT NULL,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	UNIQUE(name, organization_id)
) STRICT;

CREATE TABLE project_tags (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
	tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
	UNIQUE(project_id, tag_id)
) STRICT;

-- Trigger Function
CREATE TRIGGER projects_updated_at
AFTER UPDATE ON projects
FOR EACH ROW
BEGIN
	UPDATE projects
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;