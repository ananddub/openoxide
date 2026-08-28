-- ============================================================
-- GIT PROVIDERS (umbrella)
-- ============================================================

-- name: GetGitProviderByID :one
SELECT * FROM git_providers WHERE id = ?;

-- name: ListGitProviders :many
SELECT * FROM git_providers ORDER BY created_at DESC;

-- name: ListGitProvidersByType :many
SELECT * FROM git_providers WHERE provider_type = ? ORDER BY created_at DESC;

-- name: CreateGitProvider :one
INSERT INTO git_providers (name, provider_type, shared)
VALUES (?, ?, ?)
RETURNING *;

-- name: UpdateGitProvider :one
UPDATE git_providers SET name = ?, shared = ? WHERE id = ?
RETURNING *;

-- name: DeleteGitProvider :exec
DELETE FROM git_providers WHERE id = ?;

-- ============================================================
-- GITHUB PROVIDERS
-- ============================================================

-- name: GetGithubProviderByID :one
SELECT * FROM github_providers WHERE id = ?;

-- name: GetGithubProviderByGitProviderID :one
SELECT * FROM github_providers WHERE git_provider_id = ?;

-- name: CreateGithubProvider :one
INSERT INTO github_providers (
    github_app_name, github_app_id, github_client_id,
    github_client_secret, github_installation_id,
    github_private_key, github_webhook_secret, git_provider_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateGithubProvider :one
UPDATE github_providers
SET
    github_app_name         = ?,
    github_app_id           = ?,
    github_client_id        = ?,
    github_client_secret    = ?,
    github_installation_id  = ?,
    github_private_key      = ?,
    github_webhook_secret   = ?
WHERE id = ?
RETURNING *;

-- name: DeleteGithubProvider :exec
DELETE FROM github_providers WHERE id = ?;

-- ============================================================
-- GITLAB PROVIDERS
-- ============================================================

-- name: GetGitlabProviderByID :one
SELECT * FROM gitlab_providers WHERE id = ?;

-- name: GetGitlabProviderByGitProviderID :one
SELECT * FROM gitlab_providers WHERE git_provider_id = ?;

-- name: CreateGitlabProvider :one
INSERT INTO gitlab_providers (
    gitlab_url, gitlab_internal_url, application_id,
    redirect_uri, secret, access_token, refresh_token,
    group_name, expires_at, git_provider_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateGitlabProvider :one
UPDATE gitlab_providers
SET
    gitlab_url          = ?,
    gitlab_internal_url = ?,
    application_id      = ?,
    redirect_uri        = ?,
    secret              = ?,
    access_token        = ?,
    refresh_token       = ?,
    group_name          = ?,
    expires_at          = ?
WHERE id = ?
RETURNING *;

-- name: DeleteGitlabProvider :exec
DELETE FROM gitlab_providers WHERE id = ?;

-- ============================================================
-- GITEA PROVIDERS
-- ============================================================

-- name: GetGiteaProviderByID :one
SELECT * FROM gitea_providers WHERE id = ?;

-- name: GetGiteaProviderByGitProviderID :one
SELECT * FROM gitea_providers WHERE git_provider_id = ?;

-- name: CreateGiteaProvider :one
INSERT INTO gitea_providers (
    gitea_url, gitea_internal_url, redirect_uri,
    client_id, client_secret, access_token, refresh_token,
    expires_at, scopes, git_provider_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateGiteaProvider :one
UPDATE gitea_providers
SET
    gitea_url           = ?,
    gitea_internal_url  = ?,
    redirect_uri        = ?,
    client_id           = ?,
    client_secret       = ?,
    access_token        = ?,
    refresh_token       = ?,
    expires_at          = ?,
    scopes              = ?,
    last_authenticated_at = strftime('%s', 'now')
WHERE id = ?
RETURNING *;

-- name: DeleteGiteaProvider :exec
DELETE FROM gitea_providers WHERE id = ?;

-- ============================================================
-- BITBUCKET PROVIDERS
-- ============================================================

-- name: GetBitbucketProviderByID :one
SELECT * FROM bitbucket_providers WHERE id = ?;

-- name: GetBitbucketProviderByGitProviderID :one
SELECT * FROM bitbucket_providers WHERE git_provider_id = ?;

-- name: CreateBitbucketProvider :one
INSERT INTO bitbucket_providers (
    bitbucket_username, bitbucket_email, app_password,
    api_token, bitbucket_workspace_name, git_provider_id
) VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateBitbucketProvider :one
UPDATE bitbucket_providers
SET
    bitbucket_username      = ?,
    bitbucket_email         = ?,
    app_password            = ?,
    api_token               = ?,
    bitbucket_workspace_name = ?
WHERE id = ?
RETURNING *;

-- name: DeleteBitbucketProvider :exec
DELETE FROM bitbucket_providers WHERE id = ?;

-- BEGIN GENERATED REPOSITORY QUERIES
-- Extracted from src/db/repository/git_providers.rs. Do not edit this generated section directly.

-- name: GitProvidersGetAll :many
SELECT id AS id, name AS name, provider_type AS provider_type, shared AS shared, created_at AS created_at, updated_at AS updated_at FROM git_providers;

-- name: GitProvidersGetById :one
SELECT id AS id, name AS name, provider_type AS provider_type, shared AS shared, created_at AS created_at, updated_at AS updated_at FROM git_providers WHERE id = ?;

-- name: GitProvidersCreate :exec
INSERT INTO git_providers (name, provider_type, shared, created_at, updated_at) VALUES (?, ?, ?, ?, ?);

-- name: GitProvidersUpdate :exec
UPDATE git_providers SET name = ?, provider_type = ?, shared = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: GitProvidersDelete :exec
DELETE FROM git_providers WHERE id = ?;

-- name: GitProvidersRotateWebhookSecret :exec
UPDATE git_providers SET webhook_secret = ?, updated_at = strftime('%s', 'now') WHERE id = ?;

-- name: GitProvidersHasWebhookSecret :one
SELECT COUNT(*) AS count FROM git_providers WHERE id = ? AND webhook_secret IS NOT NULL AND webhook_secret != '';

-- name: GitProvidersWebhookSecret :one
SELECT webhook_secret FROM git_providers WHERE id = ?;

-- name: GitProvidersResourceUsageCount :one
SELECT
                (SELECT COUNT(*) FROM applications a
                 WHERE a.github_provider_id IN (SELECT id FROM github_providers WHERE git_provider_id = ?)
                    OR a.gitlab_provider_id IN (SELECT id FROM gitlab_providers WHERE git_provider_id = ?)
                    OR a.gitea_provider_id IN (SELECT id FROM gitea_providers WHERE git_provider_id = ?)
                    OR a.bitbucket_provider_id IN (SELECT id FROM bitbucket_providers WHERE git_provider_id = ?))
              + (SELECT COUNT(*) FROM compose_projects c
                 WHERE c.github_provider_id IN (SELECT id FROM github_providers WHERE git_provider_id = ?)
                    OR c.gitlab_provider_id IN (SELECT id FROM gitlab_providers WHERE git_provider_id = ?)
                    OR c.gitea_provider_id IN (SELECT id FROM gitea_providers WHERE git_provider_id = ?)
                    OR c.bitbucket_provider_id IN (SELECT id FROM bitbucket_providers WHERE git_provider_id = ?)) AS count;

-- name: GitProvidersDeleteIfUnused :exec
DELETE FROM git_providers WHERE id = ?;
