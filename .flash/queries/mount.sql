-- Extracted from src/db/repository/mount.rs. Do not edit repository SQL directly.

-- name: MountFetchForDatabase :many
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id,
               created_at, updated_at
               FROM mounts
               WHERE postgres_id = ? OR mysql_id = ? OR mariadb_id = ? OR mongo_id = ? OR redis_id = ? OR libsql_id = ?;

-- name: MountFetchForApplication :many
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id,
               created_at, updated_at
               FROM mounts
               WHERE application_id = ?
               ORDER BY id;

-- name: MountGetForApplication :one
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content,
                      mount_path, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id,
                      libsql_id, compose_id, application_id, created_at, updated_at
               FROM mounts WHERE id = ? AND application_id = ?;

-- name: MountCreateForApplication :exec
INSERT INTO mounts
               (mount_type, service_type, host_path, volume_name, file_path, content,
                mount_path, application_id)
               VALUES (?, 'APPLICATION', ?, ?, ?, ?, ?, ?);

-- name: MountUpdateForApplication :exec
UPDATE mounts
               SET mount_type = ?, service_type = 'APPLICATION', host_path = ?,
                   volume_name = ?, file_path = ?, content = ?, mount_path = ?
               WHERE id = ? AND application_id = ?;

-- name: MountDeleteForApplication :exec
DELETE FROM mounts WHERE id = ? AND application_id = ?;

-- name: MountFetchForCompose :many
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id,
               created_at, updated_at
               FROM mounts
               WHERE compose_id = ?
               ORDER BY id;

-- name: MountCreateComposeFile :exec
INSERT INTO mounts
               (mount_type, service_type, file_path, content, mount_path, compose_id)
               VALUES ('FILE', 'COMPOSE', ?, ?, '', ?);

-- name: MountGetForCompose :one
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, postgres_id, mysql_id, mariadb_id, mongo_id, redis_id, libsql_id, compose_id, application_id, created_at, updated_at FROM mounts WHERE id = ? AND compose_id = ?;

-- name: MountCreateForCompose :exec
INSERT INTO mounts (mount_type, service_type, host_path, volume_name, file_path, content, mount_path, compose_id) VALUES (?, 'COMPOSE', ?, ?, ?, ?, ?, ?);

-- name: MountUpdateForCompose :exec
UPDATE mounts SET mount_type = ?, service_type = 'COMPOSE', host_path = ?, volume_name = ?, file_path = ?, content = ?, mount_path = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND compose_id = ?;

-- name: MountDeleteForCompose :exec
DELETE FROM mounts WHERE id = ? AND compose_id = ?;
