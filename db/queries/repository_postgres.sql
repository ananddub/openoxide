-- Extracted from src/db/repository/postgres.rs. Do not edit repository SQL directly.

-- name: PostgresListAllByEnvironment :many
SELECT 'postgres' AS kind, id AS id, name, app_name, description, docker_image,
               database_name AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM postgres_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'mysql' AS kind, id AS id, name, app_name, description, docker_image,
               database_name AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM mysql_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'mariadb' AS kind, id AS id, name, app_name, description, docker_image,
               database_name AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM mariadb_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'mongo' AS kind, id AS id, name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM mongo_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'redis' AS kind, id AS id, name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS database_name, CAST(NULL AS TEXT) AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM redis_dbs WHERE environment_id = ?
               UNION ALL
               SELECT 'libsql' AS kind, id AS id, name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM libsql_dbs WHERE environment_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: PostgresGetById :one
SELECT 'postgres' AS kind, id AS id, name, app_name, description, docker_image,
               database_name AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM postgres_dbs WHERE id = ?;

-- name: PostgresCreate :exec
INSERT INTO postgres_dbs
               (name, app_name, description, docker_image, database_name, database_user,
                database_password, external_port, command, args, env_var, network_ids, detach_rustploy_network, environment_id, server_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, COALESCE(?, '[]'), COALESCE(?, 0), ?, ?);

-- name: PostgresUpdate :exec
UPDATE postgres_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), database_name = COALESCE(?, database_name), database_user = COALESCE(?, database_user), database_password = COALESCE(?, database_password), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id), network_ids = COALESCE(?, network_ids), detach_rustploy_network = COALESCE(?, detach_rustploy_network) WHERE id = ?;

-- name: PostgresDelete :exec
DELETE FROM postgres_dbs WHERE id = ?;

-- name: PostgresGetServerIdAndName :one
SELECT server_id, name FROM postgres_dbs WHERE id = ?;

-- name: PostgresUpdateStatus :exec
UPDATE postgres_dbs SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: PostgresGetDetails :one
SELECT name, app_name, docker_image, database_name, database_user, database_password,
               postgres_config, replication_role, primary_host, primary_port, replication_user, replication_password,
               external_port, command, args, env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit,
               replicas, network_ids, detach_rustploy_network, environment_id
               FROM postgres_dbs WHERE id = ?;

-- name: PostgresUpdateAdvancedConfig :exec
UPDATE postgres_dbs SET postgres_config = ?, replication_role = ?,
               primary_host = ?, primary_port = ?, replication_user = ?,
               replication_password = COALESCE(?, replication_password)
               WHERE id = ?;
