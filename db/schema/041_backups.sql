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

CREATE INDEX idx_backups_compose_id ON backups(compose_id);

CREATE INDEX idx_backups_destination_id ON backups(destination_id);

CREATE INDEX idx_backups_libsql_id ON backups(libsql_id);

CREATE INDEX idx_backups_mariadb_id ON backups(mariadb_id);

CREATE INDEX idx_backups_mongo_id ON backups(mongo_id);

CREATE INDEX idx_backups_mysql_id ON backups(mysql_id);

CREATE INDEX idx_backups_organization_id ON backups(organization_id);

CREATE INDEX idx_backups_postgres_id ON backups(postgres_id);

CREATE INDEX idx_backups_redis_id ON backups(redis_id);

CREATE TRIGGER backups_updated_at
AFTER UPDATE ON backups
FOR EACH ROW
BEGIN
	UPDATE backups
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
