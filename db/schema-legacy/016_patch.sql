CREATE TABLE patches (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	-- patch_type: CREATE | UPDATE | DELETE
	patch_type TEXT NOT NULL DEFAULT 'UPDATE',
	file_path TEXT NOT NULL,
	enabled INTEGER NOT NULL DEFAULT 1,
	content TEXT NOT NULL,
	-- Foreign keys
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT patch_type_check CHECK (patch_type IN ('CREATE', 'UPDATE', 'DELETE')),
	UNIQUE(file_path, application_id),
	UNIQUE(file_path, compose_id)
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_patches_application_id ON patches(application_id);
CREATE INDEX idx_patches_compose_id ON patches(compose_id);

-- Trigger Function
CREATE TRIGGER patches_updated_at
AFTER UPDATE ON patches
FOR EACH ROW
BEGIN
	UPDATE patches
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;