-- ============================================================
-- SETTINGS (Web Server)
-- ============================================================

-- name: GetSettings :one
SELECT * FROM settings LIMIT 1;

-- name: UpsertSettings :one
INSERT INTO settings (
    server_ip, certificate_type, custom_cert_resolver,
    https, host, lets_encrypt_email,
    enable_docker_cleanup, log_cleanup_cron, metrics_config
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(id) DO UPDATE SET
    server_ip             = excluded.server_ip,
    certificate_type      = excluded.certificate_type,
    custom_cert_resolver  = excluded.custom_cert_resolver,
    https                 = excluded.https,
    host                  = excluded.host,
    lets_encrypt_email    = excluded.lets_encrypt_email,
    enable_docker_cleanup = excluded.enable_docker_cleanup,
    log_cleanup_cron      = excluded.log_cleanup_cron,
    metrics_config        = excluded.metrics_config
RETURNING *;

-- name: UpdateSettings :one
UPDATE settings
SET
    server_ip             = ?,
    certificate_type      = ?,
    custom_cert_resolver  = ?,
    https                 = ?,
    host                  = ?,
    lets_encrypt_email    = ?,
    enable_docker_cleanup = ?,
    log_cleanup_cron      = ?,
    metrics_config        = ?
WHERE id = ?
RETURNING *;

-- ============================================================
-- AI SETTINGS
-- ============================================================

-- name: GetAiSettingByID :one
SELECT * FROM ai_settings WHERE id = ?;

-- name: ListAiSettingsByOrg :many
SELECT * FROM ai_settings WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListEnabledAiSettings :many
SELECT * FROM ai_settings WHERE organization_id = ? AND is_enabled = 1 ORDER BY created_at DESC;

-- name: CreateAiSetting :one
INSERT INTO ai_settings (name, api_url, api_key, model, is_enabled, organization_id)
VALUES (?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateAiSetting :one
UPDATE ai_settings
SET name = ?, api_url = ?, api_key = ?, model = ?, is_enabled = ?
WHERE id = ?
RETURNING *;

-- name: ToggleAiSetting :exec
UPDATE ai_settings SET is_enabled = ? WHERE id = ?;

-- name: DeleteAiSetting :exec
DELETE FROM ai_settings WHERE id = ?;


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
