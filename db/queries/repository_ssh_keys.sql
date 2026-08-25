-- Extracted from src/db/repository/ssh_keys.rs. Do not edit repository SQL directly.

-- name: SshKeysGetAll :many
SELECT id AS id, name AS name, description AS description, private_key AS private_key, public_key AS public_key, last_used_at AS last_used_at, created_at AS created_at, updated_at AS updated_at FROM ssh_keys;

-- name: SshKeysGetById :one
SELECT id AS id, name AS name, description AS description, private_key AS private_key, public_key AS public_key, last_used_at AS last_used_at, created_at AS created_at, updated_at AS updated_at FROM ssh_keys WHERE id = ?;

-- name: SshKeysCreate :exec
INSERT INTO ssh_keys (name, description, private_key, public_key, last_used_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?);

-- name: SshKeysUpdate :exec
UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?, last_used_at = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: SshKeysIsInUse :one
SELECT
                (SELECT COUNT(*) FROM servers WHERE ssh_key_id = ?) +
                (SELECT COUNT(*) FROM applications WHERE custom_git_ssh_key_id = ?) +
                (SELECT COUNT(*) FROM compose_projects WHERE custom_git_ssh_key_id = ?) AS total;

-- name: SshKeysDelete :exec
DELETE FROM ssh_keys WHERE id = ?;

-- name: SshKeysTouchSshKey :exec
UPDATE ssh_keys SET last_used_at = strftime('%s', 'now') WHERE id = ?;

-- name: SshKeysListOrdered :many
SELECT id AS id, name, description, private_key, public_key, last_used_at, created_at, updated_at
               FROM ssh_keys ORDER BY created_at DESC, id DESC;

-- name: SshKeysCreateAndReturn :one
INSERT INTO ssh_keys (name, description, private_key, public_key)
               VALUES (?, ?, ?, ?)
               RETURNING id AS id, name, description, private_key, public_key, last_used_at, created_at, updated_at;

-- name: SshKeysUpdateAndReturn :one
UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?
               WHERE id = ?
               RETURNING id AS id, name, description, private_key, public_key, last_used_at, created_at, updated_at;

-- name: SshKeysTouchAndReturn :one
UPDATE ssh_keys SET last_used_at = strftime('%s', 'now')
               WHERE id = ?
               RETURNING id AS id, name, description, private_key, public_key, last_used_at, created_at, updated_at;
