-- Extracted from src/db/repository/notif_mattermost.rs. Do not edit repository SQL directly.

-- name: NotifMattermostGetAll :many
SELECT id AS id, webhook_url AS webhook_url, channel AS channel, username AS username FROM notif_mattermost;

-- name: NotifMattermostGetById :one
SELECT id AS id, webhook_url AS webhook_url, channel AS channel, username AS username FROM notif_mattermost WHERE id = ?;

-- name: NotifMattermostCreate :exec
INSERT INTO notif_mattermost (webhook_url, channel, username) VALUES (?, ?, ?);

-- name: NotifMattermostUpdate :exec
UPDATE notif_mattermost SET webhook_url = ?, channel = ?, username = ? WHERE id = ?;

-- name: NotifMattermostDelete :exec
DELETE FROM notif_mattermost WHERE id = ?;
