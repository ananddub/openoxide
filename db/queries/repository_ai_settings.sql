-- Extracted from src/db/repository/ai_settings.rs. Do not edit repository SQL directly.

-- name: AiSettingsGetAll :many
SELECT id AS id, name AS name, api_url AS api_url, api_key AS api_key, model AS model, is_enabled AS is_enabled, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM ai_settings;

-- name: AiSettingsGetById :one
SELECT id AS id, name AS name, api_url AS api_url, api_key AS api_key, model AS model, is_enabled AS is_enabled, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM ai_settings WHERE id = ?;

-- name: AiSettingsListByOrganization :many
SELECT id AS id, name AS name, api_url AS api_url, api_key AS api_key, model AS model, is_enabled AS is_enabled, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at
               FROM ai_settings WHERE organization_id = ? ORDER BY created_at DESC, id DESC;

-- name: AiSettingsGetForOrganization :one
SELECT id AS id, name AS name, api_url AS api_url, api_key AS api_key, model AS model, is_enabled AS is_enabled, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at
               FROM ai_settings WHERE id = ? AND organization_id = ?;

-- name: AiSettingsCreate :exec
INSERT INTO ai_settings (name, api_url, api_key, model, is_enabled, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: AiSettingsUpdate :exec
UPDATE ai_settings SET name = ?, api_url = ?, api_key = ?, model = ?, is_enabled = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: AiSettingsDelete :exec
DELETE FROM ai_settings WHERE id = ?;

-- name: AiSettingsDeleteForOrganization :exec
DELETE FROM ai_settings WHERE id = ? AND organization_id = ?;
