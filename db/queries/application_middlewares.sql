-- Extracted from src/db/repository/application_middlewares.rs. Do not edit repository SQL directly.

-- name: ApplicationMiddlewaresListByApplication :many
SELECT id AS id, service_id AS service_id, name AS name, middleware_type AS middleware_type, enabled AS enabled, config AS config, created_at AS created_at, updated_at AS updated_at FROM application_middlewares WHERE service_id = ? ORDER BY name, id;

-- name: ApplicationMiddlewaresGetForApplication :one
SELECT id AS id, service_id AS service_id, name AS name, middleware_type AS middleware_type, enabled AS enabled, config AS config, created_at AS created_at, updated_at AS updated_at FROM application_middlewares WHERE id = ? AND service_id = ?;

-- name: ApplicationMiddlewaresCreate :one
INSERT INTO application_middlewares (service_id, name, middleware_type, enabled, config) VALUES (?, ?, ?, ?, ?)
               RETURNING id, service_id, name, middleware_type, enabled, config, created_at, updated_at;

-- name: ApplicationMiddlewaresUpdate :one
UPDATE application_middlewares SET name = ?, middleware_type = ?, enabled = ?, config = ? WHERE id = ? AND service_id = ?
               RETURNING id, service_id, name, middleware_type, enabled, config, created_at, updated_at;

-- name: ApplicationMiddlewaresDelete :one
DELETE FROM application_middlewares WHERE id = ? AND service_id = ? RETURNING id;
