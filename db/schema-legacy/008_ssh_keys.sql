CREATE TABLE ssh_keys (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	description TEXT,
	private_key TEXT NOT NULL DEFAULT '',
	public_key TEXT NOT NULL,
	last_used_at INTEGER,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL
) STRICT;

-- Trigger Function
CREATE TRIGGER ssh_keys_updated_at
AFTER UPDATE ON ssh_keys
FOR EACH ROW
BEGIN
	UPDATE ssh_keys
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;