CREATE TABLE organization (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	logo TEXT,
	slug TEXT NOT NULL UNIQUE,
	owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;
CREATE TRIGGER organization_updated_at
AFTER UPDATE ON organization
FOR EACH ROW
BEGIN
	UPDATE organization
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
