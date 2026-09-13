-- Unified git provider credentials (vault-style: provider_type + config_json).
-- GITHUB: {"app_name":..., "app_id":..., "client_id":..., "client_secret":..., "installation_id":..., "private_key":...}
-- GITLAB: {"url":..., "internal_url":..., "application_id":..., "redirect_uri":..., "secret":..., "access_token":..., "refresh_token":..., "group_name":..., "expires_at":...}
-- GITEA:  {"url":..., "internal_url":..., "redirect_uri":..., "client_id":..., "client_secret":..., "access_token":..., "refresh_token":..., "expires_at":..., "scopes":..., "last_authenticated_at":...}
-- BITBUCKET: {"username":..., "email":..., "app_password":..., "api_token":..., "workspace_name":...}
-- Organization-scoped: reads/writes filter by organization_id.

-- name: GetGitProviderByID :one
SELECT * FROM git_providers WHERE id = ?;

-- name: ListGitProviders :many
SELECT * FROM git_providers WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListGitProvidersByType :many
SELECT * FROM git_providers WHERE provider_type = ? AND organization_id = ? ORDER BY created_at DESC;

-- name: CreateGitProvider :one
INSERT INTO git_providers (name, provider_type, config_json, shared, webhook_secret, organization_id)
VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateGitProvider :one
UPDATE git_providers SET name = ?, provider_type = ?, config_json = ?, shared = ?, webhook_secret = ? WHERE id = ? AND organization_id = ?
RETURNING *;

-- name: UpdateGitProviderConfig :exec
UPDATE git_providers SET config_json = ? WHERE id = ?;

-- name: DeleteGitProvider :exec
DELETE FROM git_providers WHERE id = ? AND organization_id = ?;

-- Webhook secret management (bool returns via COUNT / RETURNING row).

-- name: GitProvidersHasWebhookSecret :one
SELECT COUNT(*) AS hit
FROM git_providers
WHERE id = ? AND webhook_secret IS NOT NULL AND webhook_secret != '';

-- name: GitProvidersRotateWebhookSecret :many
UPDATE git_providers
SET webhook_secret = ?, updated_at = unixepoch()
WHERE id = ?
RETURNING id;

-- name: GitProvidersGetWebhookSecret :one
SELECT webhook_secret FROM git_providers WHERE id = ?;
