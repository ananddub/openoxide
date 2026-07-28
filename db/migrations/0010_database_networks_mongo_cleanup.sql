-- Migration 0010: Add database network selection parity and normalize Mongo replica_sets.

ALTER TABLE postgres_dbs ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE postgres_dbs ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;

ALTER TABLE mysql_dbs ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE mysql_dbs ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;

ALTER TABLE mariadb_dbs ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE mariadb_dbs ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;

ALTER TABLE mongo_dbs ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE mongo_dbs ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;
ALTER TABLE mongo_dbs ADD COLUMN replica_sets_int INTEGER NOT NULL DEFAULT 0;
UPDATE mongo_dbs
SET replica_sets_int = CASE
	WHEN replica_sets IN ('1', 'true', 'TRUE', 'yes', 'YES', 'on', 'ON') THEN 1
	ELSE 0
END;
ALTER TABLE mongo_dbs DROP COLUMN replica_sets;
ALTER TABLE mongo_dbs RENAME COLUMN replica_sets_int TO replica_sets;

ALTER TABLE redis_dbs ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE redis_dbs ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;

ALTER TABLE libsql_dbs ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE libsql_dbs ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;
