CREATE TABLE groups (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	-- is_system: 1 = ADMIN, MEMBER (locked, cannot edit/delete)
	-- is_system: 0 = custom groups (owner can edit/delete)
	is_system INTEGER NOT NULL DEFAULT 0,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

CREATE TABLE policy (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	action TEXT NOT NULL UNIQUE,
	-- Unique action name (e.g. 'read:containers', 'write:users')
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

CREATE TABLE group_policy (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	group_id INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
	policy_id INTEGER NOT NULL REFERENCES policy(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Trigger Function
CREATE TRIGGER groups_updated_at
AFTER UPDATE ON groups
FOR EACH ROW
BEGIN
	UPDATE groups
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;