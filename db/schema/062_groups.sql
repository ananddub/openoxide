CREATE TABLE groups (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	-- Unique group name (e.g. 'admin', 'devops')
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
, is_system INTEGER NOT NULL DEFAULT 0, organization_id INTEGER REFERENCES organization(id) ON DELETE CASCADE) STRICT;

CREATE INDEX idx_groups_organization ON groups(organization_id);

CREATE TRIGGER groups_updated_at
AFTER UPDATE ON groups
FOR EACH ROW
BEGIN
	UPDATE groups
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
