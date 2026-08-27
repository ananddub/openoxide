-- ============================================================
-- APPLICATIONS
-- ============================================================

-- name: GetApplicationByID :one
SELECT * FROM applications WHERE id = ?;

-- name: GetApplicationByAppName :one
SELECT * FROM applications WHERE app_name = ?;

-- name: ListApplicationsByEnvironment :many
SELECT * FROM applications WHERE environment_id = ? ORDER BY created_at DESC;

-- name: ListApplicationsByServer :many
SELECT * FROM applications WHERE server_id = ? ORDER BY created_at DESC;

-- name: ListApplicationsByStatus :many
SELECT * FROM applications WHERE app_status = ? ORDER BY created_at DESC;

-- name: CreateApplication :one
INSERT INTO applications (
    name, app_name, description,
    source_type, build_type, app_status, trigger_type,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateApplicationStatus :exec
UPDATE applications SET app_status = ? WHERE id = ?;

-- name: UpdateApplicationSource :one
UPDATE applications
SET
    source_type           = ?,
    repository            = ?,
    owner                 = ?,
    branch                = ?,
    auto_deploy           = ?,
    github_provider_id    = ?,
    gitlab_project_id     = ?,
    gitlab_repository     = ?,
    gitlab_owner          = ?,
    gitlab_branch         = ?,
    gitlab_path_namespace = ?,
    gitea_repository      = ?,
    gitea_owner           = ?,
    gitea_branch          = ?,
    bitbucket_repository  = ?,
    bitbucket_owner       = ?,
    bitbucket_branch      = ?,
    docker_image          = ?,
    docker_username       = ?,
    docker_password       = ?,
    registry_url          = ?,
    custom_git_url        = ?,
    custom_git_branch     = ?,
    custom_git_ssh_key_id = ?
WHERE id = ?
RETURNING *;

-- name: UpdateApplicationBuildConfig :one
UPDATE applications
SET
    build_type            = ?,
    build_args            = ?,
    build_secrets         = ?,
    dockerfile            = ?,
    docker_context_path   = ?,
    docker_build_stage    = ?,
    publish_directory     = ?,
    is_static_spa         = ?,
    create_env_file       = ?,
    railpack_version      = ?,
    heroku_version        = ?,
    command               = ?,
    args                  = ?,
    env_var               = ?,
    build_path            = ?,
    clean_cache           = ?,
    enable_submodules     = ?,
    watch_paths           = ?
WHERE id = ?
RETURNING *;

-- name: UpdateApplicationResources :one
UPDATE applications
SET
    memory_reservation = ?,
    memory_limit       = ?,
    cpu_reservation    = ?,
    cpu_limit          = ?,
    replicas           = ?
WHERE id = ?
RETURNING *;

-- name: UpdateApplicationServer :exec
UPDATE applications SET server_id = ? WHERE id = ?;

-- name: UpdateApplicationEnvVar :exec
UPDATE applications SET env_var = ? WHERE id = ?;

-- name: UpdateApplicationRefreshToken :exec
UPDATE applications SET refresh_token = ? WHERE id = ?;

-- name: UpdateApplicationRollback :exec
UPDATE applications SET rollback_active = ? WHERE id = ?;

-- name: DeleteApplication :exec
DELETE FROM applications WHERE id = ?;

-- ============================================================
-- ROLLBACKS
-- ============================================================

-- name: GetRollbackByID :one
SELECT * FROM rollbacks WHERE id = ?;

-- name: ListRollbacksByDeployment :many
SELECT * FROM rollbacks WHERE deployment_id = ? ORDER BY version DESC;

-- name: GetLatestRollback :one
SELECT * FROM rollbacks
WHERE deployment_id = ?
ORDER BY version DESC
LIMIT 1;

-- name: CreateRollback :one
INSERT INTO rollbacks (deployment_id, version, image, full_context)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: DeleteRollback :exec
DELETE FROM rollbacks WHERE id = ?;

-- name: DeleteRollbacksByDeployment :exec
DELETE FROM rollbacks WHERE deployment_id = ?;
