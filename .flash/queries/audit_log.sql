-- ============================================================
-- AUDIT LOGS
-- ============================================================

-- name: CreateAuditLog :one
INSERT INTO audit_logs (
    user_email, user_role, action, resource_type,
    resource_id, resource_name, metadata,
    organization_id, user_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: ListAuditLogsByOrg :many
SELECT * FROM audit_logs
WHERE organization_id = ?
ORDER BY created_at DESC
LIMIT ?;

-- name: ListAuditLogsByUser :many
SELECT * FROM audit_logs
WHERE user_id = ?
ORDER BY created_at DESC
LIMIT ?;

-- name: ListAuditLogsByResource :many
SELECT * FROM audit_logs
WHERE resource_type = ? AND resource_id = ?
ORDER BY created_at DESC;

-- name: DeleteOldAuditLogs :exec
DELETE FROM audit_logs
WHERE created_at < ? AND organization_id = ?;
