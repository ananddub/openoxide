-- ============================================================
-- COMPOSE PROJECTS
-- ============================================================

-- name: GetComposeByID :one
SELECT * FROM compose_projects WHERE id = ?;

-- name: GetComposeByAppName :one
SELECT * FROM compose_projects WHERE app_name = ?;

-- name: ListComposeByEnvironment :many
SELECT * FROM compose_projects WHERE environment_id = ? ORDER BY created_at DESC;

-- name: ListComposeByServer :many
SELECT * FROM compose_projects WHERE server_id = ? ORDER BY created_at DESC;

-- name: ListComposeByStatus :many
SELECT * FROM compose_projects WHERE compose_status = ? ORDER BY created_at DESC;

-- name: CreateCompose :one
INSERT INTO compose_projects (
    name, app_name, description, env_var,
    compose_file, source_type, compose_type, trigger_type,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateComposeStatus :exec
UPDATE compose_projects SET compose_status = ? WHERE id = ?;

-- name: UpdateComposeSource :one
UPDATE compose_projects
SET
    source_type              = ?,
    repository               = ?,
    owner                    = ?,
    branch                   = ?,
    auto_deploy              = ?,
    gitlab_project_id        = ?,
    gitlab_repository        = ?,
    gitlab_owner             = ?,
    gitlab_branch            = ?,
    gitlab_path_namespace    = ?,
    bitbucket_repository     = ?,
    bitbucket_repository_slug = ?,
    bitbucket_owner          = ?,
    bitbucket_branch         = ?,
    gitea_repository         = ?,
    gitea_owner              = ?,
    gitea_branch             = ?,
    custom_git_url           = ?,
    custom_git_branch        = ?,
    custom_git_ssh_key_id    = ?
WHERE id = ?
RETURNING *;

-- name: UpdateComposeConfig :one
UPDATE compose_projects
SET
    compose_file              = ?,
    env_var                   = ?,
    command                   = ?,
    compose_path              = ?,
    suffix                    = ?,
    randomize                 = ?,
    isolated_deployment       = ?,
    isolated_deployments_volume = ?,
    enable_submodules         = ?,
    watch_paths               = ?
WHERE id = ?
RETURNING *;

-- name: UpdateComposeServer :exec
UPDATE compose_projects SET server_id = ? WHERE id = ?;

-- name: UpdateComposeRefreshToken :exec
UPDATE compose_projects SET refresh_token = ? WHERE id = ?;

-- name: DeleteCompose :exec
DELETE FROM compose_projects WHERE id = ?;
