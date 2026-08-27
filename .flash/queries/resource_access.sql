-- Extracted from src/db/repository/resource_access.rs. Do not edit repository SQL directly.

-- name: ResourceAccessCheckAccess :one
SELECT COUNT(*) FROM resource_access
            WHERE user_id = ? AND org_id = ? AND resource_type = ? AND resource_id = ?;

-- name: ResourceAccessGrantAccess :one
INSERT INTO resource_access (user_id, org_id, resource_type, resource_id)
            VALUES (?, ?, ?, ?)
            RETURNING *;

-- name: ResourceAccessRevokeAccess :exec
DELETE FROM resource_access WHERE user_id = ? AND org_id = ? AND resource_type = ? AND resource_id = ?;
