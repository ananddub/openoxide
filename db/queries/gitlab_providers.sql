-- Extracted from src/db/repository/gitlab_providers.rs. Do not edit repository SQL directly.

-- name: GitlabProvidersGetAll :many
SELECT id AS id, gitlab_url AS gitlab_url, gitlab_internal_url AS gitlab_internal_url, application_id AS application_id, redirect_uri AS redirect_uri, secret AS secret, access_token AS access_token, refresh_token AS refresh_token, group_name AS group_name, expires_at AS expires_at, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM gitlab_providers;

-- name: GitlabProvidersGetById :one
SELECT id AS id, gitlab_url AS gitlab_url, gitlab_internal_url AS gitlab_internal_url, application_id AS application_id, redirect_uri AS redirect_uri, secret AS secret, access_token AS access_token, refresh_token AS refresh_token, group_name AS group_name, expires_at AS expires_at, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM gitlab_providers WHERE id = ?;

-- name: GitlabProvidersGetByGitProviderId :one
SELECT id AS id, gitlab_url AS gitlab_url, gitlab_internal_url AS gitlab_internal_url, application_id AS application_id, redirect_uri AS redirect_uri, secret AS secret, access_token AS access_token, refresh_token AS refresh_token, group_name AS group_name, expires_at AS expires_at, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM gitlab_providers WHERE git_provider_id = ?;

-- name: GitlabProvidersCreate :exec
INSERT INTO gitlab_providers (gitlab_url, gitlab_internal_url, application_id, redirect_uri, secret, access_token, refresh_token, group_name, expires_at, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: GitlabProvidersUpdate :exec
UPDATE gitlab_providers SET gitlab_url = ?, gitlab_internal_url = ?, application_id = ?, redirect_uri = ?, secret = ?, access_token = ?, refresh_token = ?, group_name = ?, expires_at = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: GitlabProvidersDelete :exec
DELETE FROM gitlab_providers WHERE id = ?;

-- name: GitlabProvidersSetOauthTokens :exec
UPDATE gitlab_providers SET access_token = ?, refresh_token = ?, expires_at = ?, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?;

-- name: GitlabProvidersDisconnect :exec
UPDATE gitlab_providers SET access_token = NULL, refresh_token = NULL, expires_at = NULL, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?;
