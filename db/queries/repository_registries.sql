-- Extracted from src/db/repository/registries.rs. Do not edit repository SQL directly.

-- name: RegistriesGetAll :many
SELECT id AS id, registry_name AS registry_name, image_prefix AS image_prefix, username AS username, password AS password, registry_url AS registry_url, registry_type AS registry_type, created_at AS created_at, updated_at AS updated_at FROM registries;

-- name: RegistriesGetById :one
SELECT id AS id, registry_name AS registry_name, image_prefix AS image_prefix, username AS username, password AS password, registry_url AS registry_url, registry_type AS registry_type, created_at AS created_at, updated_at AS updated_at FROM registries WHERE id = ?;

-- name: RegistriesCreate :exec
INSERT INTO registries (registry_name, image_prefix, username, password, registry_url, registry_type, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: RegistriesUpdate :exec
UPDATE registries SET registry_name = ?, image_prefix = ?, username = ?, password = ?, registry_url = ?, registry_type = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: RegistriesDelete :exec
DELETE FROM registries WHERE id = ?;

-- name: RegistriesUsageReferences :many
SELECT id AS resource_id, name AS resource_name, 'APPLICATION' AS resource_type, 'DEPLOY' AS usage_kind FROM applications WHERE registry_id=?
               UNION ALL SELECT id AS resource_id, name AS resource_name, 'APPLICATION' AS resource_type, 'ROLLBACK' AS usage_kind FROM applications WHERE rollback_registry_id=?
               UNION ALL SELECT id AS resource_id, name AS resource_name, 'APPLICATION' AS resource_type, 'BUILD' AS usage_kind FROM applications WHERE build_registry_id=?;
