-- ============================================================
-- DOMAINS
-- ============================================================

-- name: GetDomainByID :one
SELECT * FROM domains WHERE id = ?;

-- name: GetDomainByHost :one
SELECT * FROM domains WHERE host = ?;

-- name: ListDomainsByApplication :many
SELECT * FROM domains WHERE application_id = ? ORDER BY created_at DESC;

-- name: ListDomainsByCompose :many
SELECT * FROM domains WHERE compose_id = ? ORDER BY created_at DESC;

-- name: CreateDomain :one
INSERT INTO domains (
    host, https, port, path, internal_path,
    custom_entrypoint, service_name, custom_cert_resolver,
    strip_path, middlewares, domain_type, certificate_type,
    application_id, compose_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateDomain :one
UPDATE domains
SET
    host                = ?,
    https               = ?,
    port                = ?,
    path                = ?,
    internal_path       = ?,
    custom_entrypoint   = ?,
    service_name        = ?,
    custom_cert_resolver = ?,
    strip_path          = ?,
    middlewares         = ?,
    certificate_type    = ?
WHERE id = ?
RETURNING *;

-- name: DeleteDomain :exec
DELETE FROM domains WHERE id = ?;

-- name: DeleteDomainsByApplication :exec
DELETE FROM domains WHERE application_id = ?;

-- ============================================================
-- PATCHES
-- ============================================================

-- name: GetPatchByID :one
SELECT * FROM patches WHERE id = ?;

-- name: ListPatchesByApplication :many
SELECT * FROM patches WHERE application_id = ? ORDER BY created_at DESC;

-- name: ListPatchesByCompose :many
SELECT * FROM patches WHERE compose_id = ? ORDER BY created_at DESC;

-- name: CreatePatch :one
INSERT INTO patches (patch_type, file_path, enabled, content, application_id, compose_id)
VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdatePatch :one
UPDATE patches
SET patch_type = ?, file_path = ?, enabled = ?, content = ?
WHERE id = ?
RETURNING *;

-- name: TogglePatch :exec
UPDATE patches SET enabled = ? WHERE id = ?;

-- name: DeletePatch :exec
DELETE FROM patches WHERE id = ?;

-- ============================================================
-- MOUNTS
-- ============================================================

-- name: GetMountByID :one
SELECT * FROM mounts WHERE id = ?;

-- name: ListMountsByApplication :many
SELECT * FROM mounts WHERE application_id = ? ORDER BY created_at DESC;

-- name: ListMountsByCompose :many
SELECT * FROM mounts WHERE compose_id = ? ORDER BY created_at DESC;

-- name: ListMountsByService :many
SELECT * FROM mounts WHERE service_type = ? ORDER BY created_at DESC;

-- name: CreateMount :one
INSERT INTO mounts (
    mount_type, service_type, host_path, volume_name,
    file_path, content, mount_path,
    postgres_id, mysql_id, mariadb_id, mongo_id, redis_id,
    libsql_id, compose_id, application_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateMount :one
UPDATE mounts
SET
    mount_type  = ?,
    host_path   = ?,
    volume_name = ?,
    file_path   = ?,
    content     = ?,
    mount_path  = ?
WHERE id = ?
RETURNING *;

-- name: DeleteMount :exec
DELETE FROM mounts WHERE id = ?;

-- name: DeleteMountsByApplication :exec
DELETE FROM mounts WHERE application_id = ?;
