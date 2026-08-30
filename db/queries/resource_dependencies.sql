-- Cross-resource dependency counts used by delete guards.
-- Unified schema: all resource tables are keyed by service_id.

-- name: ServiceDependencyCounts :one
SELECT
    (SELECT COUNT(*) FROM deployments WHERE service_id = ? AND status IN ('QUEUED', 'RUNNING')) AS active_deployments,
    (SELECT COUNT(*) FROM deployments WHERE service_id = ?) AS deployments,
    (SELECT COUNT(*) FROM domains WHERE service_id = ?) AS domains,
    (SELECT COUNT(*) FROM mounts WHERE service_id = ?) AS mounts,
    (SELECT COUNT(*) FROM patches WHERE service_id = ?) AS patches,
    (SELECT COUNT(*) FROM schedules WHERE service_id = ?) AS schedules,
    (SELECT COUNT(*) FROM volume_backups WHERE service_id = ?) AS backups,
    (SELECT COUNT(*) FROM volume_backups WHERE service_id = ? AND enabled = 1) AS enabled_backups,
    (SELECT COUNT(*) FROM preview_deployments WHERE base_service_id = ? OR preview_service_id = ?) AS preview_deployments,
    (SELECT COUNT(*) FROM application_middlewares WHERE service_id = ?) AS middleware,
    (SELECT COUNT(*) FROM ports WHERE service_id = ?) AS ports,
    (SELECT COUNT(*) FROM redirects WHERE service_id = ?) AS redirects,
    (SELECT COUNT(*) FROM security WHERE service_id = ?) AS security_rules;

-- name: CertificateDependencyCounts :one
SELECT COUNT(*) AS renewals,
       COUNT(*) FILTER (WHERE status = 'RUNNING') AS running_renewals
FROM certificate_renewals WHERE certificate_id = ?;

-- Network usage across service classes (network id as JSON string value).

-- name: NetworkDependencyApplications :one
SELECT COUNT(*) AS count
FROM services CROSS JOIN json_each(services.network_ids)
WHERE services.service_type = 'APPLICATION' AND json_each.value = ?;

-- name: NetworkDependencyDatabases :one
SELECT COUNT(*) AS count
FROM services CROSS JOIN json_each(services.network_ids)
WHERE services.service_type = 'DATABASE' AND json_each.value = ?;

-- name: NetworkDependencyComposeServices :one
SELECT COUNT(DISTINCT sc.service_id) AS count
FROM service_compose sc CROSS JOIN json_tree(sc.service_networks)
WHERE json_tree.value = ?;
