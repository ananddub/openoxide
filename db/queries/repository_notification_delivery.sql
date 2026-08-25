-- Extracted from src/db/repository/notification_delivery.rs. Do not edit repository SQL directly.

-- name: NotificationDeliveryBegin :exec
INSERT INTO notification_delivery_attempts (notification_id, organization_id, trigger_name, correlation_id, status, attempt, title, body) VALUES (?, ?, ?, ?, 'PENDING', ?, ?, ?);

-- name: NotificationDeliveryFinish :exec
UPDATE notification_delivery_attempts SET status=?, error=?, finished_at=unixepoch() WHERE id=?;

-- name: NotificationDeliveryList :many
SELECT id AS id, notification_id AS notification_id, organization_id AS organization_id, trigger_name AS trigger_name, correlation_id AS correlation_id, status AS status, attempt AS attempt, title AS title, body AS body, error, created_at AS created_at, finished_at FROM notification_delivery_attempts WHERE organization_id=? ORDER BY created_at DESC LIMIT ?;

-- name: NotificationDeliveryCreateBinding :exec
INSERT INTO notification_resource_bindings (notification_id, organization_id, resource_type, resource_id) VALUES (?, ?, ?, ?);

-- name: NotificationDeliveryBindings :many
SELECT id AS id, notification_id AS notification_id, organization_id AS organization_id, resource_type AS resource_type, resource_id AS resource_id, created_at AS created_at FROM notification_resource_bindings WHERE organization_id=? ORDER BY created_at DESC;

-- name: NotificationDeliveryDeleteBinding :exec
DELETE FROM notification_resource_bindings WHERE id=? AND organization_id=?;
