-- ============================================================
-- REDIRECTS
-- ============================================================

-- name: GetRedirectByID :one
SELECT * FROM redirects WHERE id = ?;

-- name: ListRedirectsByApplication :many
SELECT * FROM redirects WHERE application_id = ? ORDER BY created_at DESC;

-- name: CreateRedirect :one
INSERT INTO redirects (regex, replacement, permanent, application_id)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: UpdateRedirect :one
UPDATE redirects
SET regex = ?, replacement = ?, permanent = ? = ?
WHERE id = ?
RETURNING *;

-- name: DeleteRedirect :exec
DELETE FROM redirects WHERE id = ?;

-- name: DeleteRedirectsByApplication :exec
DELETE FROM redirects WHERE application_id = ?;

-- ============================================================
-- PORTS
-- ============================================================

-- name: GetPortByID :one
SELECT * FROM ports WHERE id = ?;

-- name: ListPortsByApplication :many
SELECT * FROM ports WHERE application_id = ? ORDER BY published_port ASC;

-- name: CreatePort :one
INSERT INTO ports (published_port, target_port, protocol, publish_mode, application_id)
VALUES (?, ?, ?, ?, ?)
RETURNING *;

-- name: DeletePort :exec
DELETE FROM ports WHERE id = ?;

-- name: DeletePortsByApplication :exec
DELETE FROM ports WHERE application_id = ?;

-- ============================================================
-- SECURITY (Basic Auth)
-- ============================================================

-- name: GetSecurityByID :one
SELECT * FROM security WHERE id = ?;

-- name: ListSecurityByApplication :many
SELECT * FROM security WHERE application_id = ? ORDER BY created_at DESC;

-- name: CreateSecurity :one
INSERT INTO security (username, password, application_id)
VALUES (?, ?, ?)
RETURNING *;

-- name: UpdateSecurity :exec
UPDATE security SET password = ? WHERE id = ?;

-- name: DeleteSecurity :exec
DELETE FROM security WHERE id = ?;

-- name: DeleteSecurityByApplication :exec
DELETE FROM security WHERE application_id = ?;
