-- Extracted from src/db/repository/security.rs. Do not edit repository SQL directly.

-- name: SecurityGetAll :many
SELECT id AS id, username AS username, password AS password, service_id AS service_id, created_at AS created_at FROM security;

-- name: SecurityGetById :one
SELECT id AS id, username AS username, password AS password, service_id AS service_id, created_at AS created_at FROM security WHERE id = ?;

-- name: SecurityListByApplication :many
SELECT id AS id, username AS username, password AS password, service_id AS service_id, created_at AS created_at FROM security WHERE service_id = ? ORDER BY id;

-- name: SecurityCreateForApplication :one
INSERT INTO security (username, password, service_id) VALUES (?, ?, ?)
               RETURNING id, username, password, service_id, created_at;

-- name: SecurityUpdateForApplication :one
UPDATE security SET username = ?, password = ? WHERE id = ? AND service_id = ?
               RETURNING id, username, password, service_id, created_at;

-- name: SecurityDeleteForApplication :one
DELETE FROM security WHERE id = ? AND service_id = ? RETURNING id;

-- name: SecurityCreate :exec
INSERT INTO security (username, password, service_id, created_at) VALUES (?, ?, ?, ?);

-- name: SecurityUpdate :exec
UPDATE security SET username = ?, password = ?, service_id = ?, created_at = ? WHERE id = ?;

-- name: SecurityDelete :exec
DELETE FROM security WHERE id = ?;
