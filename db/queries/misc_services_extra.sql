-- Extra queries for the misc service migration (AI service, etc.).
-- GLOBALLY UNIQUE names, prefixed MiscSvc.

-- INSERT ... RETURNING the new id so callers never rely on
-- last_insert_rowid() across pooled connections.
-- name: MiscSvcAiSettingsCreateReturnId :one
INSERT INTO ai_settings (name, api_url, api_key, model, is_enabled, organization_id, created_at, updated_at)
VALUES (?, ?, ?, ?, ?, ?, ?, ?)
RETURNING id AS id;

-- Rows-affected-as-bool delete via RETURNING.
-- name: MiscSvcAiSettingsDeleteForOrganization :one
DELETE FROM ai_settings WHERE id = ? AND organization_id = ? RETURNING id AS id;
