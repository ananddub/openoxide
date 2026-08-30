-- Managed databases: POSTGRES | MYSQL | MARIADB | MONGO | REDIS | LIBSQL
-- Engine-specific credentials/config live in engine_config (JSON).
CREATE TABLE service_database (
    service_id BIGINT NOT NULL PRIMARY KEY REFERENCES services(id) ON DELETE CASCADE,
    db_type TEXT NOT NULL,
    external_port BIGINT,
    -- common credentials
    db_name TEXT,
    db_user TEXT,
    db_password TEXT,
    -- engine config JSON, validated:
    --   POSTGRES: {"postgres_config":..., "replication_role":..., "primary_host":..., "primary_port":..., "replication_user":..., "replication_password":...}
    --   MYSQL/MARIADB: {"database_root_password":...}
    --   MONGO: {"database_root_password":..., "replica_sets":...}
    --   LIBSQL: {"sqld_node":..., "sqld_primary_url":..., "enable_namespaces":..., "external_grpc_port":..., "external_admin_port":...}
    engine_config TEXT NOT NULL DEFAULT '{}',
    CONSTRAINT db_type_check CHECK (db_type IN ('POSTGRES', 'MYSQL', 'MARIADB', 'MONGO', 'REDIS', 'LIBSQL')),
    CONSTRAINT engine_config_json_check CHECK (json_valid(engine_config))
);

CREATE INDEX idx_service_database_db_type ON service_database(db_type);
