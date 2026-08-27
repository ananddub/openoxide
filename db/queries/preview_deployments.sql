-- Extracted from src/db/repository/preview_deployments.rs. Do not edit repository SQL directly.

-- name: PreviewDeploymentsList1 :many
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE status != 'CLOSED' ORDER BY created_at DESC;

-- name: PreviewDeploymentsList2 :many
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments ORDER BY created_at DESC;

-- name: PreviewDeploymentsGetById :one
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE id = ?;

-- name: PreviewDeploymentsListOpenForBaseApplication :many
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE base_application_id = ? AND status != 'CLOSED' ORDER BY created_at;

-- name: PreviewDeploymentsFindByPreviewApplication :one
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE preview_application_id = ?;

-- name: PreviewDeploymentsFindForPullRequest :one
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE base_application_id = ? AND provider_type = ? AND pull_request_number = ?;

-- name: PreviewDeploymentsFindOpenForEvent :many
SELECT id AS id, base_application_id AS base_application_id, preview_application_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE provider_type = ? AND lower(owner) = lower(?) AND lower(repository) = lower(?) AND pull_request_number = ? AND status != 'CLOSED';

-- name: PreviewDeploymentsMatchingTargets1 :many
SELECT CAST(a.id AS INTEGER) AS base_application_id, gp.id AS provider_id, COALESCE(a.preview_limit, 3) AS preview_limit, a.preview_require_collaborator_permissions != 0 AS require_collaborator_permissions FROM applications a JOIN github_providers p ON p.id = a.github_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'GITHUB' AND a.is_preview_deployments_active = 1 AND lower(a.repository) = lower(?) AND lower(a.owner) = lower(?) AND a.branch = ?;

-- name: PreviewDeploymentsMatchingTargets2 :many
SELECT CAST(a.id AS INTEGER) AS base_application_id, gp.id AS provider_id, COALESCE(a.preview_limit, 3) AS preview_limit, a.preview_require_collaborator_permissions != 0 AS require_collaborator_permissions FROM applications a JOIN gitlab_providers p ON p.id = a.gitlab_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'GITLAB' AND a.is_preview_deployments_active = 1 AND lower(a.gitlab_repository) = lower(?) AND lower(a.gitlab_owner) = lower(?) AND a.gitlab_branch = ?;

-- name: PreviewDeploymentsMatchingTargets3 :many
SELECT CAST(a.id AS INTEGER) AS base_application_id, gp.id AS provider_id, COALESCE(a.preview_limit, 3) AS preview_limit, a.preview_require_collaborator_permissions != 0 AS require_collaborator_permissions FROM applications a JOIN gitea_providers p ON p.id = a.gitea_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'GITEA' AND a.is_preview_deployments_active = 1 AND lower(a.gitea_repository) = lower(?) AND lower(a.gitea_owner) = lower(?) AND a.gitea_branch = ?;

-- name: PreviewDeploymentsMatchingTargets4 :many
SELECT CAST(a.id AS INTEGER) AS base_application_id, gp.id AS provider_id, COALESCE(a.preview_limit, 3) AS preview_limit, a.preview_require_collaborator_permissions != 0 AS require_collaborator_permissions FROM applications a JOIN bitbucket_providers p ON p.id = a.bitbucket_provider_id JOIN git_providers gp ON gp.id = p.git_provider_id WHERE a.source_type = 'BITBUCKET' AND a.is_preview_deployments_active = 1 AND lower(a.bitbucket_repository) = lower(?) AND lower(a.bitbucket_owner) = lower(?) AND a.bitbucket_branch = ?;

-- name: PreviewDeploymentsActiveCount :one
SELECT COUNT(*) AS count FROM preview_deployments WHERE base_application_id = ? AND status != 'CLOSED';

-- name: PreviewDeploymentsCreate :exec
INSERT INTO preview_deployments (base_application_id, preview_application_id, provider_type, provider_id, owner, repository, pull_request_number, source_branch, source_owner, source_repository, target_branch, commit_sha, author, status, domain) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'QUEUED', ?);

-- name: PreviewDeploymentsUpdateSource :exec
UPDATE preview_deployments SET source_branch = ?, source_owner = ?, source_repository = ?, commit_sha = ?, author = ?, status = 'QUEUED', updated_at = strftime('%s', 'now') WHERE id = ?;

-- name: PreviewDeploymentsReopen :exec
UPDATE preview_deployments SET preview_application_id = ?, source_branch = ?, source_owner = ?, source_repository = ?, target_branch = ?, commit_sha = ?, author = ?, domain = ?, last_deployment_id = NULL, status = 'QUEUED', updated_at = strftime('%s', 'now') WHERE id = ?;

-- name: PreviewDeploymentsLinkDeployment1 :exec
UPDATE deployments SET is_preview_deployment = 1 WHERE id = ?;

-- name: PreviewDeploymentsLinkDeployment2 :exec
UPDATE preview_deployments SET last_deployment_id = ?, status = 'QUEUED', updated_at = strftime('%s', 'now') WHERE id = ?;

-- name: PreviewDeploymentsSetStatus :exec
UPDATE preview_deployments SET status = ?, updated_at = strftime('%s', 'now') WHERE id = ?;

-- name: PreviewDeploymentsClose :exec
UPDATE preview_deployments SET preview_application_id = NULL, status = 'CLOSED', updated_at = strftime('%s', 'now') WHERE id = ?;

-- name: PreviewDeploymentsDelete :exec
DELETE FROM preview_deployments WHERE id = ?;
