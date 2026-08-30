-- Git/VCS source config for APPLICATION and COMPOSE services.

-- name: GetServiceGitByID :one
SELECT * FROM service_git WHERE service_id = ?;

-- name: CreateServiceGit :one
INSERT INTO service_git (
    service_id, source_type, trigger_type, auto_deploy, watch_paths, enable_submodules
) VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: DeleteServiceGit :exec
DELETE FROM service_git WHERE service_id = ?;

-- name: UpdateServiceGitTrigger :exec
UPDATE service_git SET trigger_type = ?, auto_deploy = ? WHERE service_id = ?;

-- name: SetDockerSource :exec
DELETE FROM service_git WHERE service_id = ?;

-- name: SetGithubSource :exec
INSERT INTO service_git (
    service_id, source_type, trigger_type, repository, owner, branch,
    github_provider_id, auto_deploy, watch_paths, enable_submodules
) VALUES (?, 'GITHUB', ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(service_id) DO UPDATE SET
    source_type = 'GITHUB', trigger_type = excluded.trigger_type,
    repository = excluded.repository, owner = excluded.owner, branch = excluded.branch,
    github_provider_id = excluded.github_provider_id, auto_deploy = excluded.auto_deploy,
    watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules,
    gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
    gitlab_build_path = NULL, gitlab_path_namespace = NULL,
    gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL,
    bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
    bitbucket_branch = NULL, bitbucket_build_path = NULL,
    custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL,
    custom_git_ssh_key_id = NULL;

-- name: SetGitlabSource :exec
INSERT INTO service_git (
    service_id, source_type, trigger_type, gitlab_project_id, gitlab_repository, gitlab_owner,
    gitlab_branch, gitlab_build_path, gitlab_path_namespace, gitlab_provider_id,
    auto_deploy, watch_paths, enable_submodules
) VALUES (?, 'GITLAB', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(service_id) DO UPDATE SET
    source_type = 'GITLAB', trigger_type = excluded.trigger_type,
    gitlab_project_id = excluded.gitlab_project_id, gitlab_repository = excluded.gitlab_repository,
    gitlab_owner = excluded.gitlab_owner, gitlab_branch = excluded.gitlab_branch,
    gitlab_build_path = excluded.gitlab_build_path, gitlab_path_namespace = excluded.gitlab_path_namespace,
    gitlab_provider_id = excluded.gitlab_provider_id, auto_deploy = excluded.auto_deploy,
    watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules,
    repository = NULL, owner = NULL, branch = NULL,
    gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL,
    bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
    bitbucket_branch = NULL, bitbucket_build_path = NULL,
    custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL,
    custom_git_ssh_key_id = NULL;

-- name: SetGiteaSource :exec
INSERT INTO service_git (
    service_id, source_type, trigger_type, gitea_repository, gitea_owner, gitea_branch,
    gitea_build_path, gitea_provider_id, auto_deploy, watch_paths, enable_submodules
) VALUES (?, 'GITEA', ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(service_id) DO UPDATE SET
    source_type = 'GITEA', trigger_type = excluded.trigger_type,
    gitea_repository = excluded.gitea_repository, gitea_owner = excluded.gitea_owner,
    gitea_branch = excluded.gitea_branch, gitea_build_path = excluded.gitea_build_path,
    gitea_provider_id = excluded.gitea_provider_id, auto_deploy = excluded.auto_deploy,
    watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules,
    repository = NULL, owner = NULL, branch = NULL,
    gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
    gitlab_build_path = NULL, gitlab_path_namespace = NULL,
    bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
    bitbucket_branch = NULL, bitbucket_build_path = NULL,
    custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL,
    custom_git_ssh_key_id = NULL;

-- name: SetBitbucketSource :exec
INSERT INTO service_git (
    service_id, source_type, trigger_type, bitbucket_repository, bitbucket_repository_slug,
    bitbucket_owner, bitbucket_branch, bitbucket_build_path, bitbucket_provider_id,
    auto_deploy, watch_paths, enable_submodules
) VALUES (?, 'BITBUCKET', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(service_id) DO UPDATE SET
    source_type = 'BITBUCKET', trigger_type = excluded.trigger_type,
    bitbucket_repository = excluded.bitbucket_repository,
    bitbucket_repository_slug = excluded.bitbucket_repository_slug,
    bitbucket_owner = excluded.bitbucket_owner, bitbucket_branch = excluded.bitbucket_branch,
    bitbucket_build_path = excluded.bitbucket_build_path,
    bitbucket_provider_id = excluded.bitbucket_provider_id, auto_deploy = excluded.auto_deploy,
    watch_paths = excluded.watch_paths, enable_submodules = excluded.enable_submodules,
    repository = NULL, owner = NULL, branch = NULL,
    gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
    gitlab_build_path = NULL, gitlab_path_namespace = NULL,
    gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL,
    custom_git_url = NULL, custom_git_branch = NULL, custom_git_build_path = NULL,
    custom_git_ssh_key_id = NULL;

-- name: SetCustomGitSource :exec
INSERT INTO service_git (
    service_id, source_type, trigger_type, custom_git_url, custom_git_branch,
    custom_git_build_path, custom_git_ssh_key_id, auto_deploy, watch_paths, enable_submodules
) VALUES (?, 'CUSTOM', ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(service_id) DO UPDATE SET
    source_type = 'CUSTOM', trigger_type = excluded.trigger_type,
    custom_git_url = excluded.custom_git_url, custom_git_branch = excluded.custom_git_branch,
    custom_git_build_path = excluded.custom_git_build_path,
    custom_git_ssh_key_id = excluded.custom_git_ssh_key_id,
    auto_deploy = excluded.auto_deploy, watch_paths = excluded.watch_paths,
    enable_submodules = excluded.enable_submodules,
    repository = NULL, owner = NULL, branch = NULL,
    gitlab_project_id = NULL, gitlab_repository = NULL, gitlab_owner = NULL, gitlab_branch = NULL,
    gitlab_build_path = NULL, gitlab_path_namespace = NULL,
    gitea_repository = NULL, gitea_owner = NULL, gitea_branch = NULL, gitea_build_path = NULL,
    bitbucket_repository = NULL, bitbucket_repository_slug = NULL, bitbucket_owner = NULL,
    bitbucket_branch = NULL, bitbucket_build_path = NULL;

-- Webhook-triggered auto-deploy lookups (github/gitlab/gitea/bitbucket across
-- APPLICATION and COMPOSE services).

-- name: FindServicesByGithubRepo :many
SELECT s.id AS id, g.watch_paths AS watch_paths
FROM services s
JOIN service_git g ON g.service_id = s.id
WHERE s.service_type = ? AND g.source_type = 'GITHUB' AND g.auto_deploy = 1 AND g.trigger_type = ?
  AND lower(g.repository) = lower(?) AND lower(g.owner) = lower(?) AND g.branch = ?;

-- name: FindServicesByGitlabRepo :many
SELECT s.id AS id, g.watch_paths AS watch_paths
FROM services s
JOIN service_git g ON g.service_id = s.id
WHERE s.service_type = ? AND g.source_type = 'GITLAB' AND g.auto_deploy = 1 AND g.trigger_type = ?
  AND lower(g.gitlab_repository) = lower(?) AND lower(g.gitlab_owner) = lower(?) AND g.gitlab_branch = ?;

-- name: FindServicesByGiteaRepo :many
SELECT s.id AS id, g.watch_paths AS watch_paths
FROM services s
JOIN service_git g ON g.service_id = s.id
WHERE s.service_type = ? AND g.source_type = 'GITEA' AND g.auto_deploy = 1 AND g.trigger_type = ?
  AND lower(g.gitea_repository) = lower(?) AND lower(g.gitea_owner) = lower(?) AND g.gitea_branch = ?;

-- name: FindServicesByBitbucketRepo :many
SELECT s.id AS id, g.watch_paths AS watch_paths
FROM services s
JOIN service_git g ON g.service_id = s.id
WHERE s.service_type = ? AND g.source_type = 'BITBUCKET' AND g.auto_deploy = 1 AND g.trigger_type = ?
  AND lower(g.bitbucket_repository) = lower(?) AND lower(g.bitbucket_owner) = lower(?) AND g.bitbucket_branch = ?;

-- name: CountServicesUsingSshKey :one
SELECT COUNT(*) AS total FROM service_git WHERE custom_git_ssh_key_id = ?;

-- name: CountServicesUsingGitProvider :one
SELECT COUNT(*) AS total FROM service_git
WHERE github_provider_id = ? OR gitlab_provider_id = ? OR gitea_provider_id = ? OR bitbucket_provider_id = ?;

-- APPLICATION core helpers (src/services/application).
-- Source row for a freshly created APPLICATION service; preserves the old
-- create_simple behaviour of storing the requested source type verbatim
-- (auto_deploy default 1, enable_submodules default 0 from the old table;
-- watch_paths stays NULL like the old default).

-- name: AppCoreCreateServiceGitSource :one
INSERT INTO service_git (service_id, source_type, trigger_type, auto_deploy, enable_submodules)
VALUES (?, ?, ?, 1, 0)
RETURNING *;

-- name: AppCorePatchTriggerType :exec
UPDATE service_git SET trigger_type = ? WHERE service_id = ?;
