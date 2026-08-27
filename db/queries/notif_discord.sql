-- Extracted from src/db/repository/notif_discord.rs. Do not edit repository SQL directly.

-- name: NotifDiscordGetAll :many
SELECT id AS id, webhook_url AS webhook_url, decoration AS decoration FROM notif_discord;

-- name: NotifDiscordGetById :one
SELECT id AS id, webhook_url AS webhook_url, decoration AS decoration FROM notif_discord WHERE id = ?;

-- name: NotifDiscordCreate :exec
INSERT INTO notif_discord (webhook_url, decoration) VALUES (?, ?);

-- name: NotifDiscordUpdate :exec
UPDATE notif_discord SET webhook_url = ?, decoration = ? WHERE id = ?;

-- name: NotifDiscordDelete :exec
DELETE FROM notif_discord WHERE id = ?;
