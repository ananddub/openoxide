-- Extracted from src/db/repository/patches.rs. Do not edit repository SQL directly.

-- name: PatchesGetAll :many
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM patches;

-- name: PatchesGetById :one
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE id = ?;

-- name: PatchesListByApplication :many
SELECT id AS id, patch_type AS patch_type,
                      file_path AS file_path, enabled AS enabled,
                      content AS content, service_id AS service_id,
                      created_at AS created_at,
                      updated_at AS updated_at
               FROM patches WHERE service_id = ? ORDER BY file_path, id;

-- name: PatchesListByCompose :many
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE service_id = ? ORDER BY file_path, id;

-- name: PatchesGetForCompose :one
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE id = ? AND service_id = ?;

-- name: PatchesCreateForCompose :exec
INSERT INTO patches (patch_type, file_path, enabled, content, service_id) VALUES (?, ?, ?, ?, ?);

-- name: PatchesUpdateForCompose :exec
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ? WHERE id = ? AND service_id = ?;

-- name: PatchesDeleteForCompose :exec
DELETE FROM patches WHERE id = ? AND service_id = ?;

-- name: PatchesGetForApplication :one
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE id = ? AND service_id = ?;

-- name: PatchesCreateForApplication :one
INSERT INTO patches (patch_type, file_path, enabled, content, service_id) VALUES (?, ?, ?, ?, ?)
               RETURNING id, patch_type, file_path, enabled, content, service_id, created_at, updated_at;

-- name: PatchesUpdateForApplication :one
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ? WHERE id = ? AND service_id = ?
               RETURNING id, patch_type, file_path, enabled, content, service_id, created_at, updated_at;

-- name: PatchesDeleteForApplication :one
DELETE FROM patches WHERE id = ? AND service_id = ? RETURNING id;

-- name: PatchesCreate :exec
INSERT INTO patches (patch_type, file_path, enabled, content, service_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?);

-- name: PatchesUpdate :exec
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ?, service_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: PatchesDelete :exec
DELETE FROM patches WHERE id = ?;
