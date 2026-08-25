-- destinations (Storage keys)
CREATE TABLE destinations (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- provider: S3 | R2 | BACKBLAZE | GCS | DO_SPACES
	provider TEXT NOT NULL DEFAULT 'S3',
	access_key TEXT NOT NULL,
	secret_access_key TEXT NOT NULL,
	bucket TEXT NOT NULL,
	region TEXT NOT NULL,
	endpoint TEXT NOT NULL,
	-- Foreign keys (Inline References)
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	-- Timestamp
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT destination_provider_check CHECK (provider IN ('S3', 'R2', 'BACKBLAZE', 'GCS', 'DO_SPACES'))
) STRICT;

-- backups (Backup Jobs)
CREATE TABLE backups (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	-- Unique slug for Docker service name
	app_name TEXT NOT NULL UNIQUE,
	-- Cron expression e.g. '0 2 * * *'
	schedule TEXT NOT NULL,
	enabled INTEGER NOT NULL DEFAULT 1,
	database_name TEXT NOT NULL,
	prefix TEXT NOT NULL,
	service_name TEXT, -- For compose backups
	keep_latest_count INTEGER,
	-- backup_type: DATABASE | COMPOSE
	backup_type TEXT NOT NULL DEFAULT 'DATABASE',
	-- database_type: POSTGRES | MARIADB | MYSQL | MONGO | REDIS | LIBSQL
	database_type TEXT NOT NULL,
	metadata TEXT, -- JSON string for extra config
	-- Foreign keys
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	postgres_id INTEGER REFERENCES postgres_dbs(id) ON DELETE CASCADE,
	mysql_id INTEGER REFERENCES mysql_dbs(id) ON DELETE CASCADE,
	mariadb_id INTEGER REFERENCES mariadb_dbs(id) ON DELETE CASCADE,
	mongo_id INTEGER REFERENCES mongo_dbs(id) ON DELETE CASCADE,
	redis_id INTEGER REFERENCES redis_dbs(id) ON DELETE CASCADE,
	libsql_id INTEGER REFERENCES libsql_dbs(id) ON DELETE CASCADE,
	destination_id INTEGER NOT NULL REFERENCES destinations(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	-- Timestamp
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT backup_type_check CHECK (backup_type IN ('DATABASE', 'COMPOSE')),
	CONSTRAINT backup_db_type_check CHECK (database_type IN ('POSTGRES', 'MARIADB', 'MYSQL', 'MONGO', 'REDIS', 'LIBSQL'))
) STRICT;

CREATE TABLE volume_backups (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	volume_name TEXT NOT NULL,
	prefix TEXT NOT NULL,
	service_type TEXT NOT NULL DEFAULT 'APPLICATION',
	app_name TEXT NOT NULL UNIQUE,
	service_name TEXT,
	turn_off INTEGER NOT NULL DEFAULT 0,
	cron_expression TEXT NOT NULL,
	keep_latest_count INTEGER,
	enabled INTEGER NOT NULL DEFAULT 1,
	-- Foreign keys
	destination_id INTEGER NOT NULL REFERENCES destinations(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	postgres_id INTEGER REFERENCES postgres_dbs(id) ON DELETE CASCADE,
	mysql_id INTEGER REFERENCES mysql_dbs(id) ON DELETE CASCADE,
	mariadb_id INTEGER REFERENCES mariadb_dbs(id) ON DELETE CASCADE,
	mongo_id INTEGER REFERENCES mongo_dbs(id) ON DELETE CASCADE,
	redis_id INTEGER REFERENCES redis_dbs(id) ON DELETE CASCADE,
	libsql_id INTEGER REFERENCES libsql_dbs(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT volume_backup_service_check CHECK (
		service_type IN ('APPLICATION', 'COMPOSE', 'POSTGRES', 'MYSQL', 'MARIADB', 'MONGO', 'REDIS', 'LIBSQL')
	)
) STRICT;

CREATE TABLE backup_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    backup_kind TEXT NOT NULL CHECK (backup_kind IN ('DATABASE', 'VOLUME', 'PANEL', 'COMPOSE_CONFIG')),
    operation TEXT NOT NULL CHECK (operation IN ('BACKUP', 'RESTORE')),
    backup_id INTEGER,
    status TEXT NOT NULL DEFAULT 'RUNNING' CHECK (status IN ('RUNNING', 'SUCCEEDED', 'FAILED')),
    object_key TEXT,
    checksum_sha256 TEXT,
    size_bytes INTEGER,
    attempt INTEGER NOT NULL DEFAULT 1,
    error TEXT,
    started_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    finished_at INTEGER
);

CREATE INDEX idx_backup_executions_job ON backup_executions(backup_kind, backup_id, started_at DESC);
CREATE INDEX idx_backup_executions_status ON backup_executions(status, started_at DESC);

-- Indexes for faster queries
CREATE INDEX idx_destinations_organization_id ON destinations(organization_id);
CREATE INDEX idx_backups_destination_id ON backups(destination_id);
CREATE INDEX idx_backups_organization_id ON backups(organization_id);
CREATE INDEX idx_backups_compose_id ON backups(compose_id);
CREATE INDEX idx_backups_postgres_id ON backups(postgres_id);
CREATE INDEX idx_backups_mysql_id ON backups(mysql_id);
CREATE INDEX idx_backups_mariadb_id ON backups(mariadb_id);
CREATE INDEX idx_backups_mongo_id ON backups(mongo_id);
CREATE INDEX idx_backups_redis_id ON backups(redis_id);
CREATE INDEX idx_backups_libsql_id ON backups(libsql_id);
CREATE INDEX idx_volume_backups_destination_id ON volume_backups(destination_id);
CREATE INDEX idx_volume_backups_organization_id ON volume_backups(organization_id);
CREATE INDEX idx_volume_backups_application_id ON volume_backups(application_id);
CREATE INDEX idx_volume_backups_compose_id ON volume_backups(compose_id);

-- Trigger Functions
CREATE TRIGGER destinations_updated_at
AFTER UPDATE ON destinations
FOR EACH ROW
BEGIN
	UPDATE destinations
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER backups_updated_at
AFTER UPDATE ON backups
FOR EACH ROW
BEGIN
	UPDATE backups
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER volume_backups_updated_at
AFTER UPDATE ON volume_backups
FOR EACH ROW
BEGIN
	UPDATE volume_backups
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;