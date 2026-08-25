-- Extracted from src/db/repository/notif_resend.rs. Do not edit repository SQL directly.

-- name: NotifResendGetAll :many
SELECT id AS id, api_key AS api_key, from_address AS from_address, to_addresses AS to_addresses FROM notif_resend;

-- name: NotifResendGetById :one
SELECT id AS id, api_key AS api_key, from_address AS from_address, to_addresses AS to_addresses FROM notif_resend WHERE id = ?;

-- name: NotifResendCreate :exec
INSERT INTO notif_resend (api_key, from_address, to_addresses) VALUES (?, ?, ?);

-- name: NotifResendUpdate :exec
UPDATE notif_resend SET api_key = ?, from_address = ?, to_addresses = ? WHERE id = ?;

-- name: NotifResendDelete :exec
DELETE FROM notif_resend WHERE id = ?;

-- name: NotifResendFindForUser :one
SELECT e.id AS id, e.api_key AS api_key, e.from_address AS from_address, e.to_addresses AS to_addresses
               FROM notif_resend e
               JOIN notifications n ON n.resend_id = e.id
               JOIN organization o ON o.id = n.organization_id
               LEFT JOIN organization_members m ON m.organization_id = o.id AND m.user_id = ?
               WHERE o.owner_id = ? OR m.user_id IS NOT NULL
               ORDER BY n.id LIMIT 1;
