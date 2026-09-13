-- Extracted from src/db/repository/destinations.rs. Do not edit repository SQL directly.
-- CAST(id AS TEXT) removed: the flash Destinations model keeps id as BIGINT and
-- sqlx would fail to decode a TEXT value into i64. Create/update now RETURNING.
-- Organization-scoped: reads/writes filter by organization_id.

-- name: DestinationsListByOrganization :many
SELECT * FROM destinations WHERE organization_id = ? ORDER BY created_at DESC, id DESC;

-- name: DestinationsGetById :one
SELECT * FROM destinations WHERE id = ?;

-- name: DestinationsCreateAndReturn :one
INSERT INTO destinations (name, provider, access_key, secret_access_key, bucket, region, endpoint, organization_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING *;

-- name: DestinationsUpdateAndReturn :one
UPDATE destinations SET name = ?, provider = ?, access_key = ?, secret_access_key = ?, bucket = ?, region = ?, endpoint = ?
               WHERE id = ? AND organization_id = ?
               RETURNING *;

-- name: DestinationsDelete :exec
DELETE FROM destinations WHERE id = ? AND organization_id = ?;
