-- Keep the latest actionable operation failure with each managed database.
ALTER TABLE postgres_dbs ADD COLUMN last_error TEXT;
ALTER TABLE mysql_dbs ADD COLUMN last_error TEXT;
ALTER TABLE mariadb_dbs ADD COLUMN last_error TEXT;
ALTER TABLE mongo_dbs ADD COLUMN last_error TEXT;
ALTER TABLE redis_dbs ADD COLUMN last_error TEXT;
ALTER TABLE libsql_dbs ADD COLUMN last_error TEXT;
ALTER TABLE applications ADD COLUMN last_error TEXT;
ALTER TABLE compose_projects ADD COLUMN last_error TEXT;
