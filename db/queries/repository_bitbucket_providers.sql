-- Extracted from src/db/repository/bitbucket_providers.rs. Do not edit repository SQL directly.

-- name: BitbucketProvidersGetAll :many
SELECT id AS id, bitbucket_username AS bitbucket_username, bitbucket_email AS bitbucket_email, app_password AS app_password, api_token AS api_token, bitbucket_workspace_name AS bitbucket_workspace_name, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM bitbucket_providers;

-- name: BitbucketProvidersGetById :one
SELECT id AS id, bitbucket_username AS bitbucket_username, bitbucket_email AS bitbucket_email, app_password AS app_password, api_token AS api_token, bitbucket_workspace_name AS bitbucket_workspace_name, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM bitbucket_providers WHERE id = ?;

-- name: BitbucketProvidersGetByGitProviderId :one
SELECT id AS id, bitbucket_username AS bitbucket_username, bitbucket_email AS bitbucket_email, app_password AS app_password, api_token AS api_token, bitbucket_workspace_name AS bitbucket_workspace_name, git_provider_id AS git_provider_id, created_at AS created_at, updated_at AS updated_at FROM bitbucket_providers WHERE git_provider_id = ?;

-- name: BitbucketProvidersCreate :exec
INSERT INTO bitbucket_providers (bitbucket_username, bitbucket_email, app_password, api_token, bitbucket_workspace_name, git_provider_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: BitbucketProvidersUpdate :exec
UPDATE bitbucket_providers SET bitbucket_username = ?, bitbucket_email = ?, app_password = ?, api_token = ?, bitbucket_workspace_name = ?, git_provider_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: BitbucketProvidersDelete :exec
DELETE FROM bitbucket_providers WHERE id = ?;

-- name: BitbucketProvidersDisconnect :exec
UPDATE bitbucket_providers SET app_password = NULL, api_token = NULL, updated_at = strftime('%s', 'now') WHERE git_provider_id = ?;
