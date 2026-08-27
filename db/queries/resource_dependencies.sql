-- Extracted from src/db/repository/resource_dependencies.rs. Do not edit repository SQL directly.

-- name: ResourceDependenciesApplication :one
SELECT
                (SELECT COUNT(*) FROM deployments WHERE application_id = ? AND status IN ('QUEUED', 'RUNNING')) AS active_deployments,
                (SELECT COUNT(*) FROM deployments WHERE application_id = ?) AS deployments,
                (SELECT COUNT(*) FROM domains WHERE application_id = ?) AS domains,
                (SELECT COUNT(*) FROM mounts WHERE application_id = ?) AS mounts,
                (SELECT COUNT(*) FROM patches WHERE application_id = ?) AS patches,
                (SELECT COUNT(*) FROM schedules WHERE application_id = ?) AS schedules,
                (SELECT COUNT(*) FROM volume_backups WHERE application_id = ?) AS backups,
                (SELECT COUNT(*) FROM volume_backups WHERE application_id = ? AND enabled = 1) AS enabled_backups,
                (SELECT COUNT(*) FROM preview_deployments WHERE base_application_id = ? OR preview_application_id = ?) AS preview_deployments,
                (SELECT COUNT(*) FROM application_middlewares WHERE application_id = ?) AS middleware,
                (SELECT COUNT(*) FROM ports WHERE application_id = ?) AS ports,
                (SELECT COUNT(*) FROM redirects WHERE application_id = ?) AS redirects,
                (SELECT COUNT(*) FROM security WHERE application_id = ?) AS security_rules;

-- name: ResourceDependenciesCompose :one
SELECT
                (SELECT COUNT(*) FROM deployments WHERE compose_id = ? AND status IN ('QUEUED', 'RUNNING')) AS active_deployments,
                (SELECT COUNT(*) FROM deployments WHERE compose_id = ?) AS deployments,
                (SELECT COUNT(*) FROM domains WHERE compose_id = ?) AS domains,
                (SELECT COUNT(*) FROM mounts WHERE compose_id = ?) AS mounts,
                (SELECT COUNT(*) FROM patches WHERE compose_id = ?) AS patches,
                (SELECT COUNT(*) FROM schedules WHERE compose_id = ?) AS schedules,
                ((SELECT COUNT(*) FROM backups WHERE compose_id = ?) + (SELECT COUNT(*) FROM volume_backups WHERE compose_id = ?)) AS backups,
                ((SELECT COUNT(*) FROM backups WHERE compose_id = ? AND enabled = 1) + (SELECT COUNT(*) FROM volume_backups WHERE compose_id = ? AND enabled = 1)) AS enabled_backups;

-- name: ResourceDependenciesCertificate :one
SELECT COUNT(*) AS renewals,
                COUNT(*) FILTER (WHERE status = 'RUNNING') AS running_renewals
               FROM certificate_renewals WHERE certificate_id = ?;

-- name: ResourceDependenciesDatabase1 :one
SELECT
                (SELECT COUNT(*) FROM deployments WHERE database_id = ? AND database_kind = ? AND status IN ('QUEUED', 'RUNNING')) AS active_deployments,
                (SELECT COUNT(*) FROM deployments WHERE database_id = ? AND database_kind = ?) AS deployments,
                (SELECT COUNT(*) FROM backups WHERE
                    (? = 'postgres' AND postgres_id = ?) OR (? = 'mysql' AND mysql_id = ?) OR
                    (? = 'mariadb' AND mariadb_id = ?) OR (? = 'mongo' AND mongo_id = ?) OR
                    (? = 'redis' AND redis_id = ?) OR (? = 'libsql' AND libsql_id = ?)) AS backups,
                (SELECT COUNT(*) FROM backups WHERE enabled = 1 AND (
                    (? = 'postgres' AND postgres_id = ?) OR (? = 'mysql' AND mysql_id = ?) OR
                    (? = 'mariadb' AND mariadb_id = ?) OR (? = 'mongo' AND mongo_id = ?) OR
                    (? = 'redis' AND redis_id = ?) OR (? = 'libsql' AND libsql_id = ?))) AS enabled_backups;

-- name: ResourceDependenciesDatabase2 :one
SELECT COUNT(*) AS backups,
                COUNT(*) FILTER (WHERE enabled = 1) AS enabled_backups
               FROM volume_backups WHERE
                (? = 'postgres' AND postgres_id = ?) OR (? = 'mysql' AND mysql_id = ?) OR
                (? = 'mariadb' AND mariadb_id = ?) OR (? = 'mongo' AND mongo_id = ?) OR
                (? = 'redis' AND redis_id = ?) OR (? = 'libsql' AND libsql_id = ?);

-- name: ResourceDependenciesDatabaseNetwork1 :one
SELECT COUNT(*) AS count FROM applications, json_each(applications.network_ids) WHERE json_each.value = ?;

-- name: ResourceDependenciesDatabaseNetwork2 :one
SELECT COUNT(DISTINCT compose_projects.id) AS count
               FROM compose_projects, json_tree(compose_projects.service_networks)
               WHERE json_tree.value = ?;

-- name: ResourceDependenciesDatabaseNetwork3 :one
SELECT
                (SELECT COUNT(*) FROM postgres_dbs, json_each(postgres_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM mysql_dbs, json_each(mysql_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM mariadb_dbs, json_each(mariadb_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM mongo_dbs, json_each(mongo_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM redis_dbs, json_each(redis_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM libsql_dbs, json_each(libsql_dbs.network_ids) WHERE json_each.value = ?) AS count;
