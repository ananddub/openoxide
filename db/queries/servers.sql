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


-- name: ServersMigrateDependencies1 :exec
UPDATE applications SET server_id = ? WHERE server_id = ?;

-- name: ServersMigrateDependencies2 :exec
UPDATE applications SET build_server_id = ? WHERE build_server_id = ?;

-- name: ServersMigrateDependencies3 :exec
UPDATE compose_projects SET server_id = ? WHERE server_id = ?;

-- name: ServersMigrateDependencies4 :exec
UPDATE certificates SET server_id = ? WHERE server_id = ?;

-- name: ServersMigrateDependencies5 :exec
UPDATE schedules SET server_id = ? WHERE server_id = ?;

-- name: ServersMigratableResourceIds1 :many
SELECT id AS id FROM applications WHERE server_id = ? ORDER BY id;

-- name: ServersMigratableResourceIds2 :many
SELECT id AS id FROM compose_projects WHERE server_id = ? ORDER BY id;

-- name: ServersMigratableResourceIds3 :many
SELECT id AS id FROM applications WHERE build_server_id = ? ORDER BY id;

-- name: ServersMigratableResourceIds4 :many
SELECT id AS id FROM certificates WHERE server_id = ? ORDER BY id;

-- name: ServersMigratableResourceIds5 :many
SELECT id AS id FROM schedules WHERE server_id = ? ORDER BY id;

-- name: ServersRollbackMigratedResources1 :exec
UPDATE applications SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: ServersRollbackMigratedResources2 :exec
UPDATE compose_projects SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: ServersRollbackMigratedResources3 :exec
UPDATE applications SET build_server_id = ? WHERE id = ? AND build_server_id = ?;

-- name: ServersRollbackMigratedResources4 :exec
UPDATE certificates SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: ServersRollbackMigratedResources5 :exec
UPDATE schedules SET server_id = ? WHERE id = ? AND server_id = ?;

-- name: ServersDependencyCounts1 :one
SELECT COUNT(*) FROM applications WHERE server_id = ?;

-- name: ServersDependencyCounts2 :one
SELECT COUNT(*) FROM applications WHERE build_server_id = ?;

-- name: ServersDependencyCounts3 :one
SELECT COUNT(*) FROM compose_projects WHERE server_id = ?;

-- name: ServersDependencyCounts4 :one
SELECT COUNT(*) FROM postgres_dbs WHERE server_id = ?;

-- name: ServersDependencyCounts5 :one
SELECT COUNT(*) FROM mysql_dbs WHERE server_id = ?;

-- name: ServersDependencyCounts6 :one
SELECT COUNT(*) FROM mariadb_dbs WHERE server_id = ?;

-- name: ServersDependencyCounts7 :one
SELECT COUNT(*) FROM mongo_dbs WHERE server_id = ?;

-- name: ServersDependencyCounts8 :one
SELECT COUNT(*) FROM redis_dbs WHERE server_id = ?;

-- name: ServersDependencyCounts9 :one
SELECT COUNT(*) FROM libsql_dbs WHERE server_id = ?;

-- name: ServersDependencyCounts10 :one
SELECT COUNT(*) FROM certificates WHERE server_id = ?;

-- name: ServersDependencyCounts11 :one
SELECT COUNT(*) FROM schedules WHERE server_id = ?;

-- name: ServersGetAll :many
SELECT id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit FROM servers;

-- name: ServersGetById :one
SELECT id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit FROM servers WHERE id = ?;

-- name: ServersCreate :exec
INSERT INTO servers (name, description, ip_address, port, username, app_name, server_status, server_type, enable_docker_cleanup, log_cleanup_cron, command, metrics_config, ssh_key_id, created_at, updated_at, build_memory_limit, build_cpu_limit) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: ServersUpdate :exec
UPDATE servers SET name = ?, description = ?, ip_address = ?, port = ?, username = ?, app_name = ?, server_status = ?, server_type = ?, enable_docker_cleanup = ?, log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?, created_at = ?, updated_at = ?, build_memory_limit = ?, build_cpu_limit = ? WHERE id = ?;

-- name: ServersDelete :exec
DELETE FROM servers WHERE id = ?;

-- name: ServersGetSshCredentials :one
SELECT
                  s.ip_address AS public_ip,
                  n.private_host AS private_ip,
                  n.status AS private_status,
                  n.connection_mode AS connection_mode,
                  s.port,
                  s.username,
                  k.private_key,
                  k.public_key
               FROM servers s
               JOIN ssh_keys k ON k.id = s.ssh_key_id
               LEFT JOIN server_private_networks n ON n.server_id = s.id
               WHERE s.id = ?;

-- name: ServersGetDirectSshCredentials :one
SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id WHERE s.id = ?;

-- name: ServersListOrdered :many
SELECT id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit
               FROM servers ORDER BY created_at DESC, id DESC;

-- name: ServersCreateAndReturn :one
INSERT INTO servers
               (name, description, ip_address, port, username, app_name, server_type, ssh_key_id, build_memory_limit, build_cpu_limit)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit;

-- name: ServersUpdateAndReturn :one
UPDATE servers SET
               name = ?, description = ?, ip_address = ?, port = ?, username = ?,
               server_status = ?, server_type = ?, enable_docker_cleanup = ?,
               log_cleanup_cron = ?, command = ?, metrics_config = ?, ssh_key_id = ?,
               build_memory_limit = ?, build_cpu_limit = ?
               WHERE id = ?
               RETURNING id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit;

-- name: ServersSetStatus :one
UPDATE servers SET server_status = ? WHERE id = ?
               RETURNING id AS id, name AS name, description AS description, ip_address AS ip_address, port AS port, username AS username, app_name AS app_name, server_status AS server_status, server_type AS server_type, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, command AS command, metrics_config AS metrics_config, ssh_key_id AS ssh_key_id, created_at AS created_at, updated_at AS updated_at, build_memory_limit AS build_memory_limit, build_cpu_limit AS build_cpu_limit;
