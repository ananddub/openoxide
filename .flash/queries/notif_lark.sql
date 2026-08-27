-- Extracted from src/db/repository/notif_lark.rs. Do not edit repository SQL directly.

-- name: NotifLarkGetAll :many
SELECT id AS id, webhook_url AS webhook_url FROM notif_lark;

-- name: NotifLarkGetById :one
SELECT id AS id, webhook_url AS webhook_url FROM notif_lark WHERE id = ?;

-- name: NotifLarkCreate :exec
INSERT INTO notif_lark (webhook_url) VALUES (?);

-- name: NotifLarkUpdate :exec
UPDATE notif_lark SET webhook_url = ? WHERE id = ?;

-- name: NotifLarkDelete :exec
DELETE FROM notif_lark WHERE id = ?;
