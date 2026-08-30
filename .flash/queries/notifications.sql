-- Unified notifications (vault-style: notification_type + config_json).
-- SLACK:    {"webhook_url":..., "channel":...}
-- TELEGRAM: {"bot_token":..., "chat_id":..., "message_thread_id":...}
-- DISCORD:  {"webhook_url":..., "decoration":...}
-- EMAIL:    {"smtp_server":..., "smtp_port":..., "username":..., "password":..., "from_address":..., "to_addresses":...}
-- RESEND:   {"api_key":..., "from_address":..., "to_addresses":...}
-- GOTIFY:   {"server_url":..., "app_token":..., "priority":..., "decoration":...}
-- NTFY:     {"server_url":..., "topic":..., "access_token":..., "priority":...}
-- MATTERMOST: {"webhook_url":..., "channel":..., "username":...}
-- TEAMS:    {"webhook_url":...}
-- LARK:     {"webhook_url":...}
-- PUSHOVER: {"api_token":..., "user_key":..., "priority":...}
-- CUSTOM:   {"url":..., "method":..., "headers":..., "payload_template":...}

-- name: GetNotificationByID :one
SELECT * FROM notifications WHERE id = ?;

-- First notification of the given type reachable by a user: owned org or
-- membership. Caller parses config_json for channel-specific credentials.

-- name: NotificationsFindForUser :one
SELECT n.id AS id,
       n.config_json AS config_json
FROM notifications n
JOIN organization o ON o.id = n.organization_id
LEFT JOIN organization_members m ON m.organization_id = o.id AND m.user_id = ?
WHERE n.notification_type = ? AND (o.owner_id = ? OR m.user_id IS NOT NULL)
ORDER BY n.id
LIMIT 1;

-- name: GetNotificationByIDForOrganization :one
SELECT * FROM notifications WHERE id = ? AND organization_id = ?;

-- name: ListNotifications :many
SELECT * FROM notifications ORDER BY created_at DESC;

-- name: ListNotificationsByType :many
SELECT * FROM notifications WHERE notification_type = ? ORDER BY created_at DESC;

-- name: ListNotificationsByOrganization :many
SELECT * FROM notifications WHERE organization_id = ? ORDER BY created_at DESC;

-- name: CreateNotification :one
INSERT INTO notifications (name, notification_type, config_json, organization_id)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: CreateNotificationWithTriggers :one
INSERT INTO notifications (
    name, notification_type, config_json,
    on_app_deploy, on_app_build_error, on_database_backup, on_volume_backup,
    on_panel_backup, on_panel_restart, on_docker_cleanup, on_server_threshold,
    on_server_migration, on_security_alert, on_schedule_success, on_schedule_failure,
    organization_id, created_at, updated_at
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateNotification :one
UPDATE notifications SET name = ?, notification_type = ?, config_json = ? WHERE id = ?
RETURNING *;

-- name: UpdateNotificationConfig :exec
UPDATE notifications SET config_json = ? WHERE id = ?;

-- name: UpdateNotificationTriggers :exec
UPDATE notifications
SET on_app_deploy = ?, on_app_build_error = ?, on_database_backup = ?, on_volume_backup = ?,
    on_panel_backup = ?, on_panel_restart = ?, on_docker_cleanup = ?, on_server_threshold = ?,
    on_server_migration = ?, on_security_alert = ?, on_schedule_success = ?, on_schedule_failure = ?
WHERE id = ?;

-- name: UpdateNotificationFull :one
UPDATE notifications
SET name = ?, notification_type = ?, config_json = ?,
    on_app_deploy = ?, on_app_build_error = ?, on_database_backup = ?, on_volume_backup = ?,
    on_panel_backup = ?, on_panel_restart = ?, on_docker_cleanup = ?, on_server_threshold = ?,
    on_server_migration = ?, on_security_alert = ?, on_schedule_success = ?, on_schedule_failure = ?,
    organization_id = ?, created_at = ?, updated_at = ?
WHERE id = ?
RETURNING *;

-- name: DeleteNotification :exec
DELETE FROM notifications WHERE id = ?;

-- name: DeleteNotificationForOrganization :one
DELETE FROM notifications WHERE id = ? AND organization_id = ?
RETURNING id;

-- Trigger-based routing queries (send notifications when the flag is set).

-- name: ListNotificationsForAppDeploy :many
SELECT * FROM notifications WHERE on_app_deploy = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForAppBuildError :many
SELECT * FROM notifications WHERE on_app_build_error = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForDatabaseBackup :many
SELECT * FROM notifications WHERE on_database_backup = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForVolumeBackup :many
SELECT * FROM notifications WHERE on_volume_backup = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForPanelBackup :many
SELECT * FROM notifications WHERE on_panel_backup = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForPanelRestart :many
SELECT * FROM notifications WHERE on_panel_restart = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForDockerCleanup :many
SELECT * FROM notifications WHERE on_docker_cleanup = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForServerThreshold :many
SELECT * FROM notifications WHERE on_server_threshold = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForServerMigration :many
SELECT * FROM notifications WHERE on_server_migration = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForSecurityAlert :many
SELECT * FROM notifications WHERE on_security_alert = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForScheduleSuccess :many
SELECT * FROM notifications WHERE on_schedule_success = 1 ORDER BY created_at DESC;

-- name: ListNotificationsForScheduleFailure :many
SELECT * FROM notifications WHERE on_schedule_failure = 1 ORDER BY created_at DESC;
