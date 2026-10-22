-- FlashORM queries for tags.

-- name: TagsGetAll :many
SELECT id AS id, name AS name, color AS color, organization_id AS organization_id, created_at AS created_at FROM tags;

-- name: TagsGetById :one
SELECT id AS id, name AS name, color AS color, organization_id AS organization_id, created_at AS created_at FROM tags WHERE id = ?;

-- name: TagsCreate :exec
INSERT INTO tags (name, color, organization_id, created_at) VALUES (?, ?, ?, ?);

-- name: TagsGetLastInsertRowid :one
SELECT CAST(last_insert_rowid() AS BIGINT) AS id;

-- name: TagsUpdate :exec
UPDATE tags SET name = ?, color = ?, organization_id = ?, created_at = ? WHERE id = ?;

-- name: TagsDelete :exec
DELETE FROM tags WHERE id = ?;

-- project_tags link table (i64 binds). The unique (project_id, tag_id)
-- index makes re-attaching an existing tag a no-op instead of a duplicate.
-- name: TagsAddToProject :exec
INSERT INTO project_tags (project_id, tag_id) VALUES (?, ?) ON CONFLICT (project_id, tag_id) DO NOTHING;

-- name: TagsRemoveFromProject :exec
DELETE FROM project_tags WHERE project_id = ? AND tag_id = ?;
