-- Extracted from src/db/repository/destinations.rs. Do not edit repository SQL directly.
-- CAST(id AS TEXT) removed: the flash Destinations model keeps id as BIGINT and
-- sqlx would fail to decode a TEXT value into i64. Create/update now RETURNING.

-- name: DestinationsGetAll :many
SELECT * FROM destinations;

-- name: DestinationsGetById :one
SELECT * FROM destinations WHERE id = ?;

-- name: DestinationsCreateAndReturn :one
INSERT INTO destinations (name, provider, access_key, secret_access_key, bucket, region, endpoint, organization_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING *;

-- name: DestinationsUpdateAndReturn :one
UPDATE destinations SET name = ?, provider = ?, access_key = ?, secret_access_key = ?, bucket = ?, region = ?, endpoint = ?
               WHERE id = ?
               RETURNING *;

-- name: DestinationsDelete :exec
DELETE FROM destinations WHERE id = ?;
