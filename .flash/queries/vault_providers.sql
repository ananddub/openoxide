-- Extracted from src/db/repository/vault_providers.rs. Do not edit repository SQL directly.

-- name: VaultProvidersGetAll :many
SELECT id AS id, name AS name, provider_type AS provider_type, api_url AS api_url, auth_token AS auth_token, namespace AS namespace, config_json AS config_json, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM vault_providers ORDER BY created_at DESC;

-- name: VaultProvidersGetById :one
SELECT id AS id, name AS name, provider_type AS provider_type, api_url AS api_url, auth_token AS auth_token, namespace AS namespace, config_json AS config_json, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM vault_providers WHERE id = ?;

-- name: VaultProvidersListByOrganization :many
SELECT id AS id, name AS name, provider_type AS provider_type, api_url AS api_url, auth_token AS auth_token, namespace AS namespace, config_json AS config_json, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM vault_providers WHERE organization_id = ? ORDER BY created_at DESC;

-- name: VaultProvidersCreate :exec
INSERT INTO vault_providers (name, provider_type, api_url, auth_token, namespace, config_json, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: VaultProvidersUpdate :exec
UPDATE vault_providers SET name = ?, provider_type = ?, api_url = ?, auth_token = ?, namespace = ?, config_json = ?, updated_at = ? WHERE id = ? AND organization_id = ?;

-- name: VaultProvidersDelete :exec
DELETE FROM vault_providers WHERE id = ? AND organization_id = ?;
