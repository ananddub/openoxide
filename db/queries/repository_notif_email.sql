-- Extracted from src/db/repository/notif_email.rs. Do not edit repository SQL directly.

-- name: NotifEmailGetAll :many
SELECT id AS id, smtp_server AS smtp_server, smtp_port AS smtp_port, username AS username, password AS password, from_address AS from_address, to_addresses AS to_addresses FROM notif_email;

-- name: NotifEmailGetById :one
SELECT id AS id, smtp_server AS smtp_server, smtp_port AS smtp_port, username AS username, password AS password, from_address AS from_address, to_addresses AS to_addresses FROM notif_email WHERE id = ?;

-- name: NotifEmailCreate :exec
INSERT INTO notif_email (smtp_server, smtp_port, username, password, from_address, to_addresses) VALUES (?, ?, ?, ?, ?, ?);

-- name: NotifEmailUpdate :exec
UPDATE notif_email SET smtp_server = ?, smtp_port = ?, username = ?, password = ?, from_address = ?, to_addresses = ? WHERE id = ?;

-- name: NotifEmailDelete :exec
DELETE FROM notif_email WHERE id = ?;

-- name: NotifEmailFindForUser :one
SELECT e.id AS id, e.smtp_server AS smtp_server, e.smtp_port AS smtp_port, e.username AS username, e.password AS password, e.from_address AS from_address, e.to_addresses AS to_addresses
               FROM notif_email e
               JOIN notifications n ON n.email_id = e.id
               JOIN organization o ON o.id = n.organization_id
               LEFT JOIN organization_members m ON m.organization_id = o.id AND m.user_id = ?
               WHERE o.owner_id = ? OR m.user_id IS NOT NULL
               ORDER BY n.id LIMIT 1;
