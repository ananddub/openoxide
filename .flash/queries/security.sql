-- Extracted from src/db/repository/security.rs. Do not edit repository SQL directly.

-- name: SecurityGetAll :many
SELECT id AS id, username AS username, password AS password, application_id AS application_id, created_at AS created_at FROM security;

-- name: SecurityGetById :one
SELECT id AS id, username AS username, password AS password, application_id AS application_id, created_at AS created_at FROM security WHERE id = ?;

-- name: SecurityListByApplication :many
SELECT id AS id, username AS username, password AS password, application_id AS application_id, created_at AS created_at FROM security WHERE application_id = ? ORDER BY id;

-- name: SecurityCreateForApplication :exec
INSERT INTO security (username, password, application_id) VALUES (?, ?, ?);

-- name: SecurityUpdateForApplication :exec
UPDATE security SET username = ?, password = ? WHERE id = ? AND application_id = ?;

-- name: SecurityDeleteForApplication :exec
DELETE FROM security WHERE id = ? AND application_id = ?;

-- name: SecurityCreate :exec
INSERT INTO security (username, password, application_id, created_at) VALUES (?, ?, ?, ?);

-- name: SecurityUpdate :exec
UPDATE security SET username = ?, password = ?, application_id = ?, created_at = ? WHERE id = ?;

-- name: SecurityDelete :exec
DELETE FROM security WHERE id = ?;
