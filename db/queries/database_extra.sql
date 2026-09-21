-- Database-service migration extras.
-- NULL-precise creates and per-column updates that the shared
-- services/service_database queries cannot express (FlashORM binds are
-- non-nullable, so SQL NULL must come from literals or split variants).

-- name: DbCreateDatabaseServiceGlobal :one
INSERT INTO services (
    service_type, name, app_name, description, icon, app_status, last_error,
    env_var, command, args, refresh_token, docker_image,
    memory_reservation, memory_limit, cpu_reservation, cpu_limit,
    replicas, network_ids, detach_rustploy_network, environment_id, server_id,
    created_at, updated_at
) VALUES (
    ?, ?, ?, ?, NULL, 'IDLE', NULL,
    ?, ?, ?, NULL, ?,
    NULL, NULL, NULL, NULL,
    ?, ?, ?, ?, NULL,
    ?, ?
)
RETURNING id;

-- name: DbCreateDatabaseServiceForServer :one
INSERT INTO services (
    service_type, name, app_name, description, icon, app_status, last_error,
    env_var, command, args, refresh_token, docker_image,
    memory_reservation, memory_limit, cpu_reservation, cpu_limit,
    replicas, network_ids, detach_rustploy_network, environment_id, server_id,
    created_at, updated_at
) VALUES (
    ?, ?, ?, ?, NULL, 'IDLE', NULL,
    ?, ?, ?, NULL, ?,
    NULL, NULL, NULL, NULL,
    ?, ?, ?, ?, ?,
    ?, ?
)
RETURNING id;

-- name: DbCreateServiceDatabaseNoPort :one
INSERT INTO service_database (service_id, db_type, db_name, db_user, db_password, engine_config)
VALUES (?, ?, ?, ?, ?, ?)
RETURNING service_id;

-- name: DbCreateQueuedDatabaseDeploymentGlobal :one
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, database_kind, server_id, is_preview_deployment, user_id, last_state_at)
VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, NULL, 0, ?, unixepoch())
RETURNING id;

-- name: DbCreateQueuedDatabaseDeploymentForServer :one
INSERT INTO deployments (title, description, status, state, log_path, operation, service_id, database_kind, server_id, is_preview_deployment, user_id, last_state_at)
VALUES (?, ?, 'QUEUED', 'QUEUE', ?, ?, ?, ?, ?, 0, ?, unixepoch())
RETURNING id;

-- Per-column patch updates (old repositories used COALESCE per column;
-- callers invoke these only for provided fields so stored values — including
-- NULLs — are preserved exactly).

-- name: DbUpdateServiceMemoryReservation :exec
UPDATE services SET memory_reservation = ? WHERE id = ?;

-- name: DbUpdateServiceMemoryLimit :exec
UPDATE services SET memory_limit = ? WHERE id = ?;

-- name: DbUpdateServiceCpuReservation :exec
UPDATE services SET cpu_reservation = ? WHERE id = ?;

-- name: DbUpdateServiceCpuLimit :exec
UPDATE services SET cpu_limit = ? WHERE id = ?;

-- name: DbUpdateServiceReplicas :exec
UPDATE services SET replicas = ? WHERE id = ?;

-- name: DbUpdateDatabaseName :exec
UPDATE service_database SET db_name = ? WHERE service_id = ?;

-- name: DbUpdateDatabaseUser :exec
UPDATE service_database SET db_user = ? WHERE service_id = ?;

-- name: DbUpdateDatabasePassword :exec
UPDATE service_database SET db_password = ? WHERE service_id = ?;

-- Database-network usage counts. The old repository bound the network id as
-- TEXT against json_each values (a JSON array of id strings); CAST keeps the
-- same TEXT comparison here (the shared network_dependency_* queries bind a
-- float, which never matches the stored strings).

-- name: DbNetworkDependencyApplications :one
SELECT COUNT(*) AS count FROM services CROSS JOIN json_each(services.network_ids)
WHERE services.service_type = 'APPLICATION' AND CAST(json_each.value AS TEXT) = ?;

-- name: DbNetworkDependencyComposeServices :one
SELECT COUNT(DISTINCT sc.service_id) AS count
FROM service_compose sc CROSS JOIN json_tree(sc.service_networks)
WHERE CAST(json_tree.value AS TEXT) = ?;

-- name: DbNetworkDependencyDatabases :one
SELECT COUNT(*) AS count FROM services CROSS JOIN json_each(services.network_ids)
WHERE services.service_type = 'DATABASE' AND CAST(json_each.value AS TEXT) = ?;
