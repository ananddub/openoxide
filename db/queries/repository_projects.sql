-- Extracted from src/db/repository/projects.rs. Do not edit repository SQL directly.

-- name: ProjectsGetAll :many
SELECT id AS id, name AS name, description AS description, env_var AS env_var, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM projects;

-- name: ProjectsGetById :one
SELECT id AS id, name AS name, description AS description, env_var AS env_var, organization_id AS organization_id, created_at AS created_at, updated_at AS updated_at FROM projects WHERE id = ?;

-- name: ProjectsCreate :exec
INSERT INTO projects (name, description, env_var, organization_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?);

-- name: ProjectsUpdate :exec
UPDATE projects SET name = ?, description = ?, env_var = ?, organization_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: ProjectsDelete :exec
DELETE FROM projects WHERE id = ?;

-- name: ProjectsListByOrganization :many
SELECT id AS id, name, description, env_var, organization_id, created_at, updated_at
               FROM projects WHERE organization_id = ? ORDER BY created_at DESC, id DESC;

-- name: ProjectsCreateInTransaction :one
INSERT INTO projects (name, description, env_var, organization_id) VALUES (?, ?, ?, ?)
               RETURNING id AS id, name, description, env_var, organization_id, created_at, updated_at;

-- name: ProjectsUpdateAndReturn :one
UPDATE projects SET name = ?, description = ?, env_var = ? WHERE id = ? RETURNING id AS id, name, description, env_var, organization_id, created_at, updated_at;
