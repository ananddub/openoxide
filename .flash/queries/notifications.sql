-- ============================================================
-- NOTIFICATION PROVIDERS
-- ============================================================

-- name: CreateNotifSlack :one
INSERT INTO notif_slack (webhook_url, channel) VALUES (?, ?) RETURNING *;

-- name: UpdateNotifSlack :exec
UPDATE notif_slack SET webhook_url = ?, channel = ? WHERE id = ?;

-- name: DeleteNotifSlack :exec
DELETE FROM notif_slack WHERE id = ?;

-- name: CreateNotifTelegram :one
INSERT INTO notif_telegram (bot_token, chat_id, message_thread_id) VALUES (?, ?, ?) RETURNING *;

-- name: UpdateNotifTelegram :exec
UPDATE notif_telegram SET bot_token = ?, chat_id = ?, message_thread_id = ? WHERE id = ?;

-- name: DeleteNotifTelegram :exec
DELETE FROM notif_telegram WHERE id = ?;

-- name: CreateNotifDiscord :one
INSERT INTO notif_discord (webhook_url, decoration) VALUES (?, ?) RETURNING *;

-- name: UpdateNotifDiscord :exec
UPDATE notif_discord SET webhook_url = ?, decoration = ? WHERE id = ?;

-- name: DeleteNotifDiscord :exec
DELETE FROM notif_discord WHERE id = ?;

-- name: CreateNotifEmail :one
INSERT INTO notif_email (smtp_server, smtp_port, username, password, from_address, to_addresses)
VALUES (?, ?, ?, ?, ?, ?) RETURNING *;

-- name: UpdateNotifEmail :exec
UPDATE notif_email
SET smtp_server = ?, smtp_port = ?, username = ?, password = ?,
    from_address = ?, to_addresses = ?
WHERE id = ?;

-- name: DeleteNotifEmail :exec
DELETE FROM notif_email WHERE id = ?;

-- name: CreateNotifResend :one
INSERT INTO notif_resend (api_key, from_address, to_addresses)
VALUES (?, ?, ?) RETURNING *;

-- name: UpdateNotifResend :exec
UPDATE notif_resend SET api_key = ?, from_address = ?, to_addresses = ? WHERE id = ?;

-- name: DeleteNotifResend :exec
DELETE FROM notif_resend WHERE id = ?;

-- name: CreateNotifGotify :one
INSERT INTO notif_gotify (server_url, app_token, priority, decoration)
VALUES (?, ?, ?, ?) RETURNING *;

-- name: UpdateNotifGotify :exec
UPDATE notif_gotify SET server_url = ?, app_token = ?, priority = ?, decoration = ? WHERE id = ?;

-- name: DeleteNotifGotify :exec
DELETE FROM notif_gotify WHERE id = ?;

-- name: CreateNotifNtfy :one
INSERT INTO notif_ntfy (server_url, topic, access_token, priority)
VALUES (?, ?, ?, ?) RETURNING *;

-- name: UpdateNotifNtfy :exec
UPDATE notif_ntfy SET server_url = ?, topic = ?, access_token = ?, priority = ? WHERE id = ?;

-- name: DeleteNotifNtfy :exec
DELETE FROM notif_ntfy WHERE id = ?;

-- name: CreateNotifMattermost :one
INSERT INTO notif_mattermost (webhook_url, channel, username) VALUES (?, ?, ?) RETURNING *;

-- name: UpdateNotifMattermost :exec
UPDATE notif_mattermost SET webhook_url = ?, channel = ?, username = ? WHERE id = ?;

-- name: DeleteNotifMattermost :exec
DELETE FROM notif_mattermost WHERE id = ?;

-- name: CreateNotifTeams :one
INSERT INTO notif_teams (webhook_url) VALUES (?) RETURNING *;

-- name: UpdateNotifTeams :exec
UPDATE notif_teams SET webhook_url = ? WHERE id = ?;

-- name: DeleteNotifTeams :exec
DELETE FROM notif_teams WHERE id = ?;

-- name: CreateNotifLark :one
INSERT INTO notif_lark (webhook_url) VALUES (?) RETURNING *;

-- name: UpdateNotifLark :exec
UPDATE notif_lark SET webhook_url = ? WHERE id = ?;

-- name: DeleteNotifLark :exec
DELETE FROM notif_lark WHERE id = ?;

-- name: CreateNotifPushover :one
INSERT INTO notif_pushover (user_key, api_token, priority, retry, expire)
VALUES (?, ?, ?, ?, ?) RETURNING *;

-- name: UpdateNotifPushover :exec
UPDATE notif_pushover
SET user_key = ?, api_token = ?, priority = ?, retry = ?, expire = ?
WHERE id = ?;

-- name: DeleteNotifPushover :exec
DELETE FROM notif_pushover WHERE id = ?;

-- name: CreateNotifCustom :one
INSERT INTO notif_custom (endpoint, headers) VALUES (?, ?) RETURNING *;

-- name: UpdateNotifCustom :exec
UPDATE notif_custom SET endpoint = ?, headers = ? WHERE id = ?;

-- name: DeleteNotifCustom :exec
DELETE FROM notif_custom WHERE id = ?;

-- ============================================================
-- NOTIFICATIONS (main)
-- ============================================================

-- name: GetNotificationByID :one
SELECT * FROM notifications WHERE id = ?;

-- name: ListNotificationsByOrg :many
SELECT * FROM notifications WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListNotificationsByType :many
SELECT * FROM notifications
WHERE notification_type = ? AND organization_id = ?
ORDER BY created_at DESC;

-- name: CreateNotification :one
INSERT INTO notifications (
    name, notification_type,
    on_app_deploy, on_app_build_error, on_database_backup,
    on_volume_backup, on_panel_restart, on_docker_cleanup, on_server_threshold,
    slack_id, telegram_id, discord_id, email_id, resend_id,
    gotify_id, ntfy_id, mattermost_id, custom_id,
    lark_id, pushover_id, teams_id,
    organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateNotificationTriggers :exec
UPDATE notifications
SET
    on_app_deploy        = ?,
    on_app_build_error   = ?,
    on_database_backup   = ?,
    on_volume_backup     = ?,
    on_panel_restart     = ?,
    on_docker_cleanup    = ?,
    on_server_threshold  = ?
WHERE id = ?;

-- name: DeleteNotification :exec
DELETE FROM notifications WHERE id = ?;


-- name: NotificationsGetAll :many
SELECT id AS id, name AS name, notification_type AS notification_type, on_app_deploy AS on_app_deploy, on_app_build_error AS on_app_build_error, on_database_backup AS on_database_backup, on_volume_backup AS on_volume_backup, on_panel_restart AS on_panel_restart, on_docker_cleanup AS on_docker_cleanup, on_server_threshold AS on_server_threshold, on_panel_backup AS on_panel_backup, on_schedule_success AS on_schedule_success, on_schedule_failure AS on_schedule_failure, slack_id AS slack_id, telegram_id AS telegram_id, discord_id AS discord_id, email_id AS email_id, resend_id AS resend_id, gotify_id AS gotify_id, ntfy_id AS ntfy_id, mattermost_id AS mattermost_id, custom_id AS custom_id, lark_id AS lark_id, pushover_id AS pushover_id, teams_id AS teams_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM notifications;

-- name: NotificationsGetByOrganization :many
SELECT id AS id, name AS name, notification_type AS notification_type, on_app_deploy AS on_app_deploy, on_app_build_error AS on_app_build_error, on_database_backup AS on_database_backup, on_volume_backup AS on_volume_backup, on_panel_restart AS on_panel_restart, on_docker_cleanup AS on_docker_cleanup, on_server_threshold AS on_server_threshold, on_panel_backup AS on_panel_backup, on_schedule_success AS on_schedule_success, on_schedule_failure AS on_schedule_failure, slack_id AS slack_id, telegram_id AS telegram_id, discord_id AS discord_id, email_id AS email_id, resend_id AS resend_id, gotify_id AS gotify_id, ntfy_id AS ntfy_id, mattermost_id AS mattermost_id, custom_id AS custom_id, lark_id AS lark_id, pushover_id AS pushover_id, teams_id AS teams_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM notifications WHERE organization_id = ?;

-- name: NotificationsGetById :one
SELECT id AS id, name AS name, notification_type AS notification_type, on_app_deploy AS on_app_deploy, on_app_build_error AS on_app_build_error, on_database_backup AS on_database_backup, on_volume_backup AS on_volume_backup, on_panel_restart AS on_panel_restart, on_docker_cleanup AS on_docker_cleanup, on_server_threshold AS on_server_threshold, on_panel_backup AS on_panel_backup, on_schedule_success AS on_schedule_success, on_schedule_failure AS on_schedule_failure, slack_id AS slack_id, telegram_id AS telegram_id, discord_id AS discord_id, email_id AS email_id, resend_id AS resend_id, gotify_id AS gotify_id, ntfy_id AS ntfy_id, mattermost_id AS mattermost_id, custom_id AS custom_id, lark_id AS lark_id, pushover_id AS pushover_id, teams_id AS teams_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM notifications WHERE id = ?;

-- name: NotificationsGetByIdForOrganization :one
SELECT id AS id, name AS name, notification_type AS notification_type, on_app_deploy AS on_app_deploy, on_app_build_error AS on_app_build_error, on_database_backup AS on_database_backup, on_volume_backup AS on_volume_backup, on_panel_restart AS on_panel_restart, on_docker_cleanup AS on_docker_cleanup, on_server_threshold AS on_server_threshold, on_panel_backup AS on_panel_backup, on_schedule_success AS on_schedule_success, on_schedule_failure AS on_schedule_failure, slack_id AS slack_id, telegram_id AS telegram_id, discord_id AS discord_id, email_id AS email_id, resend_id AS resend_id, gotify_id AS gotify_id, ntfy_id AS ntfy_id, mattermost_id AS mattermost_id, custom_id AS custom_id, lark_id AS lark_id, pushover_id AS pushover_id, teams_id AS teams_id, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM notifications WHERE id = ? AND organization_id = ?;

-- name: NotificationsCreate :exec
INSERT INTO notifications (name, notification_type, on_app_deploy, on_app_build_error, on_database_backup, on_volume_backup, on_panel_restart, on_docker_cleanup, on_server_threshold, on_panel_backup, on_schedule_success, on_schedule_failure, slack_id, telegram_id, discord_id, email_id, resend_id, gotify_id, ntfy_id, mattermost_id, custom_id, lark_id, pushover_id, teams_id, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: NotificationsUpdate :exec
UPDATE notifications SET name = ?, notification_type = ?, on_app_deploy = ?, on_app_build_error = ?, on_database_backup = ?, on_volume_backup = ?, on_panel_restart = ?, on_docker_cleanup = ?, on_server_threshold = ?, on_panel_backup = ?, on_schedule_success = ?, on_schedule_failure = ?, slack_id = ?, telegram_id = ?, discord_id = ?, email_id = ?, resend_id = ?, gotify_id = ?, ntfy_id = ?, mattermost_id = ?, custom_id = ?, lark_id = ?, pushover_id = ?, teams_id = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: NotificationsDelete :exec
DELETE FROM notifications WHERE id = ?;

-- name: NotificationsDeleteForOrganization :exec
DELETE FROM notifications WHERE id = ? AND organization_id = ?;
