-- Extracted from src/db/repository/libsql.rs. Do not edit repository SQL directly.

-- name: LibsqlGetById :one
SELECT 'libsql' AS kind, id AS id, name, app_name, description, docker_image,
               CAST(NULL AS TEXT) AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM libsql_dbs WHERE id = ?;

-- name: LibsqlCreate :exec
INSERT INTO libsql_dbs
               (name, app_name, description, docker_image, database_user, database_password,
                sqld_node, sqld_primary_url, enable_namespaces,
                external_port, external_grpc_port, external_admin_port, command, args, env_var, network_ids, detach_rustploy_network, environment_id, server_id)
               VALUES (?, ?, ?, ?, ?, ?, COALESCE(?, 'PRIMARY'), COALESCE(?, ''), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: LibsqlUpdate :exec
UPDATE libsql_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), database_user = COALESCE(?, database_user), database_password = COALESCE(?, database_password), external_port = COALESCE(?, external_port), external_grpc_port = COALESCE(?, external_grpc_port), external_admin_port = COALESCE(?, external_admin_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id), network_ids = COALESCE(?, network_ids), detach_rustploy_network = COALESCE(?, detach_rustploy_network) WHERE id = ?;

-- name: LibsqlDelete :exec
DELETE FROM libsql_dbs WHERE id = ?;

-- name: LibsqlGetServerIdAndName :one
SELECT server_id, name FROM libsql_dbs WHERE id = ?;

-- name: LibsqlUpdateStatus :exec
UPDATE libsql_dbs SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: LibsqlGetDetails :one
SELECT name, app_name, docker_image, database_user, database_password, sqld_node, sqld_primary_url, enable_namespaces,
               external_port, external_grpc_port, external_admin_port, command, args, env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit,
               replicas, network_ids, detach_rustploy_network, environment_id
               FROM libsql_dbs WHERE id = ?;
