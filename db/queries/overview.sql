-- Overview service queries (unified services schema).
-- Reproduces the old inline overview SQL from src/services/overview.rs:
-- applications / compose_projects / postgres_dbs / mysql_dbs / mariadb_dbs /
-- mongo_dbs / redis_dbs are merged into services (+ service_database).
-- LIBSQL databases stay hidden, matching the old overview which never
-- queried the old libsql table.

-- name: OverviewListServices :many
SELECT
    s.id AS id,
    s.name AS name,
    s.service_type AS service_type,
    s.app_status AS status,
    s.created_at AS created_at,
    p.id AS project_id,
    p.name AS project_name,
    e.id AS environment_id,
    e.name AS environment_name,
    d.db_type AS db_kind,
    s.server_id AS server_id,
    sv.name AS server_name
FROM services s
JOIN environments e ON s.environment_id = e.id
JOIN projects p ON e.project_id = p.id
LEFT JOIN servers sv ON s.server_id = sv.id
LEFT JOIN service_database d ON d.service_id = s.id
WHERE p.organization_id = ?
  AND NOT (s.service_type = 'DATABASE' AND COALESCE(d.db_type, '') = 'LIBSQL')
ORDER BY
    CASE s.service_type
        WHEN 'APPLICATION' THEN 0
        WHEN 'COMPOSE' THEN 1
        ELSE 2
    END,
    CASE d.db_type
        WHEN 'POSTGRES' THEN 0
        WHEN 'MYSQL' THEN 1
        WHEN 'MARIADB' THEN 2
        WHEN 'MONGO' THEN 3
        WHEN 'REDIS' THEN 4
        ELSE 5
    END,
    s.created_at DESC;

-- name: OverviewListDomains :many
SELECT
    dom.id AS id,
    dom.host AS host,
    dom.path AS path,
    dom.port AS port,
    dom.https AS https,
    dom.service_id AS service_id,
    s.service_type AS service_type,
    dom.internal_path AS internal_path,
    dom.custom_entrypoint AS custom_entrypoint,
    dom.custom_cert_resolver AS custom_cert_resolver,
    dom.strip_path AS strip_path,
    dom.middlewares AS middlewares,
    dom.domain_type AS domain_type,
    dom.certificate_type AS certificate_type,
    dom.service_name AS domain_service_name,
    COALESCE(s.name, dom.service_name, 'Service') AS service_name,
    COALESCE(p.name, 'Project') AS project_name
FROM domains dom
LEFT JOIN services s ON dom.service_id = s.id
LEFT JOIN environments e ON s.environment_id = e.id
LEFT JOIN projects p ON e.project_id = p.id
WHERE p.organization_id = ?
ORDER BY dom.created_at DESC;

-- name: OverviewListBackups :many
SELECT
    b.id AS id,
    b.app_name AS name,
    b.enabled AS enabled,
    b.destination_id AS destination_id,
    b.service_id AS service_id,
    s.service_type AS service_type,
    d.db_type AS db_type,
    b.created_at AS created_at
FROM backups b
LEFT JOIN services s ON b.service_id = s.id
LEFT JOIN service_database d ON d.service_id = b.service_id
WHERE b.organization_id = ?;

-- name: OverviewListVolumeBackups :many
SELECT
    vb.id AS id,
    vb.name AS name,
    vb.enabled AS enabled,
    vb.destination_id AS destination_id,
    vb.service_id AS service_id,
    vb.service_type AS service_type,
    vb.created_at AS created_at
FROM volume_backups vb
WHERE vb.organization_id = ?;
