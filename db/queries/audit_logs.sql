-- Audit log queries. Extracted from src/db/repository/audit_logs.rs.

-- name: AuditLogsListByOrganization :many
SELECT id, user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at FROM audit_logs WHERE organization_id = ? ORDER BY created_at DESC, id DESC LIMIT ? OFFSET ?;

-- name: AuditLogsListByOrganizationAndAction :many
SELECT id, user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at FROM audit_logs WHERE organization_id = ? AND action = ? ORDER BY created_at DESC, id DESC LIMIT ? OFFSET ?;

-- name: AuditLogsCountByOrganization :one
SELECT COUNT(*) FROM audit_logs WHERE organization_id = ?;

-- name: AuditLogsCountByOrganizationAndAction :one
SELECT COUNT(*) FROM audit_logs WHERE organization_id = ? AND action = ?;

-- name: AuditLogsGetAll :many
SELECT id AS id, user_email AS user_email, user_role AS user_role, action AS action, resource_type AS resource_type, resource_id AS resource_id, resource_name AS resource_name, metadata AS metadata, organization_id AS organization_id, user_id AS user_id, created_at AS created_at FROM audit_logs;

-- name: AuditLogsGetById :one
SELECT id AS id, user_email AS user_email, user_role AS user_role, action AS action, resource_type AS resource_type, resource_id AS resource_id, resource_name AS resource_name, metadata AS metadata, organization_id AS organization_id, user_id AS user_id, created_at AS created_at FROM audit_logs WHERE id = ?;

-- name: AuditLogsCreate :exec
INSERT INTO audit_logs (user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: AuditLogsUpdate :exec
UPDATE audit_logs SET user_email = ?, user_role = ?, action = ?, resource_type = ?, resource_id = ?, resource_name = ?, metadata = ?, organization_id = ?, user_id = ?, created_at = ? WHERE id = ?;

-- name: AuditLogsDelete :exec
DELETE FROM audit_logs WHERE id = ?;

-- name: AuditLogsDeleteOlderThan :exec
DELETE FROM audit_logs WHERE created_at < ?;

-- name: AuditLogsListByUser :many
SELECT id, user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at FROM audit_logs WHERE user_id = ? ORDER BY created_at DESC LIMIT ?;

-- name: AuditLogsListByResource :many
SELECT id, user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at FROM audit_logs WHERE resource_type = ? AND resource_id = ? ORDER BY created_at DESC;
