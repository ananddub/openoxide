-- Extracted from src/db/repository/github_providers.rs. Do not edit repository SQL directly.

-- name: GithubProvidersGetAll :many
SELECT id AS id, github_app_name AS github_app_name, github_app_id AS github_app_id, github_client_id AS github_client_id, github_client_secret AS github_client_secret, github_installation_id AS github_installation_id, github_private_key AS github_private_key, github_webhook_secret AS github_webhook_secret, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM github_providers;

-- name: GithubProvidersGetById :one
SELECT id AS id, github_app_name AS github_app_name, github_app_id AS github_app_id, github_client_id AS github_client_id, github_client_secret AS github_client_secret, github_installation_id AS github_installation_id, github_private_key AS github_private_key, github_webhook_secret AS github_webhook_secret, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM github_providers WHERE id = ?;

-- name: GithubProvidersGetByGitProviderId :one
SELECT id AS id, github_app_name AS github_app_name, github_app_id AS github_app_id, github_client_id AS github_client_id, github_client_secret AS github_client_secret, github_installation_id AS github_installation_id, github_private_key AS github_private_key, github_webhook_secret AS github_webhook_secret, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM github_providers WHERE git_provider_id = ?;

-- name: GithubProvidersCreate :exec
INSERT INTO github_providers (github_app_name, github_app_id, github_client_id, github_client_secret, github_installation_id, github_private_key, github_webhook_secret, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: GithubProvidersUpdate :exec
UPDATE github_providers SET github_app_name = ?, github_app_id = ?, github_client_id = ?, github_client_secret = ?, github_installation_id = ?, github_private_key = ?, github_webhook_secret = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: GithubProvidersDelete :exec
DELETE FROM github_providers WHERE id = ?;

-- name: GithubProvidersSetInstallationId :exec
UPDATE github_providers SET github_installation_id = ?, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?;

-- name: GithubProvidersDisconnect :exec
UPDATE github_providers SET github_installation_id = NULL, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?;
