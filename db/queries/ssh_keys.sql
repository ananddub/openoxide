-- Extracted from src/db/repository/ssh_keys.rs. Do not edit repository SQL directly.
-- is_in_use was split into two counts because the generator cannot infer INTEGER
-- parameters inside SELECT-clause subqueries. CURRENT_TIMESTAMP replaced with
-- unixepoch() because last_used_at is a STRICT INTEGER column.
--
-- Organization-scoped: reads/writes filter by organization_id so switching
-- workspaces changes the visible set.

-- name: SshKeysListByOrganization :many
SELECT id AS id, name AS name, description AS description, private_key AS private_key, public_key AS public_key, last_used_at AS last_used_at, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM ssh_keys WHERE organization_id = ? ORDER BY created_at DESC, id DESC;

-- name: SshKeysGetById :one
SELECT id AS id, name AS name, description AS description, private_key AS private_key, public_key AS public_key, last_used_at AS last_used_at, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM ssh_keys WHERE id = ?;

-- name: SshKeysCreate :exec
INSERT INTO ssh_keys (name, description, private_key, public_key, last_used_at, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: SshKeysUpdate :exec
UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?, last_used_at = ?, created_at = ?, updated_at = ? WHERE id = ? AND organization_id = ?;

-- name: SshKeysServerUseCount :one
SELECT COUNT(*) AS count FROM servers WHERE ssh_key_id = ?;

-- name: SshKeysGitUseCount :one
SELECT COUNT(*) AS count FROM service_git WHERE custom_git_ssh_key_id = ?;

-- name: SshKeysDelete :exec
DELETE FROM ssh_keys WHERE id = ? AND organization_id = ?;

-- name: SshKeysTouchSshKey :exec
UPDATE ssh_keys SET last_used_at = unixepoch() WHERE id = ?;

-- name: SshKeysCreateAndReturn :one
INSERT INTO ssh_keys (name, description, private_key, public_key, organization_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS id, name, description, private_key, public_key, last_used_at, organization_id, created_at, updated_at;

-- name: SshKeysUpdateAndReturn :one
UPDATE ssh_keys SET name = ?, description = ?, private_key = ?, public_key = ?
               WHERE id = ? AND organization_id = ?
               RETURNING id AS id, name, description, private_key, public_key, last_used_at, organization_id, created_at, updated_at;

-- name: SshKeysTouchAndReturn :one
UPDATE ssh_keys SET last_used_at = unixepoch()
               WHERE id = ?
               RETURNING id AS id, name, description, private_key, public_key, last_used_at, organization_id, created_at, updated_at;
