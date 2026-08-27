-- Extracted from src/db/repository/audit_logs.rs. Do not edit repository SQL directly.

-- name: AuditLogsListFiltered :many
SELECT id, user_email, user_role, action, resource_type, resource_id, resource_name, metadata, organization_id, user_id, created_at FROM audit_logs WHERE organization_id = ? AND (? IS NULL OR action = ?) AND (? IS NULL OR resource_type = ?) ORDER BY created_at DESC, id DESC LIMIT ? OFFSET ?;

-- name: AuditLogsCountFiltered :one
SELECT COUNT(*) FROM audit_logs WHERE organization_id = ? AND (? IS NULL OR action = ?) AND (? IS NULL OR resource_type = ?);

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
