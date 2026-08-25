CREATE TABLE resource_access (
	id            INTEGER PRIMARY KEY AUTOINCREMENT,
	user_id       INTEGER REFERENCES users(id) ON DELETE CASCADE,
	org_id        INTEGER REFERENCES organization(id) ON DELETE CASCADE,
	resource_type TEXT,
	resource_id   INTEGER,
	created_at    INTEGER DEFAULT (strftime('%s', 'now')),
	CONSTRAINT resource_type_check CHECK (
		resource_type IN ('PROJECT', 'SERVER', 'ENVIRONMENT', 'SERVICE', 'GIT_PROVIDER')
	)
) STRICT;
CREATE UNIQUE INDEX idx_resource_access_unique
ON resource_access(user_id, org_id, resource_type, resource_id);
