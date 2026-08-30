-- Extracted from src/db/repository/environments.rs. Do not edit repository SQL directly.

-- name: EnvironmentsGetAll :many
SELECT id AS id, name AS name, description AS description, env_var AS env_var, is_default AS is_default, project_id AS project_id, created_at AS created_at, updated_at AS updated_at FROM environments;

-- name: EnvironmentsGetById :one
SELECT id AS id, name AS name, description AS description, env_var AS env_var, is_default AS is_default, project_id AS project_id, created_at AS created_at, updated_at AS updated_at FROM environments WHERE id = ?;

-- name: EnvironmentsCreate :exec
INSERT INTO environments (name, description, env_var, is_default, project_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?);

-- name: EnvironmentsUpdate :exec
UPDATE environments SET name = ?, description = ?, env_var = ?, is_default = ?, project_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: EnvironmentsDelete :exec
DELETE FROM environments WHERE id = ?;

-- name: EnvironmentsGetInTransaction :one
SELECT id AS id, name, description, env_var, is_default, project_id, created_at, updated_at
               FROM environments WHERE id = ?;

-- name: EnvironmentsListByProject :many
SELECT id AS id, name, description, env_var, is_default, project_id, created_at, updated_at
               FROM environments WHERE project_id = ?
               ORDER BY is_default DESC, created_at ASC, id ASC;

-- name: EnvironmentsCountByProject :one
SELECT COUNT(*) FROM environments WHERE project_id = ?;

-- name: EnvironmentsClearDefault :exec
UPDATE environments SET is_default = 0 WHERE project_id = ? AND is_default = 1;

-- name: EnvironmentsCreateInTransaction :one
INSERT INTO environments (name, description, env_var, is_default, project_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS id, name, description, env_var, is_default, project_id, created_at, updated_at;

-- name: EnvironmentsUpdateInTransaction :one
UPDATE environments SET name = ?, description = ?, env_var = ?, is_default = ?
               WHERE id = ?
               RETURNING id AS id, name, description, env_var, is_default, project_id, created_at, updated_at;

-- name: EnvironmentsDeleteInTransaction :exec
DELETE FROM environments WHERE id = ?;

-- name: EnvironmentsPromoteOldestToDefault :exec
UPDATE environments SET is_default = 1 WHERE id = (SELECT id FROM environments WHERE project_id = ? ORDER BY created_at ASC, id ASC LIMIT 1);
