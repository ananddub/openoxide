-- Extracted from src/db/repository/notif_slack.rs. Do not edit repository SQL directly.

-- name: NotifSlackGetAll :many
SELECT id AS id, webhook_url AS webhook_url, channel AS channel FROM notif_slack;

-- name: NotifSlackGetById :one
SELECT id AS id, webhook_url AS webhook_url, channel AS channel FROM notif_slack WHERE id = ?;

-- name: NotifSlackCreate :exec
INSERT INTO notif_slack (webhook_url, channel) VALUES (?, ?);

-- name: NotifSlackUpdate :exec
UPDATE notif_slack SET webhook_url = ?, channel = ? WHERE id = ?;

-- name: NotifSlackDelete :exec
DELETE FROM notif_slack WHERE id = ?;
