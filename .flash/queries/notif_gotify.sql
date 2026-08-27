-- Extracted from src/db/repository/notif_gotify.rs. Do not edit repository SQL directly.

-- name: NotifGotifyGetAll :many
SELECT id AS id, server_url AS server_url, app_token AS app_token, priority AS priority, decoration AS decoration FROM notif_gotify;

-- name: NotifGotifyGetById :one
SELECT id AS id, server_url AS server_url, app_token AS app_token, priority AS priority, decoration AS decoration FROM notif_gotify WHERE id = ?;

-- name: NotifGotifyCreate :exec
INSERT INTO notif_gotify (server_url, app_token, priority, decoration) VALUES (?, ?, ?, ?);

-- name: NotifGotifyUpdate :exec
UPDATE notif_gotify SET server_url = ?, app_token = ?, priority = ?, decoration = ? WHERE id = ?;

-- name: NotifGotifyDelete :exec
DELETE FROM notif_gotify WHERE id = ?;
