-- Extracted from src/db/repository/webhooks.rs. Do not edit repository SQL directly.

-- name: WebhooksCredentialSecret1 :one
SELECT gp.webhook_secret FROM github_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?;

-- name: WebhooksCredentialSecret2 :one
SELECT gp.webhook_secret FROM gitlab_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?;

-- name: WebhooksCredentialSecret3 :one
SELECT gp.webhook_secret FROM gitea_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?;

-- name: WebhooksCredentialSecret4 :one
SELECT gp.webhook_secret FROM bitbucket_providers p JOIN git_providers gp ON gp.id = p.git_provider_id WHERE p.id = ?;

-- name: WebhooksMatchingApplications1 :many
SELECT id AS id, watch_paths FROM applications WHERE source_type = 'GITHUB' AND auto_deploy = 1 AND trigger_type = ? AND lower(repository) = lower(?) AND lower(owner) = lower(?) AND branch = ?;

-- name: WebhooksMatchingApplications2 :many
SELECT id AS id, watch_paths FROM applications WHERE source_type = 'GITLAB' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitlab_repository) = lower(?) AND lower(gitlab_owner) = lower(?) AND gitlab_branch = ?;

-- name: WebhooksMatchingApplications3 :many
SELECT id AS id, watch_paths FROM applications WHERE source_type = 'GITEA' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitea_repository) = lower(?) AND lower(gitea_owner) = lower(?) AND gitea_branch = ?;

-- name: WebhooksMatchingApplications4 :many
SELECT id AS id, watch_paths FROM applications WHERE source_type = 'BITBUCKET' AND auto_deploy = 1 AND trigger_type = ? AND lower(bitbucket_repository) = lower(?) AND lower(bitbucket_owner) = lower(?) AND bitbucket_branch = ?;

-- name: WebhooksMatchingComposeProjects1 :many
SELECT id AS id, watch_paths FROM compose_projects WHERE source_type = 'GITHUB' AND auto_deploy = 1 AND trigger_type = ? AND lower(repository) = lower(?) AND lower(owner) = lower(?) AND branch = ?;

-- name: WebhooksMatchingComposeProjects2 :many
SELECT id AS id, watch_paths FROM compose_projects WHERE source_type = 'GITLAB' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitlab_repository) = lower(?) AND lower(gitlab_owner) = lower(?) AND gitlab_branch = ?;

-- name: WebhooksMatchingComposeProjects3 :many
SELECT id AS id, watch_paths FROM compose_projects WHERE source_type = 'GITEA' AND auto_deploy = 1 AND trigger_type = ? AND lower(gitea_repository) = lower(?) AND lower(gitea_owner) = lower(?) AND gitea_branch = ?;

-- name: WebhooksMatchingComposeProjects4 :many
SELECT id AS id, watch_paths FROM compose_projects WHERE source_type = 'BITBUCKET' AND auto_deploy = 1 AND trigger_type = ? AND lower(bitbucket_repository) = lower(?) AND lower(bitbucket_owner) = lower(?) AND bitbucket_branch = ?;
