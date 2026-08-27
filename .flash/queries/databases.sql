-- ============================================================
-- POSTGRES
-- ============================================================

-- name: GetPostgresByID :one
SELECT * FROM postgres_dbs WHERE id = ?;

-- name: GetPostgresByAppName :one
SELECT * FROM postgres_dbs WHERE app_name = ?;

-- name: ListPostgresByEnvironment :many
SELECT * FROM postgres_dbs WHERE environment_id = ? ORDER BY created_at DESC;

-- name: ListPostgresByServer :many
SELECT * FROM postgres_dbs WHERE server_id = ? ORDER BY created_at DESC;

-- name: CreatePostgres :one
INSERT INTO postgres_dbs (
    name, app_name, description, docker_image,
    database_name, database_user, database_password,
    external_port, env_var, replicas,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdatePostgresStatus :exec
UPDATE postgres_dbs SET app_status = ? WHERE id = ?;

-- name: UpdatePostgresResources :one
UPDATE postgres_dbs
SET memory_reservation = ?, memory_limit = ?,
    cpu_reservation = ?, cpu_limit = ?, replicas = ?
WHERE id = ?
RETURNING *;

-- name: DeletePostgres :exec
DELETE FROM postgres_dbs WHERE id = ?;

-- ============================================================
-- MYSQL
-- ============================================================

-- name: GetMysqlByID :one
SELECT * FROM mysql_dbs WHERE id = ?;

-- name: GetMysqlByAppName :one
SELECT * FROM mysql_dbs WHERE app_name = ?;

-- name: ListMysqlByEnvironment :many
SELECT * FROM mysql_dbs WHERE environment_id = ? ORDER BY created_at DESC;

-- name: ListMysqlByServer :many
SELECT * FROM mysql_dbs WHERE server_id = ? ORDER BY created_at DESC;

-- name: CreateMysql :one
INSERT INTO mysql_dbs (
    name, app_name, description, docker_image,
    database_name, database_user, database_password, database_root_password,
    external_port, env_var, replicas,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateMysqlStatus :exec
UPDATE mysql_dbs SET app_status = ? WHERE id = ?;

-- name: UpdateMysqlResources :one
UPDATE mysql_dbs
SET memory_reservation = ?, memory_limit = ?,
    cpu_reservation = ?, cpu_limit = ?, replicas = ?
WHERE id = ?
RETURNING *;

-- name: DeleteMysql :exec
DELETE FROM mysql_dbs WHERE id = ?;

-- ============================================================
-- MARIADB
-- ============================================================

-- name: GetMariadbByID :one
SELECT * FROM mariadb_dbs WHERE id = ?;

-- name: GetMariadbByAppName :one
SELECT * FROM mariadb_dbs WHERE app_name = ?;

-- name: ListMariadbByEnvironment :many
SELECT * FROM mariadb_dbs WHERE environment_id = ? ORDER BY created_at DESC;

-- name: CreateMariadb :one
INSERT INTO mariadb_dbs (
    name, app_name, description, docker_image,
    database_name, database_user, database_password, database_root_password,
    external_port, env_var, replicas,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateMariadbStatus :exec
UPDATE mariadb_dbs SET app_status = ? WHERE id = ?;

-- name: DeleteMariadb :exec
DELETE FROM mariadb_dbs WHERE id = ?;

-- ============================================================
-- MONGODB
-- ============================================================

-- name: GetMongoByID :one
SELECT * FROM mongo_dbs WHERE id = ?;

-- name: GetMongoByAppName :one
SELECT * FROM mongo_dbs WHERE app_name = ?;

-- name: ListMongoByEnvironment :many
SELECT * FROM mongo_dbs WHERE environment_id = ? ORDER BY created_at DESC;

-- name: CreateMongo :one
INSERT INTO mongo_dbs (
    name, app_name, description, docker_image,
    database_user, database_password,
    replica_sets, external_port, env_var, replicas,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateMongoStatus :exec
UPDATE mongo_dbs SET app_status = ? WHERE id = ?;

-- name: DeleteMongo :exec
DELETE FROM mongo_dbs WHERE id = ?;

-- ============================================================
-- REDIS
-- ============================================================

-- name: GetRedisByID :one
SELECT * FROM redis_dbs WHERE id = ?;

-- name: GetRedisByAppName :one
SELECT * FROM redis_dbs WHERE app_name = ?;

-- name: ListRedisByEnvironment :many
SELECT * FROM redis_dbs WHERE environment_id = ? ORDER BY created_at DESC;

-- name: CreateRedis :one
INSERT INTO redis_dbs (
    name, app_name, description, docker_image,
    database_password, external_port, env_var, replicas,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateRedisStatus :exec
UPDATE redis_dbs SET app_status = ? WHERE id = ?;

-- name: DeleteRedis :exec
DELETE FROM redis_dbs WHERE id = ?;

-- ============================================================
-- LIBSQL
-- ============================================================

-- name: GetLibsqlByID :one
SELECT * FROM libsql_dbs WHERE id = ?;

-- name: GetLibsqlByAppName :one
SELECT * FROM libsql_dbs WHERE app_name = ?;

-- name: ListLibsqlByEnvironment :many
SELECT * FROM libsql_dbs WHERE environment_id = ? ORDER BY created_at DESC;

-- name: CreateLibsql :one
INSERT INTO libsql_dbs (
    name, app_name, description, docker_image,
    database_user, database_password,
    sqld_node, sqld_primary_url, enable_namespaces,
    external_port, external_grpc_port, external_admin_port,
    env_var, replicas,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateLibsqlStatus :exec
UPDATE libsql_dbs SET app_status = ? WHERE id = ?;

-- name: DeleteLibsql :exec
DELETE FROM libsql_dbs WHERE id = ?;
