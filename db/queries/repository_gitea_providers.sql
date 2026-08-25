-- Extracted from src/db/repository/gitea_providers.rs. Do not edit repository SQL directly.

-- name: GiteaProvidersGetAll :many
SELECT id AS id, gitea_url AS gitea_url, gitea_internal_url AS gitea_internal_url, redirect_uri AS redirect_uri, client_id AS client_id, client_secret AS client_secret, access_token AS access_token, refresh_token AS refresh_token, expires_at AS expires_at, scopes AS scopes, last_authenticated_at AS last_authenticated_at, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM gitea_providers;

-- name: GiteaProvidersGetById :one
SELECT id AS id, gitea_url AS gitea_url, gitea_internal_url AS gitea_internal_url, redirect_uri AS redirect_uri, client_id AS client_id, client_secret AS client_secret, access_token AS access_token, refresh_token AS refresh_token, expires_at AS expires_at, scopes AS scopes, last_authenticated_at AS last_authenticated_at, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM gitea_providers WHERE id = ?;

-- name: GiteaProvidersGetByGitProviderId :one
SELECT id AS id, gitea_url AS gitea_url, gitea_internal_url AS gitea_internal_url, redirect_uri AS redirect_uri, client_id AS client_id, client_secret AS client_secret, access_token AS access_token, refresh_token AS refresh_token, expires_at AS expires_at, scopes AS scopes, last_authenticated_at AS last_authenticated_at, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM gitea_providers WHERE git_provider_id = ?;

-- name: GiteaProvidersCreate :exec
INSERT INTO gitea_providers (gitea_url, gitea_internal_url, redirect_uri, client_id, client_secret, access_token, refresh_token, expires_at, scopes, last_authenticated_at, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: GiteaProvidersUpdate :exec
UPDATE gitea_providers SET gitea_url = ?, gitea_internal_url = ?, redirect_uri = ?, client_id = ?, client_secret = ?, access_token = ?, refresh_token = ?, expires_at = ?, scopes = ?, last_authenticated_at = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: GiteaProvidersDelete :exec
DELETE FROM gitea_providers WHERE id = ?;

-- name: GiteaProvidersSetOauthTokens :exec
UPDATE gitea_providers SET access_token = ?, refresh_token = ?, expires_at = ?, scopes = COALESCE(?, scopes), last_authenticated_at = ?, updated_at = ? WHERE git_provider_id = ?;

-- name: GiteaProvidersDisconnect :exec
UPDATE gitea_providers SET access_token = NULL, refresh_token = NULL, expires_at = NULL, last_authenticated_at = NULL, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?;
