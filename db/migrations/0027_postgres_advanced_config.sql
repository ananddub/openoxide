ALTER TABLE postgres_dbs ADD COLUMN postgres_config TEXT NOT NULL DEFAULT '{}';
ALTER TABLE postgres_dbs ADD COLUMN replication_role TEXT NOT NULL DEFAULT 'STANDALONE'
    CHECK (replication_role IN ('STANDALONE', 'PRIMARY', 'REPLICA'));
ALTER TABLE postgres_dbs ADD COLUMN primary_host TEXT;
ALTER TABLE postgres_dbs ADD COLUMN primary_port INTEGER;
ALTER TABLE postgres_dbs ADD COLUMN replication_user TEXT;
ALTER TABLE postgres_dbs ADD COLUMN replication_password TEXT;
