-- Extracted from src/db/repository/mysql.rs. Do not edit repository SQL directly.

-- name: MysqlGetById :one
SELECT 'mysql' AS kind, id AS id, name, app_name, description, docker_image,
               database_name AS database_name, database_user AS database_user, database_password AS database_password, external_port, command, args,
               env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
               network_ids, detach_rustploy_network, app_status, last_error, environment_id, server_id, created_at, updated_at
               FROM mysql_dbs WHERE id = ?;

-- name: MysqlCreate :exec
INSERT INTO mysql_dbs
               (name, app_name, description, docker_image, database_name, database_user,
                database_password, database_root_password, external_port, command, args, env_var, network_ids, detach_rustploy_network, environment_id, server_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: MysqlUpdate :exec
UPDATE mysql_dbs SET name = COALESCE(?, name), description = COALESCE(?, description), docker_image = COALESCE(?, docker_image), database_name = COALESCE(?, database_name), database_user = COALESCE(?, database_user), database_password = COALESCE(?, database_password), external_port = COALESCE(?, external_port), command = COALESCE(?, command), args = COALESCE(?, args), env_var = COALESCE(?, env_var), memory_reservation = COALESCE(?, memory_reservation), memory_limit = COALESCE(?, memory_limit), cpu_reservation = COALESCE(?, cpu_reservation), cpu_limit = COALESCE(?, cpu_limit), replicas = COALESCE(?, replicas), server_id = COALESCE(?, server_id), network_ids = COALESCE(?, network_ids), detach_rustploy_network = COALESCE(?, detach_rustploy_network) WHERE id = ?;

-- name: MysqlDelete :exec
DELETE FROM mysql_dbs WHERE id = ?;

-- name: MysqlGetServerIdAndName :one
SELECT server_id, name FROM mysql_dbs WHERE id = ?;

-- name: MysqlUpdateStatus :exec
UPDATE mysql_dbs SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: MysqlGetDetails :one
SELECT name, app_name, docker_image, database_name, database_user, database_password, database_root_password,
               external_port, command, args, env_var, memory_reservation, memory_limit, cpu_reservation, cpu_limit,
               replicas, network_ids, detach_rustploy_network, environment_id
               FROM mysql_dbs WHERE id = ?;
