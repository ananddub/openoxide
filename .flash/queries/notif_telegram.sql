-- Extracted from src/db/repository/notif_telegram.rs. Do not edit repository SQL directly.

-- name: NotifTelegramGetAll :many
SELECT id AS id, bot_token AS bot_token, chat_id AS chat_id, message_thread_id AS message_thread_id FROM notif_telegram;

-- name: NotifTelegramGetById :one
SELECT id AS id, bot_token AS bot_token, chat_id AS chat_id, message_thread_id AS message_thread_id FROM notif_telegram WHERE id = ?;

-- name: NotifTelegramCreate :exec
INSERT INTO notif_telegram (bot_token, chat_id, message_thread_id) VALUES (?, ?, ?);

-- name: NotifTelegramUpdate :exec
UPDATE notif_telegram SET bot_token = ?, chat_id = ?, message_thread_id = ? WHERE id = ?;

-- name: NotifTelegramDelete :exec
DELETE FROM notif_telegram WHERE id = ?;
