-- Extracted from src/db/repository/backups.rs. Do not edit repository SQL directly.

-- name: BackupsGetAll :many
SELECT id AS id, app_name AS app_name, schedule AS schedule, enabled AS enabled, database_name AS database_name, prefix AS prefix, service_name AS service_name, keep_latest_count AS keep_latest_count, backup_type AS backup_type, database_type AS database_type, metadata AS metadata, compose_id AS compose_id, postgres_id AS postgres_id, mysql_id AS mysql_id, mariadb_id AS mariadb_id, mongo_id AS mongo_id, redis_id AS redis_id, libsql_id AS libsql_id, destination_id AS destination_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM backups;

-- name: BackupsGetById :one
SELECT id AS id, app_name AS app_name, schedule AS schedule, enabled AS enabled, database_name AS database_name, prefix AS prefix, service_name AS service_name, keep_latest_count AS keep_latest_count, backup_type AS backup_type, database_type AS database_type, metadata AS metadata, compose_id AS compose_id, postgres_id AS postgres_id, mysql_id AS mysql_id, mariadb_id AS mariadb_id, mongo_id AS mongo_id, redis_id AS redis_id, libsql_id AS libsql_id, destination_id AS destination_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM backups WHERE id = ?;

-- name: BackupsCreate :exec
INSERT INTO backups (app_name, schedule, enabled, database_name, prefix, service_name, keep_latest_count, backup_type, database_type, metadata, compose_id, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, destination_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: BackupsUpdate :exec
UPDATE backups SET app_name = ?, schedule = ?, enabled = ?, database_name = ?, prefix = ?, service_name = ?, keep_latest_count = ?, backup_type = ?, database_type = ?, metadata = ?, compose_id = ?, postgres_id = ?, mysql_id = ?, mariadb_id = ?, mongo_id = ?, redis_id = ?, libsql_id = ?, destination_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: BackupsDelete :exec
DELETE FROM backups WHERE id = ?;
