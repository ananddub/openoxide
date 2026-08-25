CREATE TABLE servers (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	description TEXT,
	ip_address TEXT NOT NULL,
	port INTEGER NOT NULL DEFAULT 22,
	username TEXT NOT NULL DEFAULT 'root',
	app_name TEXT NOT NULL UNIQUE,
	-- server_status: active | inactive
	server_status TEXT NOT NULL DEFAULT 'ACTIVE',
	-- server_type: deploy | build
	server_type TEXT NOT NULL DEFAULT 'DEPLOY',
	enable_docker_cleanup INTEGER NOT NULL DEFAULT 0,
	log_cleanup_cron TEXT DEFAULT '0 0 * * *',
	command TEXT NOT NULL DEFAULT '',
	-- JSON: metrics config object { server: {...}, containers: {...} }
	metrics_config TEXT NOT NULL DEFAULT '{}',
	ssh_key_id INTEGER REFERENCES ssh_keys(id) ON DELETE SET NULL,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL, build_memory_limit TEXT, build_cpu_limit TEXT,
	CONSTRAINT server_status_check CHECK (server_status IN ('ACTIVE', 'INACTIVE')),
	CONSTRAINT server_type_check CHECK (server_type IN ('DEPLOY', 'BUILD'))
) STRICT;
CREATE TRIGGER servers_updated_at
AFTER UPDATE ON servers
FOR EACH ROW
BEGIN
	UPDATE servers
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
