-- ============================================================
-- PROJECTS
-- ============================================================

-- name: GetProjectByID :one
SELECT * FROM projects WHERE id = ?;

-- name: GetProjectByName :one
SELECT * FROM projects WHERE name = ?;

-- name: ListProjectsByOrg :many
SELECT * FROM projects WHERE organization_id = ? ORDER BY created_at DESC;

-- name: CreateProject :one
INSERT INTO projects (name, description, env_var, organization_id)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: UpdateProject :one
UPDATE projects
SET name = ?, description = ?, env_var = ?
WHERE id = ?
RETURNING *;

-- name: DeleteProject :exec
DELETE FROM projects WHERE id = ?;

-- ============================================================
-- ENVIRONMENTS
-- ============================================================

-- name: GetEnvironmentByID :one
SELECT * FROM environments WHERE id = ?;

-- name: ListEnvironmentsByProject :many
SELECT * FROM environments WHERE project_id = ? ORDER BY is_default DESC, created_at ASC;

-- name: GetDefaultEnvironment :one
SELECT * FROM environments WHERE project_id = ? AND is_default = 1;

-- name: CreateEnvironment :one
INSERT INTO environments (name, description, env_var, is_default, project_id)
VALUES (?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateEnvironment :one
UPDATE environments
SET name = ?, description = ?, env_var = ?
WHERE id = ?
RETURNING *;

-- name: SetDefaultEnvironment :exec
UPDATE environments SET is_default = (CASE WHEN id = ? THEN 1 ELSE 0 END)
WHERE project_id = ?;

-- name: DeleteEnvironment :exec
DELETE FROM environments WHERE id = ?;

-- ============================================================
-- TAGS
-- ============================================================

-- name: GetTagByID :one
SELECT * FROM tags WHERE id = ?;

-- name: ListTagsByOrg :many
SELECT * FROM tags WHERE organization_id = ? ORDER BY name ASC;

-- name: CreateTag :one
INSERT INTO tags (name, color, organization_id)
VALUES (?, ?, ?)
RETURNING *;

-- name: DeleteTag :exec
DELETE FROM tags WHERE id = ?;

-- ============================================================
-- PROJECT TAGS
-- ============================================================

-- name: ListTagsByProject :many
SELECT t.* FROM tags t
JOIN project_tags pt ON pt.tag_id = t.id
WHERE pt.project_id = ?;

-- name: AddTagToProject :exec
INSERT OR IGNORE INTO project_tags (project_id, tag_id) VALUES (?, ?);

-- name: RemoveTagFromProject :exec
DELETE FROM project_tags WHERE project_id = ? AND tag_id = ?;


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
