-- Build config and preview deployments for APPLICATION services.

-- name: GetServiceApplication :one
SELECT * FROM service_application WHERE service_id = ?;

-- name: CreateServiceApplication :one
INSERT INTO service_application (
    service_id, build_type, create_env_file, clean_cache, rollback_active,
    preview_https, preview_certificate_type, is_preview_deployments_active,
    preview_require_collaborator_permissions
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: DeleteServiceApplication :exec
DELETE FROM service_application WHERE service_id = ?;

-- name: UpdateBuildConfig :one
UPDATE service_application
SET
    build_type          = ?,
    build_args          = ?,
    build_secrets       = ?,
    dockerfile          = ?,
    docker_context_path = ?,
    docker_build_stage  = ?,
    publish_directory   = ?,
    is_static_spa       = ?,
    railpack_version    = ?,
    heroku_version      = ?,
    build_path          = ?,
    clean_cache         = ?,
    drop_build_path     = ?
WHERE service_id = ?
RETURNING *;

-- name: PatchBuildConfig :exec
UPDATE service_application
SET
    build_args          = COALESCE(?, build_args),
    build_secrets       = COALESCE(?, build_secrets),
    dockerfile          = COALESCE(?, dockerfile),
    docker_context_path = COALESCE(?, docker_context_path),
    docker_build_stage  = COALESCE(?, docker_build_stage),
    publish_directory   = COALESCE(?, publish_directory),
    is_static_spa       = COALESCE(?, is_static_spa),
    railpack_version    = COALESCE(?, railpack_version),
    heroku_version      = COALESCE(?, heroku_version),
    build_path          = COALESCE(?, build_path),
    clean_cache         = COALESCE(?, clean_cache),
    drop_build_path     = COALESCE(?, drop_build_path)
WHERE service_id = ?;

-- name: UpdatePreviewConfig :exec
UPDATE service_application
SET preview_env = ?, preview_build_args = ?, preview_build_secrets = ?, preview_labels = ?,
    preview_wildcard = ?, preview_port = ?, preview_https = ?, preview_path = ?,
    preview_certificate_type = ?, preview_custom_cert_resolver = ?, preview_limit = ?,
    is_preview_deployments_active = ?, preview_require_collaborator_permissions = ?
WHERE service_id = ?;

-- name: UpdateRollbackActive :exec
UPDATE service_application SET rollback_active = ? WHERE service_id = ?;

-- name: UpdateDockerRegistryCredentials :exec
UPDATE service_application
SET docker_username = ?, docker_password = ?, registry_url = ?
WHERE service_id = ?;

-- name: UpdateRegistryRefs :exec
UPDATE service_application
SET build_server_id = ?, registry_id = ?, rollback_registry_id = ?, build_registry_id = ?
WHERE service_id = ?;

-- Switch an APPLICATION service to a DROP-in source. Run in order:
-- 1) remove the git source row, 2) set drop_build_path (and clear any
-- docker registry credentials), 3) clear the docker image on the base
-- service. (1) reuses DeleteServiceGit in service_git.sql.

-- name: ApplicationSetDropSource :exec
UPDATE service_application
SET drop_build_path = ?, docker_username = NULL, docker_password = NULL, registry_url = NULL
WHERE service_id = ?;

-- name: ApplicationClearDockerImage :exec
UPDATE services SET docker_image = NULL WHERE id = ?;

-- APPLICATION core helpers (src/services/application).
-- Build-config row for a freshly created APPLICATION service carrying the
-- old `applications` table defaults (preview_port 3000, preview_path '/',
-- preview_limit 3 — the unified service_application table has no defaults
-- for these nullable columns).

-- name: AppCoreCreateApplicationBuildConfig :one
INSERT INTO service_application (
    service_id, build_type, create_env_file, clean_cache, rollback_active,
    preview_https, preview_certificate_type, is_preview_deployments_active,
    preview_require_collaborator_permissions,
    preview_port, preview_path, preview_limit
) VALUES (?, ?, 1, 0, 0, 0, 'NONE', 0, 1, 3000, '/', 3)
RETURNING *;

-- Targeted patch statements; each is only executed with a concrete value
-- (old ApplicationRepository::patch computed input.or(current), so NULL was
-- never actually introduced by a patch).

-- name: AppCorePatchBuildType :exec
UPDATE service_application SET build_type = ? WHERE service_id = ?;

-- name: AppCorePatchBuildServer :exec
UPDATE service_application SET build_server_id = ? WHERE service_id = ?;

-- name: AppCorePatchRegistry :exec
UPDATE service_application SET registry_id = ? WHERE service_id = ?;
