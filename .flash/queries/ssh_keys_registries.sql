-- ============================================================
-- SSH KEYS
-- ============================================================

-- name: GetSshKeyByID :one
SELECT * FROM ssh_keys WHERE id = ?;

-- name: ListSshKeys :many
SELECT * FROM ssh_keys ORDER BY created_at DESC;

-- name: CreateSshKey :one
INSERT INTO ssh_keys (name, description, private_key, public_key)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: UpdateSshKey :one
UPDATE ssh_keys
SET name = ?, description = ?
WHERE id = ?
RETURNING *;

-- name: UpdateSshKeyLastUsed :exec
UPDATE ssh_keys
SET last_used_at = CURRENT_TIMESTAMP
WHERE id = ?;

-- name: DeleteSshKey :exec
DELETE FROM ssh_keys WHERE id = ?;

-- ============================================================
-- REGISTRIES
-- ============================================================

-- name: GetRegistryByID :one
SELECT * FROM registries WHERE id = ?;

-- name: ListRegistries :many
SELECT * FROM registries ORDER BY created_at DESC;

-- name: ListRegistriesByType :many
SELECT * FROM registries WHERE registry_type = ? ORDER BY created_at DESC;

-- name: CreateRegistry :one
INSERT INTO registries (
    registry_name, image_prefix, username, password,
    registry_url, registry_type
) VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateRegistry :one
UPDATE registries
SET
    registry_name = ?,
    image_prefix  = ?,
    username      = ?,
    password      = ?,
    registry_url  = ?,
    registry_type = ?
WHERE id = ?
RETURNING *;

-- name: DeleteRegistry :exec
DELETE FROM registries WHERE id = ?;
