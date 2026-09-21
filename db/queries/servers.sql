-- ============================================================
-- SERVERS
-- ============================================================

-- name: GetServerByID :one
SELECT * FROM servers WHERE id = ?;

-- name: GetServerByAppName :one
SELECT * FROM servers WHERE app_name = ?;

-- name: ListServers :many
SELECT * FROM servers ORDER BY created_at DESC;

-- name: ListServersByStatus :many
SELECT * FROM servers WHERE server_status = ? ORDER BY created_at DESC;

-- name: ListServersByType :many
SELECT * FROM servers WHERE server_type = ? ORDER BY created_at DESC;

-- name: CreateServer :one
INSERT INTO servers (
    name, description, ip_address, port, username,
    app_name, server_status, server_type,
    enable_docker_cleanup, log_cleanup_cron,
    command, metrics_config, ssh_key_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateServer :one
UPDATE servers
SET
    name              = ?,
    description       = ?,
    ip_address        = ?,
    port              = ?,
    username          = ?,
    server_status     = ?,
    server_type       = ?,
    enable_docker_cleanup = ?,
    log_cleanup_cron  = ?,
    command           = ?,
    metrics_config    = ?,
    ssh_key_id        = ?
WHERE id = ?
RETURNING *;

-- name: UpdateServerStatus :exec
UPDATE servers SET server_status = ? WHERE id = ?;

-- name: UpdateServerMetricsConfig :exec
UPDATE servers SET metrics_config = ? WHERE id = ?;

-- name: DeleteServer :exec
DELETE FROM servers WHERE id = ?;


-- name: ServersMigrateApplicationServers :exec
UPDATE services SET server_id = ? WHERE server_id = ? AND service_type = 'APPLICATION';

-- name: ServersMigrateBuildAssignments :exec
UPDATE service_application SET build_server_id = ? WHERE build_server_id = ?;

-- name: ServersMigrateComposeServers :exec
UPDATE services SET server_id = ? WHERE server_id = ? AND service_type = 'COMPOSE';

-- name: ServersMigrateCertificates :exec
UPDATE certificates SET server_id = ? WHERE server_id = ?;

-- name: ServersMigrateSchedules :exec
UPDATE schedules SET server_id = ? WHERE server_id = ?;

-- Selective (per-service) repoint variants: flip only the ids in the JSON
-- array, and validate each row still belongs to the source server in the same
-- statement (guards against a row that moved after the id list was captured).
-- name: ServersMigrateApplicationServersByIds :exec
UPDATE services SET server_id = ? WHERE server_id = ? AND service_type = 'APPLICATION' AND id IN (SELECT CAST(value AS INTEGER) FROM json_each(?));

-- name: ServersMigrateBuildAssignmentsByIds :exec
UPDATE service_application SET build_server_id = ? WHERE build_server_id = ? AND service_id IN (SELECT CAST(value AS INTEGER) FROM json_each(?));

-- name: ServersMigrateComposeServersByIds :exec
UPDATE services SET server_id = ? WHERE server_id = ? AND service_type = 'COMPOSE' AND id IN (SELECT CAST(value AS INTEGER) FROM json_each(?));

-- name: ServersMigrateCertificatesByIds :exec
UPDATE certificates SET server_id = ? WHERE server_id = ? AND id IN (SELECT CAST(value AS INTEGER) FROM json_each(?));

-- name: ServersMigrateSchedulesByIds :exec
UPDATE schedules SET server_id = ? WHERE server_id = ? AND id IN (SELECT CAST(value AS INTEGER) FROM json_each(?));

-- name: ServersMigratableApplicationIds :many
SELECT id AS id FROM services WHERE server_id = ? AND service_type = 'APPLICATION' ORDER BY id;

-- name: ServersMigratableComposeIds :many
SELECT id AS id FROM services WHERE server_id = ? AND service_type = 'COMPOSE' ORDER BY id;

-- name: ServersMigratableBuildServiceIds :many
SELECT service_id AS id FROM service_application WHERE build_server_id = ? ORDER BY service_id;

-- name: ServersMigratableCertificateIds :many
SELECT id AS id FROM certificates WHERE server_id = ? ORDER BY id;

-- name: ServersMigratableScheduleIds :many
SELECT id AS id FROM schedules WHERE server_id = ? ORDER BY id;

-- name: ServersRollbackApplicationServers :exec
UPDATE services SET server_id = ? WHERE id = ? AND server_id = ? AND service_type = 'APPLICATION';

-- name: ServersRollbackComposeServers :exec
UPDATE services SET server_id = ? WHERE id = ? AND server_id = ? AND service_type = 'COMPOSE';

-- name: ServersRollbackBuildAssignments :exec
UPDATE service_application SET build_server_id = ? WHERE service_id = ? AND build_server_id = ?;

-- name: ServersRollbackCertificates :exec
UPDATE certificates SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: ServersRollbackSchedules :exec
UPDATE schedules SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: ServersCountApplications :one
SELECT COUNT(*) FROM services WHERE server_id = ? AND service_type = 'APPLICATION';

-- name: ServersCountBuildAssignments :one
SELECT COUNT(*) FROM service_application WHERE build_server_id = ?;

-- name: ServersCountComposeProjects :one
SELECT COUNT(*) FROM services WHERE server_id = ? AND service_type = 'COMPOSE';

-- name: ServersCountDatabases :one
SELECT COUNT(*) FROM services WHERE server_id = ? AND service_type = 'DATABASE';

-- name: ServersCountCertificates :one
SELECT COUNT(*) FROM certificates WHERE server_id = ?;

-- name: ServersCountSchedules :one
SELECT COUNT(*) FROM schedules WHERE server_id = ?;

-- name: ServersListMigratedApplications :many
SELECT id AS id FROM services WHERE service_type = 'APPLICATION' AND id IN (SELECT CAST(value AS INTEGER) FROM json_each(?)) ORDER BY id;

-- name: ServersListMigratedCompose :many
SELECT id AS id FROM services WHERE service_type = 'COMPOSE' AND id IN (SELECT CAST(value AS INTEGER) FROM json_each(?)) ORDER BY id;

-- name: ServersGetAll :many
SELECT id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit, host_key_fingerprint AS host_key_fingerprint, host_key_openssh AS host_key_openssh FROM servers;

-- name: ServersGetById :one
SELECT id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit, host_key_fingerprint AS host_key_fingerprint, host_key_openssh AS host_key_openssh FROM servers WHERE id = ?;

-- name: ServersCreate :exec
INSERT INTO servers (name, description, ip_address, port, username, app_name, server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command, metrics_config, ssh_key_id, created_at, updated_at, build_memory_limit, build_cpu_limit) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: ServersUpdate :exec
UPDATE servers SET name = ?, description = ?, ip_address = ?, port = ?, username = ?, app_name = ?, server_status = ?, server_type = ?, enable_docker_cleanup = ?, log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?, created_at = ?, updated_at = ?, build_memory_limit = ?, build_cpu_limit = ? WHERE id = ?;

-- name: ServersDelete :exec
DELETE FROM servers WHERE id = ? AND organization_id = ?;

-- name: ServersGetSshCredentials :one
SELECT
                  s.ip_address AS public_ip,
                  n.private_host AS private_ip,
                  COALESCE(n.status, '') AS private_status,
                  n.connection_mode AS connection_mode,
                  s.port,
                  s.username,
                  s.host_key_fingerprint AS host_key_fingerprint,
                  s.host_key_openssh AS host_key_openssh,
                  k.private_key,
                  k.public_key
               FROM servers s
               JOIN ssh_keys k ON k.id = s.ssh_key_id
               LEFT JOIN server_private_networks n ON n.server_id = s.id
               WHERE s.id = ?;

-- name: ServersGetDirectSshCredentials :one
SELECT s.ip_address, s.port, s.username, s.host_key_fingerprint AS host_key_fingerprint, s.host_key_openssh AS host_key_openssh, k.private_key, k.public_key FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id WHERE s.id = ?;

-- name: ServersListOrdered :many
SELECT id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit, host_key_fingerprint AS host_key_fingerprint, host_key_openssh AS host_key_openssh
               FROM servers WHERE organization_id = ? ORDER BY created_at DESC, id DESC;

-- name: ServersCreateAndReturn :one
INSERT INTO servers
               (name, description, ip_address, port, username, app_name, server_status, server_type, enable_docker_cleanup, command, metrics_config, ssh_key_id, build_memory_limit, build_cpu_limit, organization_id)
               VALUES (?, ?, ?, ?, ?, ?, 'INACTIVE', ?, 0, '', '{}', ?, ?, ?, ?)
               RETURNING id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit, host_key_fingerprint AS host_key_fingerprint, host_key_openssh AS host_key_openssh;

-- name: ServersUpdateAndReturn :one
UPDATE servers SET
               name = ?, description = ?, ip_address = ?, port = ?, username = ?,
               server_status = ?, server_type = ?, enable_docker_cleanup = ?,
               log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?,
               build_memory_limit = ?, build_cpu_limit = ?
               WHERE id = ? AND organization_id = ?
               RETURNING id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit, host_key_fingerprint AS host_key_fingerprint, host_key_openssh AS host_key_openssh;

-- name: ServersSetStatus :one
UPDATE servers SET server_status = ? WHERE id = ?
               RETURNING id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit, host_key_fingerprint AS host_key_fingerprint, host_key_openssh AS host_key_openssh;

-- name: ServersSetHostKeyFingerprint :exec
UPDATE servers SET host_key_fingerprint = ? WHERE id = ?;

-- name: ServersGetHostKeyFingerprint :one
SELECT host_key_fingerprint FROM servers WHERE id = ?;

-- name: ServersSetHostKey :exec
UPDATE servers SET host_key_fingerprint = ?, host_key_openssh = ? WHERE id = ?;

-- name: ServersGetHostKey :one
SELECT host_key_fingerprint, host_key_openssh FROM servers WHERE id = ?;
