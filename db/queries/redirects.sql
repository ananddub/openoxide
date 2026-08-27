-- Extracted from src/db/repository/redirects.rs. Do not edit repository SQL directly.

-- name: RedirectsGetAll :many
SELECT id AS id, regex AS regex, replacement AS replacement, permanent AS permanent, unique_config_key AS unique_config_key, application_id AS application_id, created_at AS created_at, updated_at AS updated_at FROM redirects;

-- name: RedirectsGetById :one
SELECT id AS id, regex AS regex, replacement AS replacement, permanent AS permanent, unique_config_key AS unique_config_key, application_id AS application_id, created_at AS created_at, updated_at AS updated_at FROM redirects WHERE id = ?;

-- name: RedirectsListByApplication :many
SELECT id AS id, regex AS regex, replacement AS replacement,
                      permanent AS permanent, unique_config_key AS unique_config_key,
                      application_id AS application_id, created_at AS created_at,
                      updated_at AS updated_at
               FROM redirects WHERE application_id = ? ORDER BY id;

-- name: RedirectsCreateForApplication :exec
INSERT INTO redirects (regex, replacement, permanent, unique_config_key, application_id) VALUES (?, ?, ?, ?, ?);

-- name: RedirectsUpdateForApplication :exec
UPDATE redirects SET regex = ?, replacement = ?, permanent = ?, unique_config_key = ? WHERE id = ? AND application_id = ?;

-- name: RedirectsDeleteForApplication :exec
DELETE FROM redirects WHERE id = ? AND application_id = ?;

-- name: RedirectsCreate :exec
INSERT INTO redirects (regex, replacement, permanent, unique_config_key, application_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?);

-- name: RedirectsUpdate :exec
UPDATE redirects SET regex = ?, replacement = ?, permanent = ?, unique_config_key = ?, application_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: RedirectsDelete :exec
DELETE FROM redirects WHERE id = ?;
