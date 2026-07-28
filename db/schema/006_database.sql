-- PostgreSQL managed instances
CREATE TABLE postgres_dbs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	docker_image TEXT NOT NULL DEFAULT 'postgres:18',
	database_name TEXT NOT NULL,
	database_user TEXT NOT NULL,
	database_password TEXT NOT NULL,
	external_port INTEGER,
	command TEXT,
	args TEXT,	-- JSON array
	env_var TEXT,
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	-- Swarm configs (JSON)
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	network_ids TEXT NOT NULL DEFAULT '[]',
	detach_rustploy_network INTEGER NOT NULL DEFAULT 0,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT pg_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED'))
) STRICT;

-- MySQL managed instances
CREATE TABLE mysql_dbs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	docker_image TEXT NOT NULL DEFAULT 'mysql:9',
	database_name TEXT NOT NULL,
	database_user TEXT NOT NULL,
	database_password TEXT NOT NULL,
	database_root_password TEXT NOT NULL,
	external_port INTEGER,
	command TEXT,
	args TEXT,	-- JSON array
	env_var TEXT,
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	network_ids TEXT NOT NULL DEFAULT '[]',
	detach_rustploy_network INTEGER NOT NULL DEFAULT 0,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT mysql_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED'))
) STRICT;

-- MariaDB managed instances
CREATE TABLE mariadb_dbs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	docker_image TEXT NOT NULL DEFAULT 'mariadb:13',
	database_name TEXT NOT NULL,
	database_user TEXT NOT NULL,
	database_password TEXT NOT NULL,
	database_root_password TEXT NOT NULL,
	external_port INTEGER,
	command TEXT,
	args TEXT,	-- JSON array
	env_var TEXT,
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	network_ids TEXT NOT NULL DEFAULT '[]',
	detach_rustploy_network INTEGER NOT NULL DEFAULT 0,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT mariadb_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED'))
) STRICT;

-- MongoDB managed instances
CREATE TABLE mongo_dbs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	docker_image TEXT NOT NULL DEFAULT 'mongo:8',
	database_user TEXT NOT NULL,
	database_password TEXT NOT NULL,
	external_port INTEGER,
	replica_sets INTEGER NOT NULL DEFAULT 0,
	command TEXT,
	args TEXT,	-- JSON array
	env_var TEXT,
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	network_ids TEXT NOT NULL DEFAULT '[]',
	detach_rustploy_network INTEGER NOT NULL DEFAULT 0,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT mongo_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED'))
) STRICT;

-- Redis managed instances
CREATE TABLE redis_dbs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	docker_image TEXT NOT NULL DEFAULT 'redis:8',
	database_password TEXT NOT NULL,
	external_port INTEGER,
	command TEXT,
	args TEXT,	-- JSON array
	env_var TEXT,
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	network_ids TEXT NOT NULL DEFAULT '[]',
	detach_rustploy_network INTEGER NOT NULL DEFAULT 0,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT redis_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED'))
) STRICT;

-- LibSQL managed instances
CREATE TABLE libsql_dbs (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	app_name TEXT NOT NULL UNIQUE,
	description TEXT,
	docker_image TEXT NOT NULL DEFAULT 'ghcr.io/tursodatabase/libsql-server:v0.24.32',
	database_user TEXT NOT NULL,
	database_password TEXT NOT NULL,
	sqld_node TEXT NOT NULL DEFAULT 'PRIMARY', -- primary | replica
	sqld_primary_url TEXT,
	enable_namespaces INTEGER NOT NULL DEFAULT 0,
	external_port INTEGER,
	external_grpc_port INTEGER,
	external_admin_port INTEGER,
	command TEXT,
	args TEXT,	-- JSON array
	env_var TEXT,
	memory_reservation TEXT,
	memory_limit TEXT,
	cpu_reservation TEXT,
	cpu_limit TEXT,
	replicas INTEGER NOT NULL DEFAULT 1,
	app_status TEXT NOT NULL DEFAULT 'IDLE',
	-- Swarm configs (JSON)
	health_check_swarm TEXT,
	restart_policy_swarm TEXT,
	placement_swarm TEXT,
	update_config_swarm TEXT,
	rollback_config_swarm TEXT,
	mode_swarm TEXT,
	labels_swarm TEXT,
	network_swarm TEXT,
	network_ids TEXT NOT NULL DEFAULT '[]',
	detach_rustploy_network INTEGER NOT NULL DEFAULT 0,
	endpoint_spec_swarm TEXT,
	ulimits_swarm TEXT,
	stop_grace_period_swarm INTEGER,
	environment_id INTEGER NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
	server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT libsql_status_check CHECK (app_status IN ('IDLE', 'RUNNING', 'DONE', 'ERROR', 'QUEUED', 'STARTING', 'STOPPED')),
	CONSTRAINT libsql_node_check CHECK (sqld_node IN ('PRIMARY', 'REPLICA'))
) STRICT;

-- Trigger Function
CREATE TRIGGER pg_updated_at
AFTER UPDATE ON postgres_dbs
FOR EACH ROW
BEGIN
	UPDATE postgres_dbs
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER mysql_updated_at
AFTER UPDATE ON mysql_dbs
FOR EACH ROW
BEGIN
	UPDATE mysql_dbs
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER mariadb_updated_at
AFTER UPDATE ON mariadb_dbs
FOR EACH ROW
BEGIN
	UPDATE mariadb_dbs
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER mongo_updated_at
AFTER UPDATE ON mongo_dbs
FOR EACH ROW
BEGIN
	UPDATE mongo_dbs
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER redis_updated_at
AFTER UPDATE ON redis_dbs
FOR EACH ROW
BEGIN
	UPDATE redis_dbs
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

-- Trigger Function
CREATE TRIGGER libsql_updated_at
AFTER UPDATE ON libsql_dbs
FOR EACH ROW
BEGIN
	UPDATE libsql_dbs
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
