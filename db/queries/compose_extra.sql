-- ============================================================
-- COMPOSE EXTRA
-- Compose-service-local queries (src/services/compose/*).
-- Names are prefixed with `Compose` and globally unique to avoid
-- collisions with query files owned by other migration agents.
-- ============================================================

-- Old DeploymentRepository::create_queued_compose_deployment returned the new
-- row id; the generated DeploymentsCreateQueuedComposeDeployment is :exec.
-- Split With/NoServer keeps a NULL server_id insertable because bind params
-- for nullable columns are non-Option.

-- name: ComposeQueueDeploymentWithServer :one
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, server_id, is_preview_deployment, user_id, last_state_at)
VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, 0, ?, unixepoch())
RETURNING id;

-- name: ComposeQueueDeploymentNoServer :one
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, is_preview_deployment, user_id, last_state_at)
VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, 0, ?, unixepoch())
RETURNING id;

-- Old ComposeProjectRepository::create_simple mapped onto the unified
-- services table (service_type = 'COMPOSE'). Split With/NoServer keeps a NULL
-- server_id insertable; explicit values satisfy the NOT NULL columns without
-- defaults (app_status, replicas, network_ids, detach_rustploy_network).

-- name: ComposeCreateServiceWithServer :one
INSERT INTO services (
    service_type, name, app_name, description, app_status,
    network_ids, detach_rustploy_network, replicas,
    environment_id, server_id
) VALUES ('COMPOSE', ?, ?, ?, 'IDLE', '[]', 0, 1, ?, ?)
RETURNING *;

-- name: ComposeCreateServiceNoServer :one
INSERT INTO services (
    service_type, name, app_name, description, app_status,
    network_ids, detach_rustploy_network, replicas,
    environment_id
) VALUES ('COMPOSE', ?, ?, ?, 'IDLE', '[]', 0, 1, ?)
RETURNING *;

-- Old ComposeProjectRepository::move_to_environment_in_same_organization
-- returned Option<ComposeProject> (rows_affected == 0 -> None); :one
-- RETURNING id maps RowNotFound back to None. The same-organization guard is
-- preserved verbatim from the original SQL.

-- name: ComposeMoveToEnvironment :one
UPDATE services
   SET environment_id = ?, updated_at = unixepoch()
 WHERE id = ? AND EXISTS (
     SELECT 1 FROM environments source_environment
     JOIN projects source_project ON source_project.id = source_environment.project_id
     JOIN environments target_environment ON target_environment.id = ?
     JOIN projects target_project ON target_project.id = target_environment.project_id
     WHERE source_environment.id = services.environment_id
       AND source_project.organization_id = target_project.organization_id
   )
RETURNING id;

-- Old ComposeProjectRepository::rotate_refresh_token returned
-- rows_affected == 1; :one RETURNING id + .is_ok() preserves that.

-- name: ComposeRotateWebhookToken :one
UPDATE services SET refresh_token = ?, updated_at = unixepoch() WHERE id = ?
RETURNING id;

-- Mount CRUD for compose. Old MountRepository create/update/delete_for_compose
-- returned the row / Option<row> / rows-affected bool respectively.

-- name: ComposeMountCreate :one
INSERT INTO mounts
    (mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id)
VALUES (?, 'COMPOSE', ?, ?, ?, ?, ?, ?)
RETURNING id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id, created_at, updated_at;

-- name: ComposeMountUpdate :one
UPDATE mounts
   SET mount_type = ?, service_type = 'COMPOSE', host_path = ?, volume_name = ?,
       file_path = ?, content = ?, mount_path = ?, updated_at = unixepoch()
 WHERE id = ? AND service_id = ?
RETURNING id, mount_type, service_type, host_path, volume_name, file_path, content, mount_path, service_id, created_at, updated_at;

-- name: ComposeMountDelete :one
DELETE FROM mounts WHERE id = ? AND service_id = ? RETURNING id;

-- Patch CRUD for compose. Old PatchRepository create/update/delete_for_compose
-- returned the row / Option<row> / rows-affected bool respectively.

-- name: ComposePatchCreate :one
INSERT INTO patches (patch_type, file_path, enabled, content, service_id)
VALUES (?, ?, ?, ?, ?)
RETURNING id, patch_type, file_path, enabled, content, service_id, created_at, updated_at;

-- name: ComposePatchUpdate :one
UPDATE patches SET patch_type = ?, file_path = ?, enabled = ?, content = ?
WHERE id = ? AND service_id = ?
RETURNING id, patch_type, file_path, enabled, content, service_id, created_at, updated_at;

-- name: ComposePatchDelete :one
DELETE FROM patches WHERE id = ? AND service_id = ? RETURNING id;
