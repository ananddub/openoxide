CREATE TABLE organization (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL UNIQUE,
	logo TEXT,
	slug TEXT NOT NULL UNIQUE,
	owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

CREATE TABLE organization_members (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	role TEXT DEFAULT 'MEMBER',
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL,
	CONSTRAINT role_check CHECK (role IN ('ADMIN', 'MEMBER'))
) STRICT;

CREATE TABLE organization_invites (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	email TEXT NOT NULL,
	role TEXT DEFAULT 'MEMBER',
	status TEXT DEFAULT 'PENDING',
	token TEXT NOT NULL UNIQUE,
	group_id INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	invited_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	expired_at INTEGER NOT NULL,
	created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL,
	CONSTRAINT role_check CHECK (role IN ('ADMIN', 'MEMBER')),
	CONSTRAINT status_check CHECK (status IN ('PENDING', 'ACCEPTED', 'REJECTED'))
) STRICT;

-- Trigger Function
CREATE TRIGGER organization_updated_at
AFTER UPDATE ON organization
FOR EACH ROW
BEGIN
	UPDATE organization
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER organization_members_updated_at
AFTER UPDATE ON organization_members
FOR EACH ROW
BEGIN
	UPDATE organization_members
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;