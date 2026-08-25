-- Docker Registry
-- Registry type: CLOUD | SELF_HOSTED
CREATE TABLE registries (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	registry_name TEXT NOT NULL,
	image_prefix TEXT,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	registry_url TEXT NOT NULL DEFAULT '',
	-- registry_type: cloud | selfHosted
	registry_type TEXT NOT NULL DEFAULT 'CLOUD',
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT registry_type_check CHECK (registry_type IN ('CLOUD', 'SELF_HOSTED'))
) STRICT;

-- Trigger Function
CREATE TRIGGER registries_updated_at
AFTER UPDATE ON registries
FOR EACH ROW
BEGIN
	UPDATE registries
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;