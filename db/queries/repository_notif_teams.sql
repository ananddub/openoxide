-- Extracted from src/db/repository/notif_teams.rs. Do not edit repository SQL directly.

-- name: NotifTeamsGetAll :many
SELECT id AS id, webhook_url AS webhook_url FROM notif_teams;

-- name: NotifTeamsGetById :one
SELECT id AS id, webhook_url AS webhook_url FROM notif_teams WHERE id = ?;

-- name: NotifTeamsCreate :exec
INSERT INTO notif_teams (webhook_url) VALUES (?);

-- name: NotifTeamsUpdate :exec
UPDATE notif_teams SET webhook_url = ? WHERE id = ?;

-- name: NotifTeamsDelete :exec
DELETE FROM notif_teams WHERE id = ?;
