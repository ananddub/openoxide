-- Extracted from src/db/repository/notif_pushover.rs. Do not edit repository SQL directly.

-- name: NotifPushoverGetAll :many
SELECT id AS id, user_key AS user_key, api_token AS api_token, priority AS priority, retry AS retry, expire AS expire FROM notif_pushover;

-- name: NotifPushoverGetById :one
SELECT id AS id, user_key AS user_key, api_token AS api_token, priority AS priority, retry AS retry, expire AS expire FROM notif_pushover WHERE id = ?;

-- name: NotifPushoverCreate :exec
INSERT INTO notif_pushover (user_key, api_token, priority, retry, expire) VALUES (?, ?, ?, ?, ?);

-- name: NotifPushoverUpdate :exec
UPDATE notif_pushover SET user_key = ?, api_token = ?, priority = ?, retry = ?, expire = ? WHERE id = ?;

-- name: NotifPushoverDelete :exec
DELETE FROM notif_pushover WHERE id = ?;
