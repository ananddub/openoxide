-- database_networks (managed/external Docker networks selectable by databases)
CREATE TABLE database_networks (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	docker_network_name TEXT NOT NULL,
	description TEXT,
	external INTEGER NOT NULL DEFAULT 1,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	UNIQUE(docker_network_name, server_id)
) STRICT;

CREATE INDEX idx_database_networks_server_id ON database_networks(server_id);

CREATE TRIGGER database_networks_updated_at
AFTER UPDATE ON database_networks
FOR EACH ROW
BEGIN
	UPDATE database_networks
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
