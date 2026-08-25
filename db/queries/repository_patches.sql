-- Extracted from src/db/repository/patches.rs. Do not edit repository SQL directly.

-- name: PatchesGetAll :many
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, application_id AS application_id, compose_id AS compose_id, created_at AS created_at, updated_at AS updated_at FROM patches;

-- name: PatchesGetById :one
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, application_id AS application_id, compose_id AS compose_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE id = ?;

-- name: PatchesListByApplication :many
SELECT id AS id, patch_type AS patch_type,
                      file_path AS file_path, enabled AS enabled,
                      content AS content, application_id AS application_id,
                      compose_id AS compose_id, created_at AS created_at,
                      updated_at AS updated_at
               FROM patches WHERE application_id = ? ORDER BY file_path, id;

-- name: PatchesListByCompose :many
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, application_id AS application_id, compose_id AS compose_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE compose_id = ? ORDER BY file_path, id;

-- name: PatchesGetForCompose :one
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, application_id AS application_id, compose_id AS compose_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE id = ? AND compose_id = ?;

-- name: PatchesCreateForCompose :exec
INSERT INTO patches (patch_type, file_path, enabled, content, compose_id) VALUES (?, ?, ?, ?, ?);

-- name: PatchesUpdateForCompose :exec
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ? WHERE id = ? AND compose_id = ?;

-- name: PatchesDeleteForCompose :exec
DELETE FROM patches WHERE id = ? AND compose_id = ?;

-- name: PatchesGetForApplication :one
SELECT id AS id, patch_type AS patch_type, file_path AS file_path, enabled AS enabled, content AS content, application_id AS application_id, compose_id AS compose_id, created_at AS created_at, updated_at AS updated_at FROM patches WHERE id = ? AND application_id = ?;

-- name: PatchesCreateForApplication :exec
INSERT INTO patches (patch_type, file_path, enabled, content, application_id) VALUES (?, ?, ?, ?, ?);

-- name: PatchesUpdateForApplication :exec
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ? WHERE id = ? AND application_id = ?;

-- name: PatchesDeleteForApplication :exec
DELETE FROM patches WHERE id = ? AND application_id = ?;

-- name: PatchesCreate :exec
INSERT INTO patches (patch_type, file_path, enabled, content, application_id, compose_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: PatchesUpdate :exec
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ?, application_id = ?, compose_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: PatchesDelete :exec
DELETE FROM patches WHERE id = ?;
