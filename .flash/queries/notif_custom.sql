-- Extracted from src/db/repository/notif_custom.rs. Do not edit repository SQL directly.

-- name: NotifCustomGetAll :many
SELECT id AS id, endpoint AS endpoint, headers AS headers FROM notif_custom;

-- name: NotifCustomGetById :one
SELECT id AS id, endpoint AS endpoint, headers AS headers FROM notif_custom WHERE id = ?;

-- name: NotifCustomCreate :exec
INSERT INTO notif_custom (endpoint, headers) VALUES (?, ?);

-- name: NotifCustomUpdate :exec
UPDATE notif_custom SET endpoint = ?, headers = ? WHERE id = ?;

-- name: NotifCustomDelete :exec
DELETE FROM notif_custom WHERE id = ?;
