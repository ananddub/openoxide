-- Extracted from src/db/repository/database_management.rs. Do not edit repository SQL directly.

-- name: DatabaseManagementCredentials1 :one
SELECT app_name, database_name, database_user, database_password, external_port, server_id FROM postgres_dbs WHERE id = ?;

-- name: DatabaseManagementCredentials2 :one
SELECT app_name, database_name, database_user, database_password, external_port, server_id FROM mysql_dbs WHERE id = ?;

-- name: DatabaseManagementCredentials3 :one
SELECT app_name, database_name, database_user, database_password, external_port, server_id FROM mariadb_dbs WHERE id = ?;

-- name: DatabaseManagementCredentials4 :one
SELECT app_name, database_user, database_password, external_port, server_id FROM mongo_dbs WHERE id = ?;

-- name: DatabaseManagementCredentials5 :one
SELECT app_name, database_password, external_port, server_id FROM redis_dbs WHERE id = ?;

-- name: DatabaseManagementCredentials6 :one
SELECT app_name, database_user, database_password, external_port, server_id FROM libsql_dbs WHERE id = ?;

-- name: DatabaseManagementRotatePassword1 :exec
UPDATE postgres_dbs SET database_password = ? WHERE id = ?;

-- name: DatabaseManagementRotatePassword2 :exec
UPDATE mysql_dbs SET database_password = ? WHERE id = ?;

-- name: DatabaseManagementRotatePassword3 :exec
UPDATE mariadb_dbs SET database_password = ? WHERE id = ?;

-- name: DatabaseManagementRotatePassword4 :exec
UPDATE mongo_dbs SET database_password = ? WHERE id = ?;

-- name: DatabaseManagementRotatePassword5 :exec
UPDATE redis_dbs SET database_password = ? WHERE id = ?;

-- name: DatabaseManagementRotatePassword6 :exec
UPDATE libsql_dbs SET database_password = ? WHERE id = ?;
