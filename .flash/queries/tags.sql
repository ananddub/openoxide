-- FlashORM queries for tags.

-- name: TagsGetAll :many
SELECT id AS id, name AS name, color AS color, organization_id AS organization_id, created_at AS created_at FROM tags;

-- name: TagsGetById :one
SELECT id AS id, name AS name, color AS color, organization_id AS organization_id, created_at AS created_at FROM tags WHERE id = ?;

-- name: TagsCreate :exec
INSERT INTO tags (name, color, organization_id, created_at) VALUES (?, ?, ?, ?);

-- name: TagsGetLastInsertRowid :one
SELECT CAST(last_insert_rowid() AS INTEGER) AS id;

-- name: TagsUpdate :exec
UPDATE tags SET name = ?, color = ?, organization_id = ?, created_at = ? WHERE id = ?;

-- name: TagsDelete :exec
DELETE FROM tags WHERE id = ?;
