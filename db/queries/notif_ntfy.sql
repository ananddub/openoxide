-- Extracted from src/db/repository/notif_ntfy.rs. Do not edit repository SQL directly.

-- name: NotifNtfyGetAll :many
SELECT id AS id, server_url AS server_url, topic AS topic, access_token AS access_token, priority AS priority FROM notif_ntfy;

-- name: NotifNtfyGetById :one
SELECT id AS id, server_url AS server_url, topic AS topic, access_token AS access_token, priority AS priority FROM notif_ntfy WHERE id = ?;

-- name: NotifNtfyCreate :exec
INSERT INTO notif_ntfy (server_url, topic, access_token, priority) VALUES (?, ?, ?, ?);

-- name: NotifNtfyUpdate :exec
UPDATE notif_ntfy SET server_url = ?, topic = ?, access_token = ?, priority = ? WHERE id = ?;

-- name: NotifNtfyDelete :exec
DELETE FROM notif_ntfy WHERE id = ?;
