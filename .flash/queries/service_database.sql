-- Managed databases (service_type = DATABASE).
-- Engine-specific credentials/config live in service_database.engine_config (JSON).

-- name: GetServiceDatabase :one
SELECT * FROM service_database WHERE service_id = ?;

-- name: ListDatabasesByType :many
SELECT * FROM service_database WHERE db_type = ?;

-- name: CreateServiceDatabase :one
INSERT INTO service_database (service_id, db_type, external_port, db_name, db_user, db_password, engine_config)
VALUES (?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: DeleteServiceDatabase :exec
DELETE FROM service_database WHERE service_id = ?;

-- name: UpdateDatabaseEngineConfig :exec
UPDATE service_database SET engine_config = ? WHERE service_id = ?;

-- name: UpdateDatabaseCredentials :exec
UPDATE service_database
SET db_name = COALESCE(?, db_name), db_user = COALESCE(?, db_user), db_password = COALESCE(?, db_password)
WHERE service_id = ?;

-- name: UpdateDatabaseExternalPort :exec
UPDATE service_database SET external_port = ? WHERE service_id = ?;

-- name: GetDatabaseByAppName :one
SELECT service_database.* FROM service_database
JOIN services ON services.id = service_database.service_id
WHERE services.app_name = ?;

-- name: ListDatabasesByEnvironment :many
SELECT service_database.* FROM service_database
JOIN services ON services.id = service_database.service_id
WHERE services.environment_id = ?;

-- name: ListDatabasesByServer :many
SELECT service_database.* FROM service_database
JOIN services ON services.id = service_database.service_id
WHERE services.server_id = ?;

-- Management helpers (unified across all engines).

-- name: DatabaseManagementCredentials :one
SELECT s.app_name, d.db_name, d.db_user, d.db_password, d.external_port, s.server_id
FROM service_database d
JOIN services s ON s.id = d.service_id
WHERE d.service_id = ?;

-- name: DatabaseManagementRotatePassword :exec
UPDATE service_database SET db_password = ? WHERE service_id = ?;

-- Deployment context for DATABASE services
-- (server_id, environment_id, project_id, memory_limit, cpu_limit).

-- name: DatabaseGetDeploymentContext :one
SELECT s.server_id AS server_id,
       s.environment_id AS environment_id,
       e.project_id AS project_id,
       s.memory_limit AS memory_limit,
       s.cpu_limit AS cpu_limit
FROM services s
JOIN environments e ON e.id = s.environment_id
WHERE s.id = ? AND s.service_type = 'DATABASE';

-- Joined database record for services that render database details.

-- name: DatabaseGetById :one
SELECT s.id AS id, s.service_type AS service_type, s.name AS name, s.app_name AS app_name,
       s.description AS description, s.icon AS icon, s.app_status AS app_status,
       s.last_error AS last_error, s.env_var AS env_var, s.command AS command, s.args AS args,
       s.refresh_token AS refresh_token, s.docker_image AS docker_image,
       s.memory_reservation AS memory_reservation, s.memory_limit AS memory_limit,
       s.cpu_reservation AS cpu_reservation, s.cpu_limit AS cpu_limit, s.replicas AS replicas,
       s.health_check_swarm AS health_check_swarm, s.restart_policy_swarm AS restart_policy_swarm,
       s.placement_swarm AS placement_swarm, s.update_config_swarm AS update_config_swarm,
       s.rollback_config_swarm AS rollback_config_swarm, s.mode_swarm AS mode_swarm,
       s.labels_swarm AS labels_swarm, s.network_swarm AS network_swarm,
       s.endpoint_spec_swarm AS endpoint_spec_swarm, s.ulimits_swarm AS ulimits_swarm,
       s.stop_grace_period_swarm AS stop_grace_period_swarm, s.network_ids AS network_ids,
       s.detach_rustploy_network AS detach_rustploy_network,
       s.environment_id AS environment_id, s.server_id AS server_id,
       s.created_at AS created_at, s.updated_at AS updated_at,
       d.db_type AS db_type, d.external_port AS external_port, d.db_name AS db_name,
       d.db_user AS db_user, d.db_password AS db_password, d.engine_config AS engine_config
FROM services s
JOIN service_database d ON d.service_id = s.id
WHERE s.id = ?;

-- Full rows (services.* + service_database.*) for one environment.

-- name: ListDatabasesByEnvironmentFull :many
SELECT s.id AS id, s.service_type AS service_type, s.name AS name, s.app_name AS app_name,
       s.description AS description, s.icon AS icon, s.app_status AS app_status,
       s.last_error AS last_error, s.env_var AS env_var, s.command AS command, s.args AS args,
       s.refresh_token AS refresh_token, s.docker_image AS docker_image,
       s.memory_reservation AS memory_reservation, s.memory_limit AS memory_limit,
       s.cpu_reservation AS cpu_reservation, s.cpu_limit AS cpu_limit, s.replicas AS replicas,
       s.network_ids AS network_ids, s.detach_rustploy_network AS detach_rustploy_network,
       s.environment_id AS environment_id, s.server_id AS server_id,
       s.created_at AS created_at, s.updated_at AS updated_at,
       d.db_type AS db_type, d.external_port AS external_port, d.db_name AS db_name,
       d.db_user AS db_user, d.db_password AS db_password, d.engine_config AS engine_config
FROM services s
JOIN service_database d ON d.service_id = s.id
WHERE s.environment_id = ?;

-- Partial patch; NULL keeps the stored value for every field.

-- name: ServicesPatchDatabase :exec
UPDATE services
SET memory_reservation = COALESCE(?, memory_reservation),
    memory_limit = COALESCE(?, memory_limit),
    cpu_reservation = COALESCE(?, cpu_reservation),
    cpu_limit = COALESCE(?, cpu_limit),
    replicas = COALESCE(?, replicas),
    env_var = COALESCE(?, env_var),
    command = COALESCE(?, command),
    args = COALESCE(?, args)
WHERE id = ?;

