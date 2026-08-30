-- Extracted from src/db/repository/preview_deployments.rs. Do not edit repository SQL directly.
-- Migrated to the unified services schema: base_service_id/preview_service_id
-- reference services(id). CURRENT_TIMESTAMP replaced with unixepoch() because
-- updated_at is a STRICT INTEGER column.

-- name: PreviewDeploymentsList1 :many
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE status != 'CLOSED' ORDER BY created_at DESC;

-- name: PreviewDeploymentsList2 :many
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments ORDER BY created_at DESC;

-- name: PreviewDeploymentsGetById :one
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE id = ?;

-- name: PreviewDeploymentsListOpenForBaseApplication :many
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE base_service_id = ? AND status != 'CLOSED' ORDER BY created_at;

-- name: PreviewDeploymentsFindByPreviewApplication :one
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE preview_service_id = ?;

-- name: PreviewDeploymentsFindForPullRequest :one
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE base_service_id = ? AND provider_type = ? AND pull_request_number = ?;

-- name: PreviewDeploymentsFindOpenForEvent :many
SELECT id AS id, base_service_id AS base_service_id, preview_service_id, provider_type AS provider_type, provider_id AS provider_id, owner AS owner, repository AS repository, pull_request_number AS pull_request_number, source_branch AS source_branch, source_owner, source_repository, target_branch AS target_branch, commit_sha, author, status AS status, domain AS domain, last_deployment_id, created_at AS created_at, updated_at AS updated_at FROM preview_deployments WHERE provider_type = ? AND lower(owner) = lower(?) AND lower(repository) = lower(?) AND pull_request_number = ? AND status != 'CLOSED';

-- name: PreviewDeploymentsMatchingTargets1 :many
SELECT CAST(s.id AS BIGINT) AS base_service_id, CAST(gp.id AS BIGINT) AS provider_id, CAST(COALESCE(sa.preview_limit, 3) AS BIGINT) AS preview_limit, sa.preview_require_collaborator_permissions AS require_collaborator_permissions FROM services s JOIN service_git g ON g.service_id = s.id JOIN service_application sa ON sa.service_id = s.id JOIN git_providers gp ON gp.id = g.github_provider_id WHERE g.source_type = 'GITHUB' AND sa.is_preview_deployments_active = 1 AND lower(g.repository) = lower(?) AND lower(g.owner) = lower(?) AND g.branch = ?;

-- name: PreviewDeploymentsMatchingTargets2 :many
SELECT CAST(s.id AS BIGINT) AS base_service_id, CAST(gp.id AS BIGINT) AS provider_id, CAST(COALESCE(sa.preview_limit, 3) AS BIGINT) AS preview_limit, sa.preview_require_collaborator_permissions AS require_collaborator_permissions FROM services s JOIN service_git g ON g.service_id = s.id JOIN service_application sa ON sa.service_id = s.id JOIN git_providers gp ON gp.id = g.gitlab_provider_id WHERE g.source_type = 'GITLAB' AND sa.is_preview_deployments_active = 1 AND lower(g.gitlab_repository) = lower(?) AND lower(g.gitlab_owner) = lower(?) AND g.gitlab_branch = ?;

-- name: PreviewDeploymentsMatchingTargets3 :many
SELECT CAST(s.id AS BIGINT) AS base_service_id, CAST(gp.id AS BIGINT) AS provider_id, CAST(COALESCE(sa.preview_limit, 3) AS BIGINT) AS preview_limit, sa.preview_require_collaborator_permissions AS require_collaborator_permissions FROM services s JOIN service_git g ON g.service_id = s.id JOIN service_application sa ON sa.service_id = s.id JOIN git_providers gp ON gp.id = g.gitea_provider_id WHERE g.source_type = 'GITEA' AND sa.is_preview_deployments_active = 1 AND lower(g.gitea_repository) = lower(?) AND lower(g.gitea_owner) = lower(?) AND g.gitea_branch = ?;

-- name: PreviewDeploymentsMatchingTargets4 :many
SELECT CAST(s.id AS BIGINT) AS base_service_id, CAST(gp.id AS BIGINT) AS provider_id, CAST(COALESCE(sa.preview_limit, 3) AS BIGINT) AS preview_limit, sa.preview_require_collaborator_permissions AS require_collaborator_permissions FROM services s JOIN service_git g ON g.service_id = s.id JOIN service_application sa ON sa.service_id = s.id JOIN git_providers gp ON gp.id = g.bitbucket_provider_id WHERE g.source_type = 'BITBUCKET' AND sa.is_preview_deployments_active = 1 AND lower(g.bitbucket_repository) = lower(?) AND lower(g.bitbucket_owner) = lower(?) AND g.bitbucket_branch = ?;

-- name: PreviewDeploymentsActiveCount :one
SELECT COUNT(*) AS count FROM preview_deployments WHERE base_service_id = ? AND status != 'CLOSED';

-- name: PreviewDeploymentsCreate :one
INSERT INTO preview_deployments (base_service_id, preview_service_id, provider_type, provider_id, owner, repository, pull_request_number, source_branch, source_owner, source_repository, target_branch, commit_sha, author, status, domain, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'QUEUED', ?, ?, ?) RETURNING id AS id;

-- name: PreviewDeploymentsUpdateSource :exec
UPDATE preview_deployments SET source_branch = ?, source_owner = ?, source_repository = ?, commit_sha = ?, author = ?, status = 'QUEUED', updated_at = unixepoch() WHERE id = ?;

-- name: PreviewDeploymentsReopen :exec
UPDATE preview_deployments SET preview_service_id = ?, source_branch = ?, source_owner = ?, source_repository = ?, target_branch = ?, commit_sha = ?, author = ?, domain = ?, last_deployment_id = NULL, status = 'QUEUED', updated_at = unixepoch() WHERE id = ?;

-- name: PreviewDeploymentsLinkDeployment1 :one
UPDATE deployments SET is_preview_deployment = 1 WHERE id = ? RETURNING id AS id;

-- name: PreviewDeploymentsLinkDeployment2 :one
UPDATE preview_deployments SET last_deployment_id = ?, status = 'QUEUED', updated_at = unixepoch() WHERE id = ? RETURNING id AS id;

-- name: PreviewDeploymentsSetStatus :exec
UPDATE preview_deployments SET status = ?, updated_at = unixepoch() WHERE id = ?;

-- name: PreviewDeploymentsClose :exec
UPDATE preview_deployments SET preview_service_id = NULL, status = 'CLOSED', updated_at = unixepoch() WHERE id = ?;

-- name: PreviewDeploymentsDelete :exec
DELETE FROM preview_deployments WHERE id = ?;
