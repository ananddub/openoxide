-- Extracted from src/db/repository/destinations.rs. Do not edit repository SQL directly.

-- name: DestinationsGetAll :many
SELECT CAST(id AS TEXT) AS id, name AS name, provider AS provider, access_key AS access_key, secret_access_key AS secret_access_key, bucket AS bucket, region AS region, endpoint AS endpoint, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM destinations;

-- name: DestinationsGetById :one
SELECT CAST(id AS TEXT) AS id, name AS name, provider AS provider, access_key AS access_key, secret_access_key AS secret_access_key, bucket AS bucket, region AS region, endpoint AS endpoint, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM destinations WHERE id = ?;

-- name: DestinationsCreate :exec
INSERT INTO destinations (name, provider, access_key, secret_access_key, bucket, region, endpoint, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: DestinationsUpdate :exec
UPDATE destinations SET name = ?, provider = ?, access_key = ?, secret_access_key = ?, bucket = ?, region = ?, endpoint = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: DestinationsDelete :exec
DELETE FROM destinations WHERE id = ?;
