-- Extracted from src/db/repository/mount.rs. Do not edit repository SQL directly.

-- name: MountFetchForDatabase :many
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               service_id,
               created_at, updated_at
               FROM mounts
               WHERE service_id = ?;

-- name: MountFetchForApplication :many
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               service_id,
               created_at, updated_at
               FROM mounts
               WHERE service_id = ?
               ORDER BY id;

-- name: MountGetForApplication :one
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content,
                      mount_path, service_id, created_at, updated_at
               FROM mounts WHERE id = ? AND service_id = ?;

-- name: MountCreateForApplication :one
INSERT INTO mounts
               (mount_type, service_type, host_path, volume_name, file_path, content,
                mount_path, service_id)
               VALUES (?, 'APPLICATION', ?, ?, ?, ?, ?, ?)
               RETURNING id, mount_type, service_type, host_path, volume_name, file_path,
                         content, mount_path, service_id, created_at, updated_at;

-- name: MountUpdateForApplication :one
UPDATE mounts
               SET mount_type = ?, service_type = 'APPLICATION', host_path = ?,
                   volume_name = ?, file_path = ?, content = ?, mount_path = ?
               WHERE id = ? AND service_id = ?
               RETURNING id, mount_type, service_type, host_path, volume_name, file_path,
                         content, mount_path, service_id, created_at, updated_at;

-- name: MountDeleteForApplication :one
DELETE FROM mounts WHERE id = ? AND service_id = ? RETURNING id;

-- name: MountFetchForCompose :many
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path,
               service_id,
               created_at, updated_at
               FROM mounts
               WHERE service_id = ?
               ORDER BY id;

-- name: MountCreateComposeFile :exec
INSERT INTO mounts
               (mount_type, service_type, file_path, content, mount_path, service_id)
               VALUES ('FILE', 'COMPOSE', ?, ?, '', ?);

-- name: MountGetForCompose :one
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id, created_at, updated_at FROM mounts WHERE id = ? AND service_id = ?;

-- Database mounts (unified: keyed by service_id, no per-engine columns).

-- name: MountGetForDatabase :one
SELECT id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id, created_at, updated_at
FROM mounts WHERE id = ? AND service_id = ?;

-- name: MountCreateForDatabase :one
INSERT INTO mounts
               (mount_type, service_type, host_path, volume_name, file_path, content,
                mount_path, service_id)
               VALUES (?, 'DATABASE', ?, ?, ?, ?, ?, ?)
               RETURNING id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id, created_at, updated_at;

-- name: MountUpdateForDatabase :one
UPDATE mounts
               SET mount_type = ?, service_type = 'DATABASE', host_path = ?,
                   volume_name = ?, file_path = ?, content = ?, mount_path = ?, updated_at = unixepoch()
               WHERE id = ? AND service_id = ?
               RETURNING id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id, created_at, updated_at;

-- name: MountDeleteForDatabase :exec
DELETE FROM mounts WHERE id = ? AND service_id = ?;

-- name: MountCreateForCompose :exec
INSERT INTO mounts (mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id) VALUES (?, 'COMPOSE', ?, ?, ?, ?, ?, ?);

-- name: MountUpdateForCompose :exec
UPDATE mounts SET mount_type = ?, service_type = 'COMPOSE', host_path = ?, volume_name = ?, file_path = ?, content = ?, mount_path = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND service_id = ?;

-- name: MountDeleteForCompose :exec
DELETE FROM mounts WHERE id = ? AND service_id = ?;
