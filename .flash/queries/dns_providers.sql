-- Extracted from src/db/repository/dns_providers.rs. Do not edit repository SQL directly.

-- name: DnsProvidersGetAll :many
SELECT id AS id, name AS name, provider_type AS provider_type, credentials_json AS credentials_json, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM dns_providers ORDER BY created_at DESC;

-- name: DnsProvidersGetById :one
SELECT id AS id, name AS name, provider_type AS provider_type, credentials_json AS credentials_json, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM dns_providers WHERE id = ?;

-- name: DnsProvidersListByOrganization :many
SELECT id AS id, name AS name, provider_type AS provider_type, credentials_json AS credentials_json, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM dns_providers WHERE organization_id = ? ORDER BY created_at DESC;

-- name: DnsProvidersCreate :exec
INSERT INTO dns_providers (name, provider_type, credentials_json, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?);

-- name: DnsProvidersGetLastInsertRowid :one
SELECT CAST(last_insert_rowid() AS INTEGER) AS id;

-- name: DnsProvidersUpdate :exec
UPDATE dns_providers SET name = ?, provider_type = ?, credentials_json = ?, updated_at = ? WHERE id = ? AND organization_id = ?;

-- name: DnsProvidersDelete :one
DELETE FROM dns_providers
WHERE id = ? AND organization_id = ?
RETURNING id;
