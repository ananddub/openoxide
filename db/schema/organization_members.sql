CREATE TABLE organization_members (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	role TEXT DEFAULT 'MEMBER',
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s','now')) NOT NULL, group_id INTEGER REFERENCES groups(id) ON DELETE SET NULL,
	CONSTRAINT role_check CHECK (role IN ('ADMIN', 'MEMBER'))
) STRICT;
CREATE TRIGGER organization_members_updated_at
AFTER UPDATE ON organization_members
FOR EACH ROW
BEGIN
	UPDATE organization_members
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
CREATE UNIQUE INDEX idx_organization_members_unique
ON organization_members(user_id, organization_id);
CREATE INDEX idx_org_members_group ON organization_members(group_id);
