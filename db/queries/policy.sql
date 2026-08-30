-- Extracted from src/db/repository/policy.rs. Do not edit repository SQL directly.

-- name: PolicyGetAll :many
SELECT id AS id, action AS action, created_at AS created_at FROM policy;

-- name: PolicyGetById :one
SELECT id AS id, action AS action, created_at AS created_at FROM policy WHERE id = ?;

-- name: PolicyCreate :exec
INSERT INTO policy (action, created_at) VALUES (?, ?);

-- name: PolicyUpdate :exec
UPDATE policy SET action = ?, created_at = ? WHERE id = ?;

-- name: PolicyDelete :exec
DELETE FROM policy WHERE id = ?;
