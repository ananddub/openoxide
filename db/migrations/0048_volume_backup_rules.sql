DROP TRIGGER IF EXISTS volume_backups_updated_at;

CREATE TABLE volume_backups_new (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	volume_name TEXT NOT NULL,
	prefix TEXT NOT NULL,
	service_type TEXT NOT NULL DEFAULT 'APPLICATION',
	app_name TEXT NOT NULL,
	service_name TEXT,
	turn_off INTEGER NOT NULL DEFAULT 0,
	cron_expression TEXT NOT NULL,
	keep_latest_count INTEGER,
	enabled INTEGER NOT NULL DEFAULT 1,
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
	CONSTRAINT volume_backup_service_check CHECK (
		service_type IN ('APPLICATION', 'COMPOSE', 'POSTGRES', 'MYSQL', 'MARIADB', 'MONGO', 'REDIS', 'LIBSQL')
	)
) STRICT;

INSERT INTO volume_backups_new SELECT * FROM volume_backups;
DROP TABLE volume_backups;
ALTER TABLE volume_backups_new RENAME TO volume_backups;

CREATE INDEX idx_volume_backups_destination_id ON volume_backups(destination_id);
CREATE INDEX idx_volume_backups_organization_id ON volume_backups(organization_id);
CREATE INDEX idx_volume_backups_application_id ON volume_backups(application_id);
CREATE INDEX idx_volume_backups_compose_id ON volume_backups(compose_id);

CREATE TRIGGER volume_backups_updated_at
AFTER UPDATE ON volume_backups
FOR EACH ROW
BEGIN
	UPDATE volume_backups SET updated_at = strftime('%s', 'now') WHERE id = OLD.id;
END;
