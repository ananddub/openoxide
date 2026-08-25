CREATE TABLE mounts (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	mount_type TEXT NOT NULL DEFAULT 'VOLUME',
	service_type TEXT NOT NULL DEFAULT 'APPLICATION',
	host_path TEXT,
	volume_name TEXT,
	file_path TEXT,
	content TEXT,
	mount_path TEXT NOT NULL,
	-- Foreign keys
	postgres_id INTEGER REFERENCES postgres_dbs(id) ON DELETE CASCADE,
	mysql_id INTEGER REFERENCES mysql_dbs(id) ON DELETE CASCADE,
	mariadb_id INTEGER REFERENCES mariadb_dbs(id) ON DELETE CASCADE,
	mongo_id INTEGER REFERENCES mongo_dbs(id) ON DELETE CASCADE,
	redis_id INTEGER REFERENCES redis_dbs(id) ON DELETE CASCADE,
	libsql_id INTEGER REFERENCES libsql_dbs(id) ON DELETE CASCADE,
	compose_id INTEGER REFERENCES compose_projects(id) ON DELETE CASCADE,
	application_id INTEGER REFERENCES applications(id) ON DELETE CASCADE,
	-- Timestamp
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	-- Constraints
	CONSTRAINT mount_type_check CHECK (mount_type IN ('BIND', 'VOLUME', 'FILE')),
	CONSTRAINT mount_service_type_check CHECK (
		service_type IN ('APPLICATION', 'COMPOSE', 'POSTGRES', 'MYSQL', 'MARIADB', 'MONGO', 'REDIS', 'LIBSQL')
	)
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_mounts_application_id ON mounts(application_id);
CREATE INDEX idx_mounts_compose_id ON mounts(compose_id);
CREATE INDEX idx_mounts_postgres_id ON mounts(postgres_id);
CREATE INDEX idx_mounts_mysql_id ON mounts(mysql_id);
CREATE INDEX idx_mounts_mariadb_id ON mounts(mariadb_id);
CREATE INDEX idx_mounts_mongo_id ON mounts(mongo_id);
CREATE INDEX idx_mounts_redis_id ON mounts(redis_id);
CREATE INDEX idx_mounts_libsql_id ON mounts(libsql_id);

-- Trigger Function
CREATE TRIGGER mounts_updated_at
AFTER UPDATE ON mounts
FOR EACH ROW
BEGIN
	UPDATE mounts
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;