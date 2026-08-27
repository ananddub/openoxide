-- Extracted from src/db/repository/project_tags.rs. Do not edit repository SQL directly.

-- name: ProjectTagsGetAll :many
SELECT id AS id, project_id AS project_id, tag_id AS tag_id FROM project_tags;

-- name: ProjectTagsGetById :one
SELECT id AS id, project_id AS project_id, tag_id AS tag_id FROM project_tags WHERE id = ?;

-- name: ProjectTagsCreate :exec
INSERT INTO project_tags (project_id, tag_id) VALUES (?, ?);

-- name: ProjectTagsUpdate :exec
UPDATE project_tags SET project_id = ?, tag_id = ? WHERE id = ?;

-- name: ProjectTagsDelete :exec
DELETE FROM project_tags WHERE id = ?;
