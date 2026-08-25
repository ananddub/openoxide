-- Extracted from src/db/repository/git_providers.rs. Do not edit repository SQL directly.

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
