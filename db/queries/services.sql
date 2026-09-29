-- Unified services base table (APPLICATION | COMPOSE | DATABASE).

-- name: GetServiceByID :one
SELECT * FROM services WHERE id = ?;

-- name: GetServiceByAppName :one
SELECT * FROM services WHERE app_name = ?;

-- name: ListServices :many
SELECT * FROM services ORDER BY created_at DESC, id DESC;

-- name: ListServicesByEnvironment :many
SELECT * FROM services WHERE environment_id = ? ORDER BY created_at DESC, id DESC;

-- name: ListServicesByServer :many
SELECT * FROM services WHERE server_id = ? ORDER BY created_at DESC, id DESC;

-- name: ListServicesByType :many
SELECT * FROM services WHERE service_type = ? ORDER BY created_at DESC, id DESC;

-- name: ListServicesByTypeAndEnvironment :many
SELECT * FROM services WHERE service_type = ? AND environment_id = ? ORDER BY created_at DESC, id DESC;

-- name: ListServicesByStatus :many
SELECT * FROM services WHERE app_status = ? ORDER BY created_at DESC, id DESC;

-- name: CreateService :one
INSERT INTO services (
    service_type, name, app_name, description, icon,
    app_status, env_var, command, args, refresh_token, docker_image,
    memory_reservation, memory_limit, cpu_reservation, cpu_limit, replicas,
    network_ids, detach_rustploy_network,
    environment_id, server_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: CreateServiceSimple :exec
INSERT INTO services (service_type, name, app_name, description, environment_id, server_id)
VALUES (?, ?, ?, ?, ?, ?);

-- name: DeleteService :exec
DELETE FROM services WHERE id = ?;

-- name: UpdateServiceStatus :exec
UPDATE services SET app_status = ?, last_error = NULL WHERE id = ?;

-- name: UpdateServiceStatusWithError :exec
UPDATE services SET app_status = ?, last_error = ? WHERE id = ?;

-- name: UpdateServiceResources :one
UPDATE services
SET memory_reservation = ?, memory_limit = ?,
    cpu_reservation = ?, cpu_limit = ?, replicas = ?
WHERE id = ?
RETURNING *;

-- name: UpdateServiceEnvVar :exec
UPDATE services SET env_var = ? WHERE id = ?;

-- name: UpdateServiceCommand :exec
UPDATE services SET command = ?, args = ? WHERE id = ?;

-- name: UpdateServiceDockerImage :exec
UPDATE services SET docker_image = ? WHERE id = ?;

-- name: UpdateServiceServer :exec
UPDATE services SET server_id = ? WHERE id = ?;

-- name: UpdateServiceServerIfMatch :exec
UPDATE services SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: UpdateServiceIcon :exec
UPDATE services SET icon = ? WHERE id = ?;

-- name: UpdateServiceNetworks :exec
UPDATE services SET network_ids = ?, detach_rustploy_network = ? WHERE id = ?;

-- name: RotateServiceRefreshToken :exec
UPDATE services SET refresh_token = ? WHERE id = ?;

-- name: UpdateServiceSwarmConfig :exec
UPDATE services SET
    health_check_swarm = ?, restart_policy_swarm = ?, placement_swarm = ?,
    update_config_swarm = ?, rollback_config_swarm = ?, mode_swarm = ?,
    labels_swarm = ?, network_swarm = ?, endpoint_spec_swarm = ?,
    ulimits_swarm = ?, stop_grace_period_swarm = ?
WHERE id = ?;

-- name: ServicesMoveToEnvironmentInSameOrganization :exec
UPDATE services
SET environment_id = ?, updated_at = unixepoch()
WHERE id = ?
  AND EXISTS (
      SELECT 1
      FROM environments source_environment
      INNER JOIN projects source_project ON source_project.id = source_environment.project_id
      INNER JOIN environments target_environment ON target_environment.id = ?
      INNER JOIN projects target_project ON target_project.id = target_environment.project_id
      WHERE source_environment.id = services.environment_id
        AND source_project.organization_id = target_project.organization_id
  );

-- name: ServiceNameExists :one
SELECT EXISTS(SELECT 1 FROM services WHERE app_name = ? AND id != ?) AS exists_val;

-- name: CountServicesByServer :one
SELECT COUNT(*) AS total FROM services WHERE server_id = ?;

-- name: ReassignServicesServer :exec
UPDATE services SET server_id = ? WHERE server_id = ?;

-- Deployment context for APPLICATION and COMPOSE services
-- (environment_id, project_id, server_id).

-- name: ServicesGetDeploymentContext :one
SELECT s.environment_id AS environment_id, e.project_id AS project_id, s.server_id AS server_id
FROM services s
JOIN environments e ON e.id = s.environment_id
WHERE s.id = ?;

-- name: ServicesGetAppName :one
SELECT app_name FROM services WHERE id = ?;

-- name: ServicesGetServerIdAndName :one
SELECT server_id AS server_id, app_name AS app_name FROM services WHERE id = ?;

-- name: ServicesBelongsToOrganization :one
SELECT COUNT(*) AS hit
FROM services s
JOIN environments e ON e.id = s.environment_id
JOIN projects p ON p.id = e.project_id
WHERE s.id = ? AND p.organization_id = ?;

-- Partial resource patch; NULL keeps the stored value (replicas included).

-- name: ServicesPatchResourceConfig :exec
UPDATE services
SET memory_reservation = COALESCE(?, memory_reservation),
    memory_limit = COALESCE(?, memory_limit),
    cpu_reservation = COALESCE(?, cpu_reservation),
    cpu_limit = COALESCE(?, cpu_limit),
    replicas = COALESCE(?, replicas)
WHERE id = ?;

-- APPLICATION core CRUD helpers (src/services/application).
-- Explicit NOT NULL values replace the old `applications` table defaults
-- (app_status 'IDLE', network_ids '[]', detach 0, replicas 1); the unified
-- services table has no column defaults for them. Split With/NoServer
-- variants keep server_id NULL round-trippable (bind params are non-Option).

-- name: AppCoreCreateApplicationWithServer :one
INSERT INTO services (
    service_type, name, app_name, description, app_status,
    network_ids, detach_rustploy_network, replicas,
    environment_id, server_id
) VALUES ('APPLICATION', ?, ?, ?, 'IDLE', '[]', 0, 1, ?, ?)
RETURNING *;

-- name: AppCoreCreateApplicationNoServer :one
INSERT INTO services (
    service_type, name, app_name, description, app_status,
    network_ids, detach_rustploy_network, replicas,
    environment_id
) VALUES ('APPLICATION', ?, ?, ?, 'IDLE', '[]', 0, 1, ?)
RETURNING *;

-- Old ApplicationRepository::patch rewrote name/description/env_var/icon/
-- network_ids/detach on the single applications row. Nullable columns are
-- split into dedicated statements so NULL never has to be bound; the core
-- update carries only NOT NULL columns.

-- name: AppCorePatchApplication :exec
UPDATE services
SET name = ?, network_ids = ?, detach_rustploy_network = ?
WHERE id = ?;

-- name: AppCorePatchDescription :exec
UPDATE services SET description = ? WHERE id = ?;

-- Full deploy spec for an APPLICATION service, resolved across
-- services + service_application + service_git + git_providers.
-- source_type: service_git for git sources, else docker/drop by
-- services.docker_image / service_application.drop_build_path.

-- name: ApplicationGetSpecRow :one
SELECT
    s.app_name AS app_name,
    COALESCE(g.source_type,
        CASE WHEN s.docker_image IS NOT NULL THEN 'DOCKER'
             WHEN a.drop_build_path IS NOT NULL THEN 'DROP'
             ELSE 'NONE' END) AS source_type,
    a.build_type AS build_type,
    a.build_args AS build_args,
    a.build_secrets AS build_secrets,
    a.dockerfile AS dockerfile,
    a.docker_context_path AS docker_context_path,
    a.docker_build_stage AS docker_build_stage,
    a.publish_directory AS publish_directory,
    a.is_static_spa AS is_static_spa,
    a.railpack_version AS railpack_version,
    g.enable_submodules AS enable_submodules,
    COALESCE(CASE WHEN g.source_type IS NULL AND s.docker_image IS NULL AND a.drop_build_path IS NOT NULL THEN a.drop_build_path ELSE a.build_path END, '') AS build_path,
    COALESCE(s.command, '') AS command,
    s.args AS args,
    s.env_var AS env_var,
    COALESCE(a.clean_cache, 0) AS clean_cache,
    s.network_ids AS network_ids,
    s.detach_rustploy_network AS detach_rustploy_network,
    s.memory_reservation AS memory_reservation,
    s.memory_limit AS memory_limit,
    s.cpu_reservation AS cpu_reservation,
    s.cpu_limit AS cpu_limit,
    s.replicas AS replicas,
    s.health_check_swarm AS health_check_swarm,
    s.placement_swarm AS placement_swarm,
    s.stop_grace_period_swarm AS stop_grace_period_swarm,
    g.repository AS repository,
    g.owner AS owner,
    g.branch AS branch,
    g.gitlab_repository AS gitlab_repository,
    g.gitlab_owner AS gitlab_owner,
    g.gitlab_branch AS gitlab_branch,
    g.gitea_repository AS gitea_repository,
    g.gitea_branch AS gitea_branch,
    g.bitbucket_repository AS bitbucket_repository,
    g.bitbucket_owner AS bitbucket_owner,
    g.bitbucket_branch AS bitbucket_branch,
    s.docker_image AS docker_image,
    a.docker_username AS docker_username,
    a.docker_password AS docker_password,
    a.registry_url AS registry_url,
    g.custom_git_url AS custom_git_url,
    g.custom_git_branch AS custom_git_branch,
    g.custom_git_ssh_key_id AS custom_git_ssh_key_id,
    e.env_var AS environment_env,
    p.env_var AS project_env,
    r.image_prefix AS registry_image_prefix,
    COALESCE(r.username, '') AS registry_username,
    COALESCE(r.password, '') AS registry_password,
    COALESCE(r.registry_url, '') AS joined_registry_url,
    gpgh.config_json AS github_config,
    gpgl.config_json AS gitlab_config,
    gpbb.config_json AS bitbucket_config,
    gpgt.config_json AS gitea_config,
    sk.private_key AS ssh_private_key
FROM services s
JOIN service_application a ON a.service_id = s.id
JOIN environments e ON e.id = s.environment_id
JOIN projects p ON p.id = e.project_id
LEFT JOIN service_git g ON g.service_id = s.id
LEFT JOIN registries r ON r.id = a.registry_id
LEFT JOIN git_providers gpgh ON gpgh.id = g.github_provider_id
LEFT JOIN git_providers gpgl ON gpgl.id = g.gitlab_provider_id
LEFT JOIN git_providers gpbb ON gpbb.id = g.bitbucket_provider_id
LEFT JOIN git_providers gpgt ON gpgt.id = g.gitea_provider_id
LEFT JOIN ssh_keys sk ON sk.id = g.custom_git_ssh_key_id
WHERE s.id = ?;
