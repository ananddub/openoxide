-- Extracted from src/db/repository/user_policy.rs. Do not edit repository SQL directly.

-- name: UserPolicyListByUserAndOrg :many
SELECT * FROM user_policy WHERE user_id = ? AND org_id = ?;

-- name: UserPolicyUpsert :one
INSERT INTO user_policy (user_id, org_id, policy_id, effect)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(user_id, org_id, policy_id) DO UPDATE SET effect = excluded.effect
            RETURNING *;

-- name: UserPolicyDelete :exec
DELETE FROM user_policy WHERE user_id = ? AND org_id = ? AND policy_id = ?;
