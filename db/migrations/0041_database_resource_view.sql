-- Unified read model for all managed database engines.
-- Engine-specific tables remain the write source during the compatibility
-- migration; services can move to this view without six-table UNION queries.
CREATE VIEW IF NOT EXISTS databases AS
SELECT id, 'POSTGRES' AS engine, name, app_name, description, docker_image,
       environment_id, server_id, app_status, external_port, network_ids,
       created_at, updated_at FROM postgres_dbs
UNION ALL
SELECT id, 'MYSQL', name, app_name, description, docker_image,
       environment_id, server_id, app_status, external_port, network_ids,
       created_at, updated_at FROM mysql_dbs
UNION ALL
SELECT id, 'MARIADB', name, app_name, description, docker_image,
       environment_id, server_id, app_status, external_port, network_ids,
       created_at, updated_at FROM mariadb_dbs
UNION ALL
SELECT id, 'MONGO', name, app_name, description, docker_image,
       environment_id, server_id, app_status, external_port, network_ids,
       created_at, updated_at FROM mongo_dbs
UNION ALL
SELECT id, 'REDIS', name, app_name, description, docker_image,
       environment_id, server_id, app_status, external_port, network_ids,
       created_at, updated_at FROM redis_dbs
UNION ALL
SELECT id, 'LIBSQL', name, app_name, description, docker_image,
       environment_id, server_id, app_status, external_port, network_ids,
       created_at, updated_at FROM libsql_dbs;
