-- Extracted from src/db/repository/application_middlewares.rs. Do not edit repository SQL directly.

-- name: ApplicationMiddlewaresListByApplication :many
SELECT id AS id, application_id AS application_id, name AS name, middleware_type AS middleware_type, enabled AS enabled, config AS config, created_at AS created_at, updated_at AS updated_at FROM application_middlewares WHERE application_id = ? ORDER BY name, id;

-- name: ApplicationMiddlewaresGetForApplication :one
SELECT id AS id, application_id AS application_id, name AS name, middleware_type AS middleware_type, enabled AS enabled, config AS config, created_at AS created_at, updated_at AS updated_at FROM application_middlewares WHERE id = ? AND application_id = ?;

-- name: ApplicationMiddlewaresCreate :exec
INSERT INTO application_middlewares (application_id, name, middleware_type, enabled, config) VALUES (?, ?, ?, ?, ?);

-- name: ApplicationMiddlewaresUpdate :exec
UPDATE application_middlewares SET name = ?, middleware_type = ?, enabled = ?, config = ? WHERE id = ? AND application_id = ?;

-- name: ApplicationMiddlewaresDelete :exec
DELETE FROM application_middlewares WHERE id = ? AND application_id = ?;
