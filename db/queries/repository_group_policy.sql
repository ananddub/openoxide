-- Extracted from src/db/repository/group_policy.rs. Do not edit repository SQL directly.

-- name: GroupPolicyGetAll :many
SELECT id AS id, group_id AS group_id, policy_id AS policy_id, created_at AS created_at FROM group_policy;

-- name: GroupPolicyGetById :one
SELECT id AS id, group_id AS group_id, policy_id AS policy_id, created_at AS created_at FROM group_policy WHERE id = ?;

-- name: GroupPolicyCreate :exec
INSERT INTO group_policy (group_id, policy_id, created_at) VALUES (?, ?, ?);

-- name: GroupPolicyUpdate :exec
UPDATE group_policy SET group_id = ?, policy_id = ?, created_at = ? WHERE id = ?;

-- name: GroupPolicyDelete :exec
DELETE FROM group_policy WHERE id = ?;
