-- Extracted from src/db/repository/domains.rs. Do not edit repository SQL directly.

-- name: DomainsRouteInUse :one
SELECT COUNT(*) AS count
               FROM domains
               WHERE host = ? AND path = ?;

-- name: DomainsRouteInUseExcluding :one
SELECT COUNT(*) AS count
               FROM domains
               WHERE host = ? AND path = ? AND id != ?;

-- name: DomainsHostInUse :one
SELECT COUNT(*) AS count FROM domains WHERE lower(host) = lower(?);

-- name: DomainsGetAll :many
SELECT id AS id, host AS host, https AS https, port AS port, path AS path, internal_path AS internal_path, custom_entrypoint AS custom_entrypoint, service_name AS service_name, custom_cert_resolver AS custom_cert_resolver, strip_path AS strip_path, middlewares AS middlewares, domain_type AS domain_type, certificate_type AS certificate_type, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM domains;

-- name: DomainsGetById :one
SELECT id AS id, host AS host, https AS https, port AS port, path AS path, internal_path AS internal_path, custom_entrypoint AS custom_entrypoint, service_name AS service_name, custom_cert_resolver AS custom_cert_resolver, strip_path AS strip_path, middlewares AS middlewares, domain_type AS domain_type, certificate_type AS certificate_type, service_id AS service_id, created_at AS created_at, updated_at AS updated_at FROM domains WHERE id = ?;

-- name: DomainsCreate :exec
INSERT INTO domains (host, https, port, path, internal_path, custom_entrypoint, service_name, custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type, service_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: DomainsUpdate :exec
UPDATE domains SET host = ?, https = ?, port = ?, path = ?, internal_path = ?, custom_entrypoint = ?, service_name = ?, custom_cert_resolver = ?, strip_path = ?, middlewares = ?, domain_type = ?, certificate_type = ?, service_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: DomainsDelete :exec
DELETE FROM domains WHERE id = ?;

-- name: DomainsListByApplication :many
SELECT id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at
               FROM domains
               WHERE service_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: DomainsListByCompose :many
SELECT id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at
               FROM domains
               WHERE service_id = ?
               ORDER BY created_at DESC, id DESC;

-- name: DomainsListAll :many
SELECT id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at
               FROM domains
               ORDER BY created_at DESC, id DESC;

-- name: DomainsListByApplicationRaw :many
SELECT id AS id, host AS host, https AS https, port AS port, path AS path, internal_path AS internal_path, custom_entrypoint AS custom_entrypoint, service_name AS service_name, custom_cert_resolver AS custom_cert_resolver, strip_path AS strip_path, middlewares AS middlewares, domain_type AS domain_type, certificate_type AS certificate_type, service_id AS service_id, created_at AS created_at, updated_at AS updated_at
               FROM domains
               WHERE service_id = ?
               ORDER BY id;

-- name: DomainsListByComposeRaw :many
SELECT id AS id, host AS host, https AS https, port AS port, path AS path, internal_path AS internal_path, custom_entrypoint AS custom_entrypoint, service_name AS service_name, custom_cert_resolver AS custom_cert_resolver, strip_path AS strip_path, middlewares AS middlewares, domain_type AS domain_type, certificate_type AS certificate_type, service_id AS service_id, created_at AS created_at, updated_at AS updated_at
               FROM domains
               WHERE service_id = ?
               ORDER BY id;

-- name: DomainsCreateAndReturn :one
INSERT INTO domains
               (host, https, port, path, internal_path, custom_entrypoint, service_name,
                custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type,
                service_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at;

-- name: DomainsUpdateAndReturn :one
UPDATE domains SET
               host = ?, https = ?, port = ?, path = ?, internal_path = ?,
               custom_entrypoint = ?, service_name = ?, custom_cert_resolver = ?,
               strip_path = ?, middlewares = ?, certificate_type = ?
               WHERE id = ?
               RETURNING id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at;

-- name: DomainsGetRecordById :one
SELECT id AS id, host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, service_id,
               created_at, updated_at
               FROM domains
               WHERE id = ?;
