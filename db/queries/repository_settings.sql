-- Extracted from src/db/repository/settings.rs. Do not edit repository SQL directly.

-- name: SettingsGetAll :many
SELECT id AS id, server_ip AS server_ip, certificate_type AS certificate_type, custom_cert_resolver AS custom_cert_resolver, https AS https, host AS host, lets_encrypt_email AS lets_encrypt_email, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, metrics_config AS metrics_config, created_at AS created_at, updated_at AS updated_at FROM settings;

-- name: SettingsGetById :one
SELECT id AS id, server_ip AS server_ip, certificate_type AS certificate_type, custom_cert_resolver AS custom_cert_resolver, https AS https, host AS host, lets_encrypt_email AS lets_encrypt_email, enable_docker_cleanup AS enable_docker_cleanup, log_cleanup_cron AS log_cleanup_cron, metrics_config AS metrics_config, created_at AS created_at, updated_at AS updated_at FROM settings WHERE id = ?;

-- name: SettingsCreate :exec
INSERT INTO settings (server_ip, certificate_type, custom_cert_resolver, https, host, lets_encrypt_email, enable_docker_cleanup, log_cleanup_cron, metrics_config, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: SettingsUpdate :exec
UPDATE settings SET server_ip = ?, certificate_type = ?, custom_cert_resolver = ?, https = ?, host = ?, lets_encrypt_email = ?, enable_docker_cleanup = ?, log_cleanup_cron = ?, metrics_config = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: SettingsDelete :exec
DELETE FROM settings WHERE id = ?;
